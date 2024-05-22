pub mod config;

use sqlx::{Acquire, PgPool, Postgres, Transaction as DbTransaction};
use std::sync::{Arc, Mutex};
pub use tracing::instrument;

pub use config::*;

use crate::{
    account::Accounts,
    balance::Balances,
    entry::Entries,
    errors::*,
    journal::Journals,
    outbox::{server, EventSequence, Outbox, OutboxListener},
    primitives::TransactionId,
    transaction::{Transaction, Transactions},
    tx_template::{TxParams, TxTemplates},
};
#[cfg(feature = "import")]
mod import_deps {
    pub use crate::primitives::DataSourceId;
    pub use cala_types::outbox::OutboxEvent;
}
#[cfg(feature = "import")]
use import_deps::*;

#[derive(Clone)]
pub struct CalaLedger {
    pool: PgPool,
    accounts: Accounts,
    journals: Journals,
    transactions: Transactions,
    tx_templates: TxTemplates,
    entries: Entries,
    balances: Balances,
    outbox: Outbox,
    #[allow(clippy::type_complexity)]
    outbox_handle: Arc<Mutex<Option<tokio::task::JoinHandle<Result<(), tonic::transport::Error>>>>>,
}

impl CalaLedger {
    pub async fn init(
        config: CalaLedgerConfig,
    ) -> Result<Self, OneOf<(ConfigError, DbMigrationError, UnexpectedDbError)>> {
        let pool = match (config.pool, config.pg_con) {
            (Some(pool), None) => pool,
            (None, Some(pg_con)) => {
                let mut pool_opts = sqlx::postgres::PgPoolOptions::new();
                if let Some(max_connections) = config.max_connections {
                    pool_opts = pool_opts.max_connections(max_connections);
                }
                pool_opts
                    .connect(&pg_con)
                    .await
                    .map_err(|e| OneOf::new(UnexpectedDbError(e)))?
            }
            _ => {
                return Err(OneOf::new(ConfigError(
                    "One of pg_con or pool must be set".to_string(),
                )));
            }
        };
        if config.exec_migrations {
            sqlx::migrate!()
                .run(&pool)
                .await
                .map_err(|e| OneOf::new(DbMigrationError(e)))?;
        }

        let outbox = Outbox::init(&pool).await.map_err(OneOf::broaden)?;
        let mut outbox_handle = None;
        if let Some(outbox_config) = config.outbox {
            outbox_handle = Some(Self::start_outbox_server(outbox_config, outbox.clone()));
        }

        let accounts = Accounts::new(&pool, outbox.clone());
        let journals = Journals::new(&pool, outbox.clone());
        let tx_templates = TxTemplates::new(&pool, outbox.clone());
        let transactions = Transactions::new(&pool, outbox.clone());
        let entries = Entries::new(&pool, outbox.clone());
        let balances = Balances::new(&pool, outbox.clone());
        Ok(Self {
            accounts,
            journals,
            tx_templates,
            outbox,
            transactions,
            entries,
            balances,
            outbox_handle: Arc::new(Mutex::new(outbox_handle)),
            pool,
        })
    }

    pub fn accounts(&self) -> &Accounts {
        &self.accounts
    }

    pub fn journals(&self) -> &Journals {
        &self.journals
    }

    pub fn tx_templates(&self) -> &TxTemplates {
        &self.tx_templates
    }

    pub fn balances(&self) -> &Balances {
        &self.balances
    }

    pub fn transactions(&self) -> &Transactions {
        &self.transactions
    }

    pub async fn post_transaction(
        &self,
        tx_id: TransactionId,
        tx_template_code: &str,
        params: Option<impl Into<TxParams> + std::fmt::Debug>,
    ) -> Result<
        Transaction,
        OneOf<(
            OptimisticLockingError,
            UnbalancedTransaction,
            TxParamTypeMismatch,
            TooManyParams,
            CelEvaluationError,
            EntityNotFound,
            UnexpectedDbError,
        )>,
    > {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| OneOf::new(UnexpectedDbError(e)))?;
        self.post_transaction_in_tx(tx, tx_id, tx_template_code, params)
            .await
    }

    #[instrument(name = "cala_ledger.post_transaction", skip(self, tx))]
    pub async fn post_transaction_in_tx(
        &self,
        mut tx: DbTransaction<'_, Postgres>,
        tx_id: TransactionId,
        tx_template_code: &str,
        params: Option<impl Into<TxParams> + std::fmt::Debug>,
    ) -> Result<
        Transaction,
        OneOf<(
            OptimisticLockingError,
            UnbalancedTransaction,
            TxParamTypeMismatch,
            TooManyParams,
            CelEvaluationError,
            EntityNotFound,
            UnexpectedDbError,
        )>,
    > {
        let prepared_tx = self
            .tx_templates
            .prepare_transaction(
                tx_id,
                tx_template_code,
                params.map(|p| p.into()).unwrap_or_default(),
            )
            .await
            .map_err(OneOf::broaden)?;
        let (transaction, tx_event) = self
            .transactions
            .create_in_tx(&mut tx, prepared_tx.transaction)
            .await
            .map_err(OneOf::broaden)?;
        let (entries, entry_events) = self
            .entries
            .create_all(&mut tx, prepared_tx.entries)
            .await
            .map_err(OneOf::broaden)?;
        let balance_events = self
            .balances
            .update_balances(
                tx.begin()
                    .await
                    .map_err(|e| OneOf::new(UnexpectedDbError(e)))?,
                transaction.created_at(),
                transaction.journal_id(),
                entries,
            )
            .await
            .map_err(OneOf::broaden)?;
        self.outbox
            .persist_events(
                tx,
                std::iter::once(tx_event)
                    .chain(entry_events)
                    .chain(balance_events),
            )
            .await
            .map_err(OneOf::broaden)?;
        Ok(transaction)
    }

    pub async fn register_outbox_listener(
        &self,
        start_after: Option<EventSequence>,
    ) -> Result<OutboxListener, OneOf<(UnexpectedDbError,)>> {
        self.outbox.register_listener(start_after).await
    }

    #[cfg(feature = "import")]
    #[instrument(name = "cala_ledger.sync_outbox_event", skip(self, tx))]
    pub async fn sync_outbox_event(
        &self,
        tx: sqlx::Transaction<'_, sqlx::Postgres>,
        origin: DataSourceId,
        event: OutboxEvent,
    ) -> Result<(), UnexpectedDbError> {
        use crate::outbox::OutboxEventPayload::*;

        match event.payload {
            Empty => (),
            AccountCreated { account, .. } => self
                .accounts
                .sync_account_creation(tx, event.recorded_at, origin, account)
                .await
                .map_err(OneOf::take)?,
            JournalCreated { journal, .. } => self
                .journals
                .sync_journal_creation(tx, event.recorded_at, origin, journal)
                .await
                .map_err(OneOf::take)?,
            TransactionCreated { transaction, .. } => self
                .transactions
                .sync_transaction_creation(tx, event.recorded_at, origin, transaction)
                .await
                .map_err(OneOf::take)?,
            TxTemplateCreated { tx_template, .. } => self
                .tx_templates
                .sync_tx_template_creation(tx, event.recorded_at, origin, tx_template)
                .await
                .map_err(OneOf::take)?,
            EntryCreated { entry, .. } => self
                .entries
                .sync_entry_creation(tx, event.recorded_at, origin, entry)
                .await
                .map_err(OneOf::take)?,
            BalanceCreated { balance, .. } => self
                .balances
                .sync_balance_creation(tx, origin, balance)
                .await
                .map_err(OneOf::take)?,
            BalanceUpdated { balance, .. } => self
                .balances
                .sync_balance_update(tx, origin, balance)
                .await
                .map_err(OneOf::take)?,
        }
        Ok(())
    }

    pub async fn await_outbox_handle(&self) -> Result<(), tonic::transport::Error> {
        let handle = { self.outbox_handle.lock().expect("poisened mutex").take() };
        if let Some(handle) = handle {
            return handle.await.expect("Couldn't await outbox handle");
        }
        Ok(())
    }

    pub fn shutdown_outbox(&mut self) {
        if let Some(handle) = self.outbox_handle.lock().expect("poisened mutex").take() {
            handle.abort();
        }
    }

    fn start_outbox_server(
        config: server::OutboxServerConfig,
        outbox: Outbox,
    ) -> tokio::task::JoinHandle<Result<(), tonic::transport::Error>> {
        tokio::spawn(async move {
            server::start(config, outbox).await?;
            Ok(())
        })
    }
}

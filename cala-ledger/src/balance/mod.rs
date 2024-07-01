mod account_balance;
mod cursor;
pub mod error;
mod repo;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Acquire, PgPool, Postgres, Transaction};
use std::collections::{HashMap, HashSet};
use tracing::instrument;

pub use cala_types::balance::{BalanceAmount, BalanceSnapshot};
use cala_types::{entry::EntryValues, primitives::*};

use crate::{
    atomic_operation::*,
    outbox::*,
    primitives::{DataSource, JournalId},
    query::*,
};

pub use account_balance::*;
pub use cursor::*;
use error::BalanceError;
use repo::*;

const UNASSIGNED_ENTRY_ID: uuid::Uuid = uuid::Uuid::nil();

#[derive(Clone)]
pub struct Balances {
    repo: BalanceRepo,
    outbox: Outbox,
    _pool: PgPool,
}

impl Balances {
    pub(crate) fn new(pool: &PgPool, outbox: Outbox) -> Self {
        Self {
            repo: BalanceRepo::new(pool),
            outbox,
            _pool: pool.clone(),
        }
    }

    #[instrument(name = "cala_ledger.balance.find", skip(self), err)]
    pub async fn find(
        &self,
        journal_id: JournalId,
        account_id: impl Into<AccountId> + std::fmt::Debug,
        currency: Currency,
    ) -> Result<AccountBalance, BalanceError> {
        self.repo
            .find(journal_id, account_id.into(), currency)
            .await
    }

    #[instrument(name = "cala_ledger.balance.find_in_op", skip(self, op), err)]
    pub async fn find_in_op(
        &self,
        op: &mut AtomicOperation<'_>,
        journal_id: JournalId,
        account_id: impl Into<AccountId> + std::fmt::Debug,
        currency: Currency,
    ) -> Result<AccountBalance, BalanceError> {
        self.repo
            .find_in_tx(op.tx(), journal_id, account_id.into(), currency)
            .await
    }

    #[instrument(name = "cala_ledger.balance.find_all", skip(self), err)]
    pub async fn find_all(
        &self,
        ids: &[BalanceId],
    ) -> Result<HashMap<BalanceId, AccountBalance>, BalanceError> {
        self.repo.find_all(ids).await
    }

    pub async fn list_as_of(
        &self,
        id: BalanceId,
        as_of: chrono::NaiveDate,
        query: PaginatedQueryArgs<BalanceHistoryAsOfCursor>,
    ) -> Result<PaginatedQueryRet<AccountBalance, BalanceHistoryAsOfCursor>, BalanceError> {
        unimplemented!()
    }

    pub(crate) async fn update_balances_in_op(
        &self,
        op: &mut AtomicOperation<'_>,
        created_at: DateTime<Utc>,
        journal_id: JournalId,
        entries: Vec<EntryValues>,
        account_set_mappings: HashMap<AccountId, Vec<AccountSetId>>,
    ) -> Result<(), BalanceError> {
        let mut ids: HashSet<_> = entries
            .iter()
            .map(|entry| (entry.account_id, entry.currency))
            .collect();
        for entry in entries.iter() {
            if let Some(account_set_ids) = account_set_mappings.get(&entry.account_id) {
                ids.extend(
                    account_set_ids
                        .iter()
                        .map(|account_set_id| (AccountId::from(account_set_id), entry.currency)),
                );
            }
        }

        let mut db = op.tx().begin().await?;

        let current_balances = self.repo.find_for_update(&mut db, journal_id, ids).await?;
        let new_balances =
            Self::new_snapshots(created_at, current_balances, entries, account_set_mappings);
        self.repo
            .insert_new_snapshots(&mut db, journal_id, &new_balances)
            .await?;

        db.commit().await?;

        op.accumulate(new_balances.into_iter().map(|balance| {
            if balance.version == 1 {
                OutboxEventPayload::BalanceCreated {
                    source: DataSource::Local,
                    balance,
                }
            } else {
                OutboxEventPayload::BalanceUpdated {
                    source: DataSource::Local,
                    balance,
                }
            }
        }));
        Ok(())
    }

    pub(crate) async fn find_balances_for_update(
        &self,
        db: &mut Transaction<'_, Postgres>,
        journal_id: JournalId,
        account_id: AccountId,
    ) -> Result<HashMap<Currency, BalanceSnapshot>, BalanceError> {
        self.repo
            .load_all_for_update(db, journal_id, account_id)
            .await
    }

    fn new_snapshots(
        time: DateTime<Utc>,
        mut current_balances: HashMap<(AccountId, Currency), Option<BalanceSnapshot>>,
        entries: Vec<EntryValues>,
        mappings: HashMap<AccountId, Vec<AccountSetId>>,
    ) -> Vec<BalanceSnapshot> {
        let mut latest_balances: HashMap<(AccountId, &Currency), BalanceSnapshot> = HashMap::new();
        let mut new_balances = Vec::new();
        for entry in entries.iter() {
            let empty = Vec::new();
            for account_id in mappings
                .get(&entry.account_id)
                .unwrap_or(&empty)
                .iter()
                .map(AccountId::from)
                .chain(std::iter::once(entry.account_id))
            {
                let balance = match (
                    latest_balances.remove(&(account_id, &entry.currency)),
                    current_balances.remove(&(account_id, entry.currency)),
                ) {
                    (Some(latest), _) => {
                        new_balances.push(latest.clone());
                        latest
                    }
                    (_, Some(Some(balance))) => balance,
                    (_, Some(None)) => {
                        latest_balances.insert(
                            (account_id, &entry.currency),
                            Self::new_snapshot(time, account_id, entry),
                        );
                        continue;
                    }
                    _ => {
                        continue;
                    }
                };
                latest_balances.insert(
                    (account_id, &entry.currency),
                    Self::update_snapshot(time, balance, entry),
                );
            }
        }
        new_balances.extend(latest_balances.into_values());
        new_balances
    }

    fn new_snapshot(
        time: DateTime<Utc>,
        account_id: AccountId,
        entry: &EntryValues,
    ) -> BalanceSnapshot {
        let entry_id = EntryId::from(UNASSIGNED_ENTRY_ID);
        Self::update_snapshot(
            time,
            BalanceSnapshot {
                journal_id: entry.journal_id,
                account_id,
                entry_id,
                currency: entry.currency,
                settled: BalanceAmount {
                    dr_balance: Decimal::ZERO,
                    cr_balance: Decimal::ZERO,
                    entry_id,
                    modified_at: time,
                },
                pending: BalanceAmount {
                    dr_balance: Decimal::ZERO,
                    cr_balance: Decimal::ZERO,
                    entry_id,
                    modified_at: time,
                },
                encumbrance: BalanceAmount {
                    dr_balance: Decimal::ZERO,
                    cr_balance: Decimal::ZERO,
                    entry_id,
                    modified_at: time,
                },
                version: 0,
                modified_at: time,
                created_at: time,
            },
            entry,
        )
    }

    fn update_snapshot(
        time: DateTime<Utc>,
        mut snapshot: BalanceSnapshot,
        entry: &EntryValues,
    ) -> BalanceSnapshot {
        snapshot.version += 1;
        snapshot.modified_at = time;
        snapshot.entry_id = entry.id;
        match entry.layer {
            Layer::Settled => {
                snapshot.settled.entry_id = entry.id;
                snapshot.settled.modified_at = time;
                match entry.direction {
                    DebitOrCredit::Debit => {
                        snapshot.settled.dr_balance += entry.units;
                    }
                    DebitOrCredit::Credit => {
                        snapshot.settled.cr_balance += entry.units;
                    }
                }
            }
            Layer::Pending => {
                snapshot.pending.entry_id = entry.id;
                snapshot.pending.modified_at = time;
                match entry.direction {
                    DebitOrCredit::Debit => {
                        snapshot.pending.dr_balance += entry.units;
                    }
                    DebitOrCredit::Credit => {
                        snapshot.pending.cr_balance += entry.units;
                    }
                }
            }
            Layer::Encumbrance => {
                snapshot.encumbrance.entry_id = entry.id;
                snapshot.encumbrance.modified_at = time;
                match entry.direction {
                    DebitOrCredit::Debit => {
                        snapshot.encumbrance.dr_balance += entry.units;
                    }
                    DebitOrCredit::Credit => {
                        snapshot.encumbrance.cr_balance += entry.units;
                    }
                }
            }
        }
        snapshot
    }

    #[cfg(feature = "import")]
    pub async fn sync_balance_creation(
        &self,
        mut db: sqlx::Transaction<'_, sqlx::Postgres>,
        origin: DataSourceId,
        balance: BalanceSnapshot,
    ) -> Result<(), BalanceError> {
        self.repo.import_balance(&mut db, origin, &balance).await?;
        let recorded_at = balance.created_at;
        self.outbox
            .persist_events_at(
                db,
                std::iter::once(OutboxEventPayload::BalanceCreated {
                    source: DataSource::Remote { id: origin },
                    balance,
                }),
                recorded_at,
            )
            .await?;
        Ok(())
    }

    #[cfg(feature = "import")]
    pub async fn sync_balance_update(
        &self,
        mut db: sqlx::Transaction<'_, sqlx::Postgres>,
        origin: DataSourceId,
        balance: BalanceSnapshot,
    ) -> Result<(), BalanceError> {
        self.repo
            .import_balance_update(&mut db, origin, &balance)
            .await?;
        let recorded_at = balance.modified_at;
        self.outbox
            .persist_events_at(
                db,
                std::iter::once(OutboxEventPayload::BalanceUpdated {
                    source: DataSource::Remote { id: origin },
                    balance,
                }),
                recorded_at,
            )
            .await?;
        Ok(())
    }
}

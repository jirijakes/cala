#[cfg(feature = "import")]
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres};

use cala_types::primitives::*;

#[cfg(feature = "import")]
use crate::primitives::DataSourceId;
use crate::{entity::*, errors::*, primitives::TransactionId};

use super::entity::*;

#[derive(Debug, Clone)]
pub(super) struct TransactionRepo {
    _pool: PgPool,
}

impl TransactionRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            _pool: pool.clone(),
        }
    }

    pub async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, Postgres>,
        new_transaction: NewTransaction,
    ) -> Result<EntityUpdate<Transaction>, OneOf<(UnexpectedDbError,)>> {
        sqlx::query!(
            r#"INSERT INTO cala_transactions (id, journal_id, external_id)
            VALUES ($1, $2, $3)"#,
            new_transaction.id as TransactionId,
            new_transaction.journal_id as JournalId,
            new_transaction.external_id
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| OneOf::new(UnexpectedDbError(e)))?;
        let mut events = new_transaction.initial_events();
        let n_new_events = events.persist(tx).await?;
        let transaction = Transaction::try_from(events).expect("Couldn't hydrate new entity");
        Ok(EntityUpdate {
            entity: transaction,
            n_new_events,
        })
    }

    #[cfg(feature = "import")]
    pub async fn import(
        &self,
        tx: &mut sqlx::Transaction<'_, Postgres>,
        recorded_at: DateTime<Utc>,
        origin: DataSourceId,
        transaction: &mut Transaction,
    ) -> Result<(), OneOf<(UnexpectedDbError,)>> {
        sqlx::query!(
            r#"INSERT INTO cala_transactions (data_source_id, id, journal_id, external_id, created_at)
            VALUES ($1, $2, $3, $4, $5)"#,
            origin as DataSourceId,
            transaction.values().id as TransactionId,
            transaction.values().journal_id as JournalId,
            transaction.values().external_id,
            recorded_at
        )
        .execute(&mut **tx)
        .await.map_err(UnexpectedDbError)?;
        transaction
            .events
            .persisted_at(tx, origin, recorded_at)
            .await?;
        Ok(())
    }
}

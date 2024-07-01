use async_graphql::{types::connection::*, *};
use serde::{Deserialize, Serialize};

use super::{convert::ToGlobalId, primitives::*};
use crate::app::CalaApp;
use cala_ledger::primitives::{AccountId, Currency, JournalId};

#[derive(SimpleObject)]
pub(super) struct Money {
    pub units: Decimal,
    pub currency: CurrencyCode,
}

impl From<(rust_decimal::Decimal, Currency)> for Money {
    fn from((units, currency): (rust_decimal::Decimal, Currency)) -> Self {
        Self {
            units: units.into(),
            currency: currency.into(),
        }
    }
}

#[derive(SimpleObject)]
pub(super) struct BalanceAmount {
    pub dr_balance: Money,
    pub cr_balance: Money,
    pub normal_balance: Money,
    pub entry_id: UUID,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub(super) struct Balance {
    pub id: ID,
    pub journal_id: UUID,
    pub account_id: UUID,
    pub entry_id: UUID,
    pub currency: CurrencyCode,
    pub settled: BalanceAmount,
    pub pending: BalanceAmount,
    pub encumbrance: BalanceAmount,
    pub version: u32,
    #[graphql(skip)]
    pub(super) balance: cala_ledger::balance::AccountBalance,
}

#[ComplexObject]
impl Balance {
    async fn available(&self, layer: Layer) -> BalanceAmount {
        let amount = self.balance.details.available(layer.into());
        let currency = self.balance.details.currency;
        BalanceAmount {
            dr_balance: (amount.dr_balance, currency).into(),
            cr_balance: (amount.cr_balance, currency).into(),
            normal_balance: (self.balance.available(layer.into()), currency).into(),
            entry_id: amount.entry_id.into(),
        }
    }

    async fn history_as_of(
        &self,
        ctx: &Context<'_>,
        as_of: Date,
        first: i32,
        after: Option<String>,
    ) -> Result<Connection<BalanceHistoryAsOfCursor, Balance, EmptyFields, EmptyFields>> {
        let app = ctx.data_unchecked::<CalaApp>();
        query(
            after,
            None,
            Some(first),
            None,
            |after, _, first, _| async move {
                let first = first.expect("First always exists");
                let result = app
                    .ledger()
                    .balances()
                    .list_as_of(
                        (
                            JournalId::from(self.journal_id),
                            AccountId::from(self.account_id),
                            Currency::from(self.currency),
                        ),
                        chrono::NaiveDate::from(as_of),
                        cala_ledger::query::PaginatedQueryArgs {
                            first,
                            after: after.map(cala_ledger::balance::BalanceHistoryAsOfCursor::from),
                        },
                    )
                    .await?;
                let mut connection = Connection::new(false, result.has_next_page);
                // connection
                //     .edges
                //     .extend(result.entities.into_iter().map(|entity| {
                //         let cursor = AccountByNameCursor::from(entity.values());
                //         Edge::new(cursor, Account::from(entity))
                //     }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

impl ToGlobalId for (JournalId, AccountId, Currency) {
    fn to_global_id(&self) -> async_graphql::types::ID {
        async_graphql::types::ID::from(format!("balance:{}:{}:{}", self.0, self.1, self.2))
    }
}

impl From<cala_ledger::balance::AccountBalance> for Balance {
    fn from(balance: cala_ledger::balance::AccountBalance) -> Self {
        let currency = balance.details.currency;
        Self {
            id: (
                balance.details.journal_id,
                balance.details.account_id,
                balance.details.currency,
            )
                .to_global_id(),
            journal_id: balance.details.journal_id.into(),
            account_id: balance.details.account_id.into(),
            entry_id: balance.details.entry_id.into(),
            currency: balance.details.currency.into(),
            version: balance.details.version,
            settled: BalanceAmount {
                dr_balance: (balance.details.settled.dr_balance, currency).into(),
                cr_balance: (balance.details.settled.cr_balance, currency).into(),
                normal_balance: (balance.settled(), currency).into(),
                entry_id: balance.details.settled.entry_id.into(),
            },
            pending: BalanceAmount {
                dr_balance: (balance.details.pending.dr_balance, currency).into(),
                cr_balance: (balance.details.pending.cr_balance, currency).into(),
                normal_balance: (balance.pending(), currency).into(),
                entry_id: balance.details.pending.entry_id.into(),
            },
            encumbrance: BalanceAmount {
                dr_balance: (balance.details.encumbrance.dr_balance, currency).into(),
                cr_balance: (balance.details.encumbrance.cr_balance, currency).into(),
                normal_balance: (balance.encumbrance(), currency).into(),
                entry_id: balance.details.encumbrance.entry_id.into(),
            },
            balance,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct BalanceHistoryAsOfCursor {
    inner: cala_ledger::balance::BalanceHistoryAsOfCursor,
}

impl CursorType for BalanceHistoryAsOfCursor {
    type Error = String;

    fn encode_cursor(&self) -> String {
        use base64::{engine::general_purpose, Engine as _};
        let json = serde_json::to_string(&self).expect("could not serialize token");
        general_purpose::STANDARD_NO_PAD.encode(json.as_bytes())
    }

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        use base64::{engine::general_purpose, Engine as _};
        let bytes = general_purpose::STANDARD_NO_PAD
            .decode(s.as_bytes())
            .map_err(|e| e.to_string())?;
        let json = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl From<BalanceHistoryAsOfCursor> for cala_ledger::balance::BalanceHistoryAsOfCursor {
    fn from(cursor: BalanceHistoryAsOfCursor) -> Self {
        cursor.inner
    }
}

use async_graphql::{dataloader::*, types::connection::*, *};
use serde::{Deserialize, Serialize};

use cala_ledger::{
    balance::*,
    primitives::{AccountId, Currency, JournalId},
};

use crate::app::CalaApp;

use super::{
    account_set::*, balance::Balance, convert::ToGlobalId, loader::LedgerDataLoader, primitives::*,
};

#[derive(Clone, SimpleObject)]
#[graphql(complex)]
pub struct Account {
    id: ID,
    account_id: UUID,
    version: u32,
    code: String,
    name: String,
    normal_balance_type: DebitOrCredit,
    status: Status,
    external_id: Option<String>,
    description: Option<String>,
    metadata: Option<JSON>,
    created_at: Timestamp,
    modified_at: Timestamp,
}

#[ComplexObject]
impl Account {
    async fn balance(
        &self,
        ctx: &Context<'_>,
        journal_id: UUID,
        currency: CurrencyCode,
    ) -> async_graphql::Result<Option<Balance>> {
        let loader = ctx.data_unchecked::<DataLoader<LedgerDataLoader>>();
        let journal_id = JournalId::from(journal_id);
        let account_id = AccountId::from(self.account_id);
        let currency = Currency::from(currency);
        let balance: Option<AccountBalance> =
            loader.load_one((journal_id, account_id, currency)).await?;
        Ok(balance.map(Balance::from))
    }

    async fn sets(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> Result<Connection<AccountSetByNameCursor, AccountSet, EmptyFields, EmptyFields>> {
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
                    .account_sets()
                    .find_where_account_is_member(
                        self.account_id.into(),
                        cala_ledger::query::PaginatedQueryArgs {
                            first,
                            after: after
                                .map(cala_ledger::account_set::AccountSetByNameCursor::from),
                        },
                    )
                    .await?;
                let mut connection = Connection::new(false, result.has_next_page);
                connection
                    .edges
                    .extend(result.entities.into_iter().map(|entity| {
                        let cursor = AccountSetByNameCursor::from(entity.values());
                        Edge::new(cursor, AccountSet::from(entity))
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct AccountByNameCursor {
    pub name: String,
    pub id: cala_ledger::primitives::AccountId,
}

impl CursorType for AccountByNameCursor {
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

#[derive(InputObject)]
pub(super) struct AccountCreateInput {
    pub account_id: UUID,
    pub external_id: Option<String>,
    pub code: String,
    pub name: String,
    #[graphql(default)]
    pub normal_balance_type: DebitOrCredit,
    pub description: Option<String>,
    #[graphql(default)]
    pub status: Status,
    pub metadata: Option<JSON>,
    pub account_set_ids: Option<Vec<UUID>>,
}

#[derive(SimpleObject)]
pub(super) struct AccountCreatePayload {
    pub account: Account,
}

#[derive(InputObject)]
pub(super) struct AccountUpdateInput {
    pub external_id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub normal_balance_type: Option<DebitOrCredit>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub metadata: Option<JSON>,
    pub eventually_consistent: Option<bool>,
}

#[derive(SimpleObject)]
pub(super) struct AccountUpdatePayload {
    pub account: Account,
}

impl ToGlobalId for cala_ledger::AccountId {
    fn to_global_id(&self) -> async_graphql::types::ID {
        async_graphql::types::ID::from(format!("account:{}", self))
    }
}

impl From<&cala_ledger::account::AccountValues> for AccountByNameCursor {
    fn from(values: &cala_ledger::account::AccountValues) -> Self {
        Self {
            name: values.name.clone(),
            id: values.id,
        }
    }
}

impl From<AccountByNameCursor> for cala_ledger::account::AccountByNameCursor {
    fn from(cursor: AccountByNameCursor) -> Self {
        Self {
            name: cursor.name,
            id: cursor.id,
        }
    }
}

impl From<cala_ledger::account::Account> for Account {
    fn from(account: cala_ledger::account::Account) -> Self {
        let created_at = account.created_at();
        let modified_at = account.modified_at();
        let values = account.into_values();
        Self {
            id: values.id.to_global_id(),
            account_id: UUID::from(values.id),
            version: values.version,
            code: values.code,
            name: values.name,
            normal_balance_type: DebitOrCredit::from(values.normal_balance_type),
            status: Status::from(values.status),
            external_id: values.external_id,
            description: values.description,
            metadata: values.metadata.map(JSON::from),
            created_at: created_at.into(),
            modified_at: modified_at.into(),
        }
    }
}

impl From<cala_ledger::account::Account> for AccountCreatePayload {
    fn from(value: cala_ledger::account::Account) -> Self {
        Self {
            account: Account::from(value),
        }
    }
}

impl From<cala_ledger::account::Account> for AccountUpdatePayload {
    fn from(value: cala_ledger::account::Account) -> Self {
        Self {
            account: Account::from(value),
        }
    }
}

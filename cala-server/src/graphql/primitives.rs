#![allow(clippy::upper_case_acronyms)]
use async_graphql::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "cala_ledger::primitives::DebitOrCredit")]
pub(super) enum DebitOrCredit {
    Debit,
    Credit,
}

impl Default for DebitOrCredit {
    fn default() -> Self {
        Self::Credit
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "cala_ledger::primitives::Status")]
pub(super) enum Status {
    Active,
    Locked,
}

impl Default for Status {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct JSON(serde_json::Value);
scalar!(JSON);
impl From<serde_json::Value> for JSON {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl JSON {
    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(remote = "cala_ledger::tx_template::ParamDataType")]
pub enum ParamDataType {
    String,
    Integer,
    Decimal,
    Boolean,
    Uuid,
    Date,
    Timestamp,
    Json,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct UUID(uuid::Uuid);
scalar!(UUID);
impl<T: Into<uuid::Uuid>> From<T> for UUID {
    fn from(id: T) -> Self {
        let uuid = id.into();
        Self(uuid)
    }
}

impl From<UUID> for cala_ledger::AccountId {
    fn from(uuid: UUID) -> Self {
        cala_ledger::AccountId::from(uuid.0)
    }
}

impl From<UUID> for cala_ledger::JournalId {
    fn from(uuid: UUID) -> Self {
        cala_ledger::JournalId::from(uuid.0)
    }
}

impl From<UUID> for cala_ledger::TxTemplateId {
    fn from(uuid: UUID) -> Self {
        cala_ledger::TxTemplateId::from(uuid.0)
    }
}

impl From<UUID> for cala_ledger::TransactionId {
    fn from(uuid: UUID) -> Self {
        cala_ledger::TransactionId::from(uuid.0)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Expression(String);
scalar!(Expression);

impl From<cel_interpreter::CelExpression> for Expression {
    fn from(expr: cel_interpreter::CelExpression) -> Self {
        Self(expr.to_string())
    }
}

impl From<Expression> for String {
    fn from(expr: Expression) -> Self {
        expr.0
    }
}

pub struct Date(NaiveDate);

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Date(date)
    }
}

impl From<Date> for NaiveDate {
    fn from(wrapper: Date) -> Self {
        wrapper.0
    }
}

#[Scalar(name = "Date")]
impl ScalarType for Date {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        let date_str = match &value {
            async_graphql::Value::String(s) => s,
            _ => return Err(async_graphql::InputValueError::expected_type(value)),
        };

        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map(Date)
            .map_err(|_| {
                async_graphql::InputValueError::custom("Invalid date format, expected YYYY-MM-DD")
            })
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.format("%Y-%m-%d").to_string())
    }
}

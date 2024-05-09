use cala_cel_interpreter::CelExpression;
use cala_types::{account::*, journal::*, outbox::*, primitives::*, tx_template::*};

use crate::{client::proto, error::*};

impl TryFrom<proto::CalaLedgerEvent> for OutboxEvent {
    type Error = CalaLedgerOutboxClientError;

    fn try_from(event: proto::CalaLedgerEvent) -> Result<Self, Self::Error> {
        let payload = OutboxEventPayload::try_from(
            event
                .payload
                .ok_or(CalaLedgerOutboxClientError::MissingField)?,
        )?;
        Ok(OutboxEvent {
            id: event.id.parse()?,
            sequence: EventSequence::from(event.sequence),
            payload,
            recorded_at: event
                .recorded_at
                .ok_or(CalaLedgerOutboxClientError::MissingField)?
                .into(),
        })
    }
}

impl TryFrom<proto::cala_ledger_event::Payload> for OutboxEventPayload {
    type Error = CalaLedgerOutboxClientError;

    fn try_from(payload: proto::cala_ledger_event::Payload) -> Result<Self, Self::Error> {
        use cala_types::outbox::OutboxEventPayload::*;
        let res = match payload {
            proto::cala_ledger_event::Payload::AccountCreated(proto::AccountCreated {
                data_source_id,
                account,
            }) => AccountCreated {
                source: data_source_id.parse()?,
                account: AccountValues::try_from(
                    account.ok_or(CalaLedgerOutboxClientError::MissingField)?,
                )?,
            },
            proto::cala_ledger_event::Payload::JournalCreated(proto::JournalCreated {
                data_source_id,
                journal,
            }) => JournalCreated {
                source: data_source_id.parse()?,
                journal: JournalValues::try_from(
                    journal.ok_or(CalaLedgerOutboxClientError::MissingField)?,
                )?,
            },
            proto::cala_ledger_event::Payload::TxTemplateCreated(proto::TxTemplateCreated {
                tx_template,
            }) => TxTemplateCreated {
                tx_template: TxTemplateValues::try_from(
                    tx_template.ok_or(CalaLedgerOutboxClientError::MissingField)?,
                )?,
            },
            proto::cala_ledger_event::Payload::Empty(_) => Empty,
        };
        Ok(res)
    }
}

impl TryFrom<proto::Account> for AccountValues {
    type Error = CalaLedgerOutboxClientError;

    fn try_from(account: proto::Account) -> Result<Self, Self::Error> {
        let metadata = account.metadata.map(serde_json::to_value).transpose()?;
        let normal_balance_type =
            proto::DebitOrCredit::try_from(account.normal_balance_type).map(DebitOrCredit::from)?;
        let status = proto::Status::try_from(account.status).map(Status::from)?;
        let res = Self {
            id: account.id.parse()?,
            code: account.code,
            name: account.name,
            external_id: account.external_id,
            normal_balance_type,
            status,
            description: account.description,
            tags: account
                .tags
                .into_iter()
                .map(|tag| tag.parse())
                .collect::<Result<Vec<Tag>, _>>()?,
            metadata,
        };
        Ok(res)
    }
}

impl TryFrom<proto::Journal> for JournalValues {
    type Error = CalaLedgerOutboxClientError;

    fn try_from(journal: proto::Journal) -> Result<Self, Self::Error> {
        let status = proto::Status::try_from(journal.status).map(Status::from)?;
        let res = Self {
            id: journal.id.parse()?,
            name: journal.name,
            status,
            external_id: journal.external_id,
            description: journal.description,
        };
        Ok(res)
    }
}

impl From<proto::DebitOrCredit> for DebitOrCredit {
    fn from(dc: proto::DebitOrCredit) -> Self {
        match dc {
            proto::DebitOrCredit::Debit => DebitOrCredit::Debit,
            proto::DebitOrCredit::Credit => DebitOrCredit::Credit,
        }
    }
}

impl From<proto::Status> for Status {
    fn from(status: proto::Status) -> Self {
        match status {
            proto::Status::Active => Status::Active,
            proto::Status::Locked => Status::Locked,
        }
    }
}

impl TryFrom<proto::TxTemplate> for TxTemplateValues {
    type Error = CalaLedgerOutboxClientError;

    fn try_from(
        proto::TxTemplate {
            id,
            code,
            params,
            tx_input,
            entries,
            description,
            metadata,
        }: proto::TxTemplate,
    ) -> Result<Self, Self::Error> {
        let params = params
            .into_iter()
            .map(ParamDefinition::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let tx_input =
            TxInput::try_from(tx_input.ok_or(CalaLedgerOutboxClientError::MissingField)?)?;
        let entries = entries
            .into_iter()
            .map(EntryInput::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let res = Self {
            id: id.parse()?,
            code,
            params: Some(params),
            tx_input,
            entries,
            description,
            metadata: metadata.map(serde_json::to_value).transpose()?,
        };
        Ok(res)
    }
}

impl TryFrom<proto::ParamDefinition> for ParamDefinition {
    type Error = CalaLedgerOutboxClientError;
    fn try_from(
        proto::ParamDefinition {
            name,
            data_type,
            default,
            description,
        }: proto::ParamDefinition,
    ) -> Result<Self, Self::Error> {
        let res = Self {
            name,
            r#type: proto::ParamDataType::try_from(data_type).map(ParamDataType::from)?,
            default: default.map(CelExpression::try_from).transpose()?,
            description,
        };
        Ok(res)
    }
}

impl TryFrom<proto::TxInput> for TxInput {
    type Error = CalaLedgerOutboxClientError;
    fn try_from(
        proto::TxInput {
            effective,
            journal_id,
            correlation_id,
            external_id,
            description,
            metadata,
        }: proto::TxInput,
    ) -> Result<Self, Self::Error> {
        let res = Self {
            effective: CelExpression::try_from(effective)?,
            journal_id: CelExpression::try_from(journal_id)?,
            correlation_id: correlation_id.map(CelExpression::try_from).transpose()?,
            external_id: external_id.map(CelExpression::try_from).transpose()?,
            description: description.map(CelExpression::try_from).transpose()?,
            metadata: metadata.map(CelExpression::try_from).transpose()?,
        };
        Ok(res)
    }
}

impl TryFrom<proto::EntryInput> for EntryInput {
    type Error = CalaLedgerOutboxClientError;
    fn try_from(
        proto::EntryInput {
            entry_type,
            account_id,
            layer,
            direction,
            units,
            currency,
            description,
        }: proto::EntryInput,
    ) -> Result<Self, Self::Error> {
        let res = Self {
            entry_type: CelExpression::try_from(entry_type)?,
            account_id: CelExpression::try_from(account_id)?,
            layer: CelExpression::try_from(layer)?,
            direction: CelExpression::try_from(direction)?,
            units: CelExpression::try_from(units)?,
            currency: CelExpression::try_from(currency)?,
            description: description.map(CelExpression::try_from).transpose()?,
        };
        Ok(res)
    }
}

impl From<proto::ParamDataType> for ParamDataType {
    fn from(data_type: proto::ParamDataType) -> Self {
        match data_type {
            proto::ParamDataType::String => ParamDataType::STRING,
            proto::ParamDataType::Integer => ParamDataType::INTEGER,
            proto::ParamDataType::Decimal => ParamDataType::DECIMAL,
            proto::ParamDataType::Boolean => ParamDataType::BOOLEAN,
            proto::ParamDataType::Uuid => ParamDataType::UUID,
            proto::ParamDataType::Date => ParamDataType::DATE,
            proto::ParamDataType::Timestamp => ParamDataType::TIMESTAMP,
            proto::ParamDataType::Json => ParamDataType::JSON,
        }
    }
}

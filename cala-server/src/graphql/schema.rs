use async_graphql::{types::connection::*, *};
use cala_ledger::tx_template::NewParamDefinition;

use super::{account::*, import_job::*, journal::*, tx_template::*};
use crate::{app::CalaApp, extension::MutationExtensionMarker};

pub struct Query;

#[Object]
impl Query {
    async fn accounts(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> Result<Connection<AccountByNameCursor, Account, EmptyFields, EmptyFields>> {
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
                    .accounts()
                    .list(cala_ledger::query::PaginatedQueryArgs {
                        first,
                        after: after.map(cala_ledger::account::AccountByNameCursor::from),
                    })
                    .await?;
                let mut connection = Connection::new(false, result.has_next_page);
                connection
                    .edges
                    .extend(result.entities.into_iter().map(|entity| {
                        let cursor = AccountByNameCursor::from(entity.values());
                        Edge::new(cursor, Account::from(entity.into_values()))
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }

    async fn import_jobs(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> Result<Connection<ImportJobByNameCursor, ImportJob, EmptyFields, EmptyFields>> {
        let app = ctx.data_unchecked::<CalaApp>();
        query(
            after,
            None,
            Some(first),
            None,
            |after, _, first, _| async move {
                let first = first.expect("First always exists");
                let result = app
                    .list_import_jobs(cala_ledger::query::PaginatedQueryArgs {
                        first,
                        after: after.map(crate::import_job::ImportJobByNameCursor::from),
                    })
                    .await?;
                let mut connection = Connection::new(false, result.has_next_page);
                connection
                    .edges
                    .extend(result.entities.into_iter().map(|entity| {
                        let cursor = ImportJobByNameCursor::from(&entity);
                        Edge::new(cursor, ImportJob::from(entity))
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

#[derive(Default)]
pub struct CoreMutation<E: MutationExtensionMarker> {
    _phantom: std::marker::PhantomData<E>,
}

#[Object(name = "Mutation")]
impl<E: MutationExtensionMarker> CoreMutation<E> {
    #[graphql(flatten)]
    async fn extension(&self) -> E {
        E::default()
    }

    async fn journal_create(
        &self,
        ctx: &Context<'_>,
        input: JournalCreateInput,
    ) -> Result<JournalCreatePayload> {
        let app = ctx.data_unchecked::<CalaApp>();
        let id = if let Some(id) = input.id {
            id.into()
        } else {
            cala_ledger::JournalId::new()
        };
        let mut new = cala_ledger::journal::NewJournal::builder();
        new.id(id).name(input.name);
        if let Some(external_id) = input.external_id {
            new.external_id(external_id);
        }
        if let Some(description) = input.description {
            new.description(description);
        }
        let journal = app.ledger().journals().create(new.build()?).await?;
        Ok(journal.into_values().into())
    }

    async fn tx_template_create(
        &self,
        ctx: &Context<'_>,
        input: TxTemplateCreateInput,
    ) -> Result<TxTemplateCreatePayload> {
        let app = ctx.data_unchecked::<CalaApp>();
        let id = if let Some(id) = input.id {
            id.into()
        } else {
            cala_ledger::TxTemplateId::new()
        };
        let mut new_tx_input_builder = cala_ledger::tx_template::NewTxInput::builder();
        let TxTemplateTxInput {
            effective,
            journal_id,
            correlation_id,
            external_id,
            description,
            metadata,
        } = input.tx_input;
        new_tx_input_builder
            .effective(effective.as_ref().to_string())
            .journal_id(journal_id.as_ref().to_string());
        if let Some(correlation_id) = correlation_id {
            new_tx_input_builder.correlation_id(correlation_id.as_ref().to_string());
        };
        if let Some(external_id) = external_id {
            new_tx_input_builder.external_id(external_id.as_ref().to_string());
        };
        if let Some(description) = description {
            new_tx_input_builder.description(description.as_ref().to_string());
        };
        if let Some(metadata) = metadata {
            new_tx_input_builder.metadata(metadata.as_ref().to_string());
        }
        let new_tx_input = new_tx_input_builder.build()?;
        let mut new_params = Vec::new();
        for param in input.params {
            let mut param_builder = NewParamDefinition::builder();
            param_builder.name(param.name).r#type(param.r#type.into());
            if let Some(default) = param.default {
                param_builder.default_expr(default.as_ref().to_string());
            }
            if let Some(desc) = param.description {
                param_builder.description(desc);
            }
            let new_param = param_builder.build()?;
            new_params.push(new_param);
        }
        let mut new_entries = Vec::new();
        for entry in input.entries {
            let TxTemplateEntryInput {
                entry_type,
                account_id,
                layer,
                direction,
                units,
                currency,
                description,
            } = entry;
            let mut new_entry_input_builder = cala_ledger::tx_template::NewEntryInput::builder();
            new_entry_input_builder
                .entry_type(entry_type.as_ref().to_string())
                .account_id(account_id.as_ref().to_string())
                .layer(layer.as_ref().to_string())
                .direction(direction.as_ref().to_string())
                .units(units.as_ref().to_string())
                .currency(currency.as_ref().to_string());
            if let Some(desc) = description {
                new_entry_input_builder.description(desc.as_ref().to_string());
            }

            let new_entry_input = new_entry_input_builder.build()?;
            new_entries.push(new_entry_input);
        }
        let mut new_tx_template_builder = cala_ledger::tx_template::NewTxTemplate::builder();
        new_tx_template_builder
            .id(id)
            .code(input.code)
            .tx_input(new_tx_input)
            .params(new_params)
            .entries(new_entries);
        if let Some(desc) = input.description {
            new_tx_template_builder.description(desc);
        }
        if let Some(metadata) = input.metadata {
            new_tx_template_builder.metadata(metadata)?;
        }

        let new_tx_template = new_tx_template_builder.build()?;
        let tx_template = app.ledger().tx_templates().create(new_tx_template).await?;
        let tx_template_payload = TxTemplateCreatePayload {
            tx_template: tx_template.into_values().into(),
        };
        Ok(tx_template_payload)
    }

    async fn import_job_create(
        &self,
        ctx: &Context<'_>,
        input: ImportJobCreateInput,
    ) -> Result<ImportJobCreatePayload> {
        let app = ctx.data_unchecked::<CalaApp>();
        Ok(ImportJobCreatePayload {
            import_job: app
                .create_import_job(input.name, input.description, input.endpoint)
                .await
                .map(ImportJob::from)?,
        })
    }
}

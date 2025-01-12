use crate::{schema, LoaderError};
use async_graphql::dataloader::Loader;
use async_graphql::ID;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SourceDocument {
    source: SourceDocumentModel,
}

/*
#[derive(Debug, Clone)]
pub enum SourceDocument {
    Document { source: SourceDocumentModel },
}
 */

#[async_graphql::Object]
impl SourceDocument {
    pub async fn version_id(&self, ctx: &async_graphql::Context<'_>) -> ID {
        ID(self.source.version_id.clone())
        /*
        match self {
            SourceDocument::Document { source } => ID(source.version_id.clone()),
        }
         */
    }
}

pub struct SourceDocumentLoader {
    pub pool: Pool<AsyncPgConnection>,
}

impl Loader<String> for SourceDocumentLoader {
    type Value = SourceDocument;
    type Error = LoaderError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        // Uses diesel to read from a postgres db
        let mut connection = self.pool.get().await?;
        let rows: Vec<SourceDocumentModel> = schema::source_documents::dsl::source_documents
            .select(SourceDocumentModel::as_select())
            .filter(schema::source_documents::dsl::version_id.eq_any(keys))
            .load(&mut connection)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.version_id.clone(),
                    //SourceDocument::Document { source: row },
                    SourceDocument{ source: row },
                )
            })
            .collect())
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::source_documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(version_id))]
pub struct SourceDocumentModel {
    pub version_id: String,
}

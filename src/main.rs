mod node;
mod schema;
mod source_document;

use crate::node::GetNodeQuery;
use crate::source_document::SourceDocumentLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use diesel::Queryable;
use diesel_async::pooled_connection::deadpool::{Pool, PoolError};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

#[derive(MergedObject, Default)]
struct Query(GetNodeQuery);

type ServerSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new("database-url");
    let pool = Pool::builder(config).max_size(1).build().unwrap();
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(DataLoader::new(SourceDocumentLoader { pool }, tokio::spawn))
        .finish();
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum LoaderError {
    #[error("Error loading data: {message}")]
    Error { message: String },
    #[error("Connection pool error: {message}")]
    PoolError { message: String },
    #[error("Diesel error: {message}")]
    DieselError { message: String },
}

impl From<PoolError> for LoaderError {
    fn from(value: PoolError) -> Self {
        Self::PoolError {
            message: value.to_string(),
        }
    }
}

impl From<diesel::result::Error> for LoaderError {
    fn from(value: diesel::result::Error) -> Self {
        Self::DieselError {
            message: value.to_string(),
        }
    }
}

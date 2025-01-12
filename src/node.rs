use async_graphql::{Context, Interface, Object, ID};
use async_graphql::dataloader::DataLoader;
use crate::source_document::{SourceDocument, SourceDocumentLoader};

#[derive(Interface)]
#[graphql(field(name = "version_id", ty = "ID"))]
enum Node {
    Other(Other),
    Doc(SourceDocument),
}

#[derive(Default)]
pub struct GetNodeQuery {}

#[Object]
impl GetNodeQuery {
    async fn node(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Node> {
        let loader = ctx.data::<DataLoader<SourceDocumentLoader>>()?;
        let node = loader.load_one(id.0).await?;
        Ok(Node::Doc(node.unwrap()))
    }
}

pub struct Other {

}

#[async_graphql::Object]
impl Other {
    pub async fn version_id(&self, ctx: &async_graphql::Context<'_>) -> ID {
        ID("other".to_string())
    }
}
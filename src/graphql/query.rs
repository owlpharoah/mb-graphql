use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn artist(&self) -> Option<String> {
        todo!()
    }
}

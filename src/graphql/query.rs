use async_graphql::{MergedObject, Object};

#[derive(Default)]
pub struct ArtistQuery;
#[Object]
impl ArtistQuery {
    async fn artist(&self) -> Option<String> {
        todo!()
    }
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub ArtistQuery);

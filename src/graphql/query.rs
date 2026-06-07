use crate::graphql::types::artist::ArtistQuery;
use async_graphql::{MergedObject, Object};

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub ArtistQuery);

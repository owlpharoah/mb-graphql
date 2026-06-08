use async_graphql::MergedObject;

use crate::graphql::types;
use types::artist::ArtistQuery;
use types::release_group::ReleaseGroupQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub ArtistQuery, pub ReleaseGroupQuery);

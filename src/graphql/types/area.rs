use async_graphql::{ComplexObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: i32,
}

use async_graphql::{ComplexObject, Context, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::{loaders::entity::label::LabelLoader, types::label::Label};

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct PartialDate {
    pub year: Option<i16>,
    pub month: Option<i16>,
    pub day: Option<i16>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    #[graphql(name = "sortName")]
    pub sort_name: Option<String>,
    #[graphql(name = "type")]
    pub alias_type: Option<String>,
    pub locale: Option<String>,
    pub primary: Option<bool>,
    #[graphql(name = "beginDate")]
    pub begin_date: Option<PartialDate>,
    #[graphql(name = "endDate")]
    pub end_date: Option<PartialDate>,
    pub ended: bool,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: i32,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub mbid: Uuid,
    pub name: String,
    pub disambiguation: Option<String>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub value: i32,
    #[graphql(name = "votesCount")]
    pub votes_count: i32,
}

//todo----
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    #[graphql(name = "joinPhrase")]
    pub join_phrase: String,
    // artist: Artist
}

//todo-----
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct LabelInfo {
    #[graphql(name = "catalogNumber")]
    pub catalog_number: Option<String>,
    #[graphql(skip)]
    pub label_id: Option<i32>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct ReleaseEvent {
    pub date: Option<PartialDate>,
    pub country: Option<String>,
}

//todo-----
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Track {
    pub mbid: Uuid,
    pub title: String,
    pub number: String,
    pub position: i32,
    pub length: Option<i32>,
    // recording: Recording
    #[graphql(skip)]
    pub id: i32,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Medium {
    pub format: Option<String>,
    pub position: i32,
    pub title: Option<String>,
    #[graphql(name = "trackCount")]
    pub track_count: i32,
    pub tracks: Vec<Track>,
}

impl PartialDate {
    pub fn from_parts(year: Option<i16>, month: Option<i16>, day: Option<i16>) -> Option<Self> {
        if year.is_none() && month.is_none() && day.is_none() {
            None
        } else {
            Some(Self { year, month, day })
        }
    }
}

#[ComplexObject]
impl LabelInfo {
    async fn label(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Label>> {
        let Some(label_id) = self.label_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<LabelLoader>>()?;

        Ok(loader.load_one(label_id).await?)
    }
}

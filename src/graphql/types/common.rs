use async_graphql::{ComplexObject, Context, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::{
    loaders::{
        entity::{
            artist::ArtistLoader, artist_credit::ArtistCreditLoader, label::LabelLoader,
            recording::RecordingLoader, tracks::TrackLoader,
        },
        relationship::track_id_by_medium::TrackIdByMediumLoader,
    },
    types::{artist::Artist, label::Label, recording::Recording},
};

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

#[derive(Clone)]
pub struct TagRef {
    pub tag_id: i32,
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
    pub value: i16,
    #[graphql(name = "votesCount")]
    pub votes_count: Option<i32>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct ArtistCredit {
    pub name: String,
    #[graphql(name = "joinPhrase")]
    pub join_phrase: String,
    #[graphql(skip)]
    pub artist_id: i32,
}

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
    pub country: Option<i32>,
}

//--todo
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Track {
    pub mbid: Uuid,
    pub name: String,
    pub number: String,
    pub position: i32,
    pub length: Option<i32>,
    #[graphql(skip)]
    pub artist_credit: i32,
    #[graphql(skip)]
    pub recording_id: i32,
    #[graphql(skip)]
    pub medium: i32,
    #[graphql(skip)]
    pub id: i32,
}

//todo--
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Medium {
    pub mbid: Uuid,
    pub format: Option<i32>,
    pub position: i32,
    pub name: String,
    #[graphql(name = "trackCount")]
    pub track_count: i32,
    #[graphql(skip)]
    pub id: i32,
    #[graphql(skip)]
    pub release: i32,
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

        loader.load_one(label_id).await
    }
}

#[ComplexObject]
impl Medium {
    async fn tracks(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Track>> {
        let track_ids_loader = ctx.data::<DataLoader<TrackIdByMediumLoader>>()?;
        let track_ids = track_ids_loader
            .load_one(self.id)
            .await?
            .unwrap_or_default();

        if track_ids.is_empty() {
            return Ok(vec![]);
        }

        let tracks_loader = ctx.data::<DataLoader<TrackLoader>>()?;
        let tracks_map = tracks_loader.load_many(track_ids.clone()).await?;

        Ok(track_ids
            .into_iter()
            .filter_map(|id| tracks_map.get(&id).cloned())
            .collect())
    }
}

#[ComplexObject]
impl Track {
    async fn recording(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Recording>> {
        let recording_loader = ctx.data::<DataLoader<RecordingLoader>>()?;
        recording_loader.load_one(self.recording_id).await
    }
    async fn artist_credit(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<Vec<ArtistCredit>>> {
        let artist_credit_loader = ctx.data::<DataLoader<ArtistCreditLoader>>()?;
        artist_credit_loader.load_one(self.artist_credit).await
    }
}

#[ComplexObject]
impl ArtistCredit {
    async fn artist(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Artist>> {
        let artist_loader = ctx.data::<DataLoader<ArtistLoader>>()?;
        artist_loader.load_one(self.artist_id).await
    }
}

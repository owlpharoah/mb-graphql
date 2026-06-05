use async_graphql::{ComplexObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::types;
use types::common::PartialDate;

struct ArtistRow {
    id: i32,
    gid: Uuid,
    name: String,
    sort_name: String,
    disambiguation: Option<String>,
    artist_type: Option<String>,
    gender: Option<String>,
    country: Option<String>,
    ended: bool,
    begin_date_year: Option<i16>,
    begin_date_month: Option<i16>,
    begin_date_day: Option<i16>,
    end_date_year: Option<i16>,
    end_date_month: Option<i16>,
    end_date_day: Option<i16>,

    area_id: Option<i32>,
    begin_area_id: Option<i32>,
    end_area_id: Option<i32>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Artist {
    pub mbid: Uuid,
    pub name: String,
    #[graphql(name = "sortName")]
    pub sort_name: String,
    pub disambiguation: Option<String>,
    #[graphql(name = "type")]
    pub artist_type: Option<String>,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub ended: bool,
    #[graphql(name = "beginDate")]
    pub begin_date: Option<PartialDate>,
    #[graphql(name = "endDate")]
    pub end_date: Option<PartialDate>,

    #[graphql(skip)]
    pub(crate) id: i32,
    #[graphql(skip)]
    pub(crate) area_id: Option<i32>,
    #[graphql(skip)]
    pub(crate) begin_area_id: Option<i32>,
    #[graphql(skip)]
    pub(crate) end_area_id: Option<i32>,
}

impl From<ArtistRow> for Artist {
    fn from(r: ArtistRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            sort_name: r.sort_name,
            disambiguation: r.disambiguation,
            artist_type: r.artist_type,
            gender: r.gender,
            country: r.country,
            ended: r.ended,
            begin_date: PartialDate::from_parts(
                r.begin_date_year,
                r.begin_date_month,
                r.begin_date_day,
            ),
            end_date: PartialDate::from_parts(r.end_date_year, r.end_date_month, r.end_date_day),
            id: r.id,
            area_id: r.area_id,
            begin_area_id: r.begin_area_id,
            end_area_id: r.end_area_id,
        }
    }
}

#[ComplexObject]
impl Artist {
    async fn game(&self) -> String {
        todo!()
    }
}

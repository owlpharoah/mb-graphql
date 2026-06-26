use async_graphql::{EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader};

use crate::graphql::loaders::alias_area::AreaAliasLoader;
use crate::graphql::loaders::alias_artist::ArtistAliasLoader;
use crate::graphql::loaders::alias_label::LabelAliasLoader;
use crate::graphql::loaders::alias_recording::RecordingAliasLoader;
use crate::graphql::loaders::alias_release::ReleaseAliasLoader;
use crate::graphql::loaders::alias_release_group::ReleaseGroupAliasLoader;
use crate::graphql::loaders::annotations_area::AreaAnnotationLoader;
use crate::graphql::loaders::annotations_artist::ArtistAnnotationLoader;
use crate::graphql::loaders::annotations_label::LabelAnnotationLoader;
use crate::graphql::loaders::annotations_recording::RecordingAnnotationLoader;
use crate::graphql::loaders::annotations_release::ReleaseAnnotationLoader;
use crate::graphql::loaders::annotations_release_group::ReleaseGroupAnnotationLoader;
use crate::graphql::loaders::artist_ipi::ArtistIpiLoader;
use crate::graphql::loaders::artist_isni::ArtistIsniLoader;
use crate::graphql::loaders::entity::area::AreaLoader;
use crate::graphql::loaders::entity::artist::ArtistLoader;
use crate::graphql::loaders::entity::artist_credit::ArtistCreditLoader;
use crate::graphql::loaders::entity::genre::GenreLoader;
use crate::graphql::loaders::entity::label::LabelLoader;
use crate::graphql::loaders::entity::medium::MediumLoader;
use crate::graphql::loaders::entity::recording::RecordingLoader;
use crate::graphql::loaders::entity::release::ReleaseLoader;
use crate::graphql::loaders::entity::release_group::ReleaseGroupLoader;
use crate::graphql::loaders::entity::tag::TagLoader;
use crate::graphql::loaders::entity::tracks::TrackLoader;
use crate::graphql::loaders::iso_code_1_by_area::IsoCode1ByAreaLoader;
use crate::graphql::loaders::iso_code_2_by_area::IsoCode2ByAreaLoader;
use crate::graphql::loaders::iso_code_3_by_area::IsoCode3ByAreaLoader;
use crate::graphql::loaders::label_infos_by_release::LabelInfosByReleaseLoader;
use crate::graphql::loaders::label_ipi::LabelIpiLoader;
use crate::graphql::loaders::label_isni::LabelIsniLoader;
use crate::graphql::loaders::rating_artist::ArtistRatingLoader;
use crate::graphql::loaders::rating_label::LabelRatingLoader;
use crate::graphql::loaders::rating_recording::RecordingRatingLoader;
use crate::graphql::loaders::rating_release_group::ReleaseGroupRatingLoader;
use crate::graphql::loaders::relationship::area_id_by_artist::{
    AreaIdsByArtistLoader, BeginAreaIdsByArtistLoader, EndAreaIdsByArtistLoader,
};
use crate::graphql::loaders::relationship::area_id_by_label::AreaIdByLabelLoader;
use crate::graphql::loaders::relationship::artist_credit_id_recording::ArtistCreditIdByRecordingLoader;
use crate::graphql::loaders::relationship::artist_credit_id_release::ArtistCreditIdByReleaseLoader;
use crate::graphql::loaders::relationship::artist_credit_id_release_group::ArtistCreditIdByReleaseGroupLoader;
use crate::graphql::loaders::relationship::genre_id_by_artist::GenreIdsByArtistLoader;
use crate::graphql::loaders::relationship::genre_id_by_label::GenreIdsByLabelLoader;
use crate::graphql::loaders::relationship::genre_id_by_recording::GenreIdsByRecordingLoader;
use crate::graphql::loaders::relationship::genre_id_by_release::GenreIdsByReleaseLoader;
use crate::graphql::loaders::relationship::genre_id_by_release_group::GenreIdsByReleaseGroupLoader;
use crate::graphql::loaders::relationship::medium_id_by_release::MediumIdByReleaseLoader;
use crate::graphql::loaders::relationship::release_group_id_by_artist::ReleaseGroupIdsByArtistLoader;
use crate::graphql::loaders::relationship::release_id_by_artist::ReleaseIdsByArtistLoader;
use crate::graphql::loaders::relationship::release_id_by_label::ReleaseIdsByLabelLoader;
use crate::graphql::loaders::relationship::release_id_by_recording::ReleaseIdsByRecordingLoader;
use crate::graphql::loaders::relationship::release_id_by_release_group::ReleaseIdByReleaseGroupLoader;
use crate::graphql::loaders::relationship::tag_id_by_area::TagIdsByAreaLoader;
use crate::graphql::loaders::relationship::tag_id_by_artist::TagIdsByArtistLoader;
use crate::graphql::loaders::relationship::tag_id_by_label::TagIdsByLabelLoader;
use crate::graphql::loaders::relationship::tag_id_by_recording::TagIdsByRecordingLoader;
use crate::graphql::loaders::relationship::tag_id_by_release::TagIdsByReleaseLoader;
use crate::graphql::loaders::relationship::tag_id_by_release_group::TagIdsByReleaseGroupLoader;
use crate::graphql::loaders::relationship::track_id_by_medium::TrackIdByMediumLoader;
use crate::graphql::loaders::release_event_by_release::ReleaseEventsByReleaseLoader;
use crate::graphql::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub mod loaders;
pub mod query;
pub mod types;

pub fn build_schema(pool: sqlx::PgPool) -> AppSchema {
    let rg_entity_loader = DataLoader::new(ReleaseGroupLoader { pool: pool.clone() }, tokio::spawn);
    let r_entity_loader = DataLoader::new(ReleaseLoader { pool: pool.clone() }, tokio::spawn);
    let l_entity_loader = DataLoader::new(LabelLoader { pool: pool.clone() }, tokio::spawn);
    let medium_entity_loader = DataLoader::new(MediumLoader { pool: pool.clone() }, tokio::spawn);
    let track_entity_loader = DataLoader::new(TrackLoader { pool: pool.clone() }, tokio::spawn);
    let tag_entity_loader = DataLoader::new(TagLoader { pool: pool.clone() }, tokio::spawn);
    let recording_entity_loader =
        DataLoader::new(RecordingLoader { pool: pool.clone() }, tokio::spawn);
    let artist_credit_entity_loader =
        DataLoader::new(ArtistCreditLoader { pool: pool.clone() }, tokio::spawn);
    let artist_entity_loader = DataLoader::new(ArtistLoader { pool: pool.clone() }, tokio::spawn);
    let genre_entity_loader = DataLoader::new(GenreLoader { pool: pool.clone() }, tokio::spawn);
    let area_entity_loader = DataLoader::new(AreaLoader { pool: pool.clone() }, tokio::spawn);

    let rg_a_loader = DataLoader::new(
        ReleaseGroupIdsByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let r_a_loader = DataLoader::new(
        ReleaseIdsByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let r_rg_loader = DataLoader::new(
        ReleaseIdByReleaseGroupLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let li_r_loader = DataLoader::new(
        LabelInfosByReleaseLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let medium_release_loader =
        DataLoader::new(MediumIdByReleaseLoader { pool: pool.clone() }, tokio::spawn);
    let track_medium_loader =
        DataLoader::new(TrackIdByMediumLoader { pool: pool.clone() }, tokio::spawn);
    let release_event_release_loader = DataLoader::new(
        ReleaseEventsByReleaseLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let release_recording_loader = DataLoader::new(
        ReleaseIdsByRecordingLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let release_label_loader =
        DataLoader::new(ReleaseIdsByLabelLoader { pool: pool.clone() }, tokio::spawn);
    let iso_1_loader = DataLoader::new(IsoCode1ByAreaLoader { pool: pool.clone() }, tokio::spawn);
    let iso_2_loader = DataLoader::new(IsoCode2ByAreaLoader { pool: pool.clone() }, tokio::spawn);
    let iso_3_loader = DataLoader::new(IsoCode3ByAreaLoader { pool: pool.clone() }, tokio::spawn);

    let tag_artist_loader =
        DataLoader::new(TagIdsByArtistLoader { pool: pool.clone() }, tokio::spawn);
    let tag_release_group_loader = DataLoader::new(
        TagIdsByReleaseGroupLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let tag_release_loader =
        DataLoader::new(TagIdsByReleaseLoader { pool: pool.clone() }, tokio::spawn);
    let tag_area_loader = DataLoader::new(TagIdsByAreaLoader { pool: pool.clone() }, tokio::spawn);
    let tag_label_loader =
        DataLoader::new(TagIdsByLabelLoader { pool: pool.clone() }, tokio::spawn);
    let tag_recording_loader =
        DataLoader::new(TagIdsByRecordingLoader { pool: pool.clone() }, tokio::spawn);

    let rating_artist_loader =
        DataLoader::new(ArtistRatingLoader { pool: pool.clone() }, tokio::spawn);
    let rating_label_loader =
        DataLoader::new(LabelRatingLoader { pool: pool.clone() }, tokio::spawn);
    let rating_recording_loader =
        DataLoader::new(RecordingRatingLoader { pool: pool.clone() }, tokio::spawn);
    let rating_release_group_loader = DataLoader::new(
        ReleaseGroupRatingLoader { pool: pool.clone() },
        tokio::spawn,
    );

    let artist_credit_release_group_loader = DataLoader::new(
        ArtistCreditIdByReleaseGroupLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let artist_credit_release_loader = DataLoader::new(
        ArtistCreditIdByReleaseLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let artist_credit_recording_group_loader = DataLoader::new(
        ArtistCreditIdByRecordingLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let genre_artist_loader =
        DataLoader::new(GenreIdsByArtistLoader { pool: pool.clone() }, tokio::spawn);
    let genre_release_loader =
        DataLoader::new(GenreIdsByReleaseLoader { pool: pool.clone() }, tokio::spawn);
    let genre_release_group_loader = DataLoader::new(
        GenreIdsByReleaseGroupLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let genre_recording_loader = DataLoader::new(
        GenreIdsByRecordingLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let genre_label_loader =
        DataLoader::new(GenreIdsByLabelLoader { pool: pool.clone() }, tokio::spawn);
    let annotations_area_loader =
        DataLoader::new(AreaAnnotationLoader { pool: pool.clone() }, tokio::spawn);
    let annotations_artist_loader =
        DataLoader::new(ArtistAnnotationLoader { pool: pool.clone() }, tokio::spawn);
    let annotations_release_loader =
        DataLoader::new(ReleaseAnnotationLoader { pool: pool.clone() }, tokio::spawn);
    let annotations_release_group_loader = DataLoader::new(
        ReleaseGroupAnnotationLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let annotations_recording_loader = DataLoader::new(
        RecordingAnnotationLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let annotations_label_loader =
        DataLoader::new(LabelAnnotationLoader { pool: pool.clone() }, tokio::spawn);
    let area_artist_loader =
        DataLoader::new(AreaIdsByArtistLoader { pool: pool.clone() }, tokio::spawn);
    let begin_area_artist_loader = DataLoader::new(
        BeginAreaIdsByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let end_area_artist_loader = DataLoader::new(
        EndAreaIdsByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let area_label_loader =
        DataLoader::new(AreaIdByLabelLoader { pool: pool.clone() }, tokio::spawn);
    let alias_area = DataLoader::new(AreaAliasLoader { pool: pool.clone() }, tokio::spawn);
    let alias_label = DataLoader::new(LabelAliasLoader { pool: pool.clone() }, tokio::spawn);
    let alias_recording =
        DataLoader::new(RecordingAliasLoader { pool: pool.clone() }, tokio::spawn);
    let alias_release = DataLoader::new(ReleaseAliasLoader { pool: pool.clone() }, tokio::spawn);
    let alias_release_group =
        DataLoader::new(ReleaseGroupAliasLoader { pool: pool.clone() }, tokio::spawn);
    let alias_artist = DataLoader::new(ArtistAliasLoader { pool: pool.clone() }, tokio::spawn);
    let ipi_artist = DataLoader::new(ArtistIpiLoader { pool: pool.clone() }, tokio::spawn);
    let ipi_label = DataLoader::new(LabelIpiLoader { pool: pool.clone() }, tokio::spawn);
    let isni_artist = DataLoader::new(ArtistIsniLoader { pool: pool.clone() }, tokio::spawn);
    let isni_label = DataLoader::new(LabelIsniLoader { pool: pool.clone() }, tokio::spawn);

    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .limit_depth(10)
        .limit_complexity(200)
        .data(pool)
        .data(rg_entity_loader)
        .data(r_entity_loader)
        .data(l_entity_loader)
        .data(medium_entity_loader)
        .data(track_entity_loader)
        .data(recording_entity_loader)
        .data(tag_entity_loader)
        .data(artist_credit_entity_loader)
        .data(artist_entity_loader)
        .data(genre_entity_loader)
        .data(area_entity_loader)
        .data(rg_a_loader)
        .data(r_a_loader)
        .data(r_rg_loader)
        .data(li_r_loader)
        .data(medium_release_loader)
        .data(track_medium_loader)
        .data(release_event_release_loader)
        .data(release_recording_loader)
        .data(release_label_loader)
        .data(iso_1_loader)
        .data(iso_2_loader)
        .data(iso_3_loader)
        .data(tag_recording_loader)
        .data(tag_area_loader)
        .data(tag_label_loader)
        .data(tag_artist_loader)
        .data(tag_release_group_loader)
        .data(tag_release_loader)
        .data(rating_artist_loader)
        .data(rating_label_loader)
        .data(rating_recording_loader)
        .data(rating_release_group_loader)
        .data(artist_credit_recording_group_loader)
        .data(artist_credit_release_group_loader)
        .data(artist_credit_release_loader)
        .data(genre_artist_loader)
        .data(genre_label_loader)
        .data(genre_recording_loader)
        .data(genre_release_group_loader)
        .data(genre_release_loader)
        .data(annotations_area_loader)
        .data(annotations_artist_loader)
        .data(annotations_label_loader)
        .data(annotations_recording_loader)
        .data(annotations_release_group_loader)
        .data(annotations_release_loader)
        .data(area_artist_loader)
        .data(begin_area_artist_loader)
        .data(end_area_artist_loader)
        .data(area_label_loader)
        .data(alias_area)
        .data(alias_artist)
        .data(alias_label)
        .data(alias_recording)
        .data(alias_release)
        .data(alias_release_group)
        .data(isni_artist)
        .data(isni_label)
        .data(ipi_artist)
        .data(ipi_label)
        .finish()
}

pub fn build_schema_export() -> AppSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .finish()
}

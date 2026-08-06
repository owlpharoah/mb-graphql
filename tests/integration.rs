#[path = "integration/common/mod.rs"]
mod common;

#[path = "../tests/queries/mod.rs"]
mod queries;

mod integration {
    pub mod area;
    pub mod artist;
    pub mod common;
    pub mod label;
    pub mod recording;
    pub mod release;
    pub mod release_group;
}

#[path = "integration/artist.rs"]
mod artist;

#[path = "integration/release_group.rs"]
mod release_group;

#[path = "integration/release.rs"]
mod release;

#[path = "integration/recording.rs"]
mod recording;

#[path = "integration/label.rs"]
mod label;

#[path = "integration/area.rs"]
mod area;

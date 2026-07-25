pub const RECORDING_BASIC: &str = r#"
    query RecordingBasic($mbid: [String!]!) {
        recording(mbid: $mbid) {
            mbid
            name
            video
        }
    }
"#;

pub const RECORDING_BATCH: &str = r#"
    query RecordingBatch($mbid: [String!]!) {
        recording(mbid: $mbid) {
            mbid
            name
        }
    }
"#;

pub const RECORDING_WITH_RELEASES: &str = r#"
    query RecordingWithReleases($mbid: [String!]!) {
        recording(mbid: $mbid) {
            name
            release { name }
        }
    }
"#;

pub const RECORDING_SECONDARY_FIELDS: &str = r#"
    query RecordingSecondaryFields($mbid: [String!]!) {
        recording(mbid: $mbid) {
            name
            firstReleaseDate { year month day }
            isrc
            rating { value votesCount }
            genres { name }
            annotation
            alias { name }
        }
    }
"#;

pub const RECORDING_ARTIST_CREDIT: &str = r#"
    query RecordingArtistCredit($mbid: [String!]!) {
        recording(mbid: $mbid) {
            artistCredit { name artist { name } }
        }
    }
"#;

pub const RECORDING_NAME_ONLY: &str = r#"
    query RecordingNameOnly($mbid: [String!]!) {
        recording(mbid: $mbid) { name }
    }
"#;

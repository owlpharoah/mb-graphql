pub const RELEASE_GROUP_BASIC: &str = r#"
    query ReleaseGroupBasic($mbid: [String!]!) {
        releaseGroup(mbid: $mbid) {
            mbid
            name
            artistCredit {
                name
                artist { name }
            }
        }
    }
"#;

pub const RELEASE_GROUP_BATCH: &str = r#"
    query ReleaseGroupBatch($mbid: [String!]!) {
        releaseGroup(mbid: $mbid) {
            mbid
            name
        }
    }
"#;

pub const RELEASE_GROUP_WITH_RELEASES: &str = r#"
    query ReleaseGroupWithReleases($mbid: [String!]!) {
        releaseGroup(mbid: $mbid) {
            name
            releases { name }
        }
    }
"#;

pub const RELEASE_GROUP_SECONDARY_FIELDS: &str = r#"
    query ReleaseGroupSecondaryFields($mbid: [String!]!) {
        releaseGroup(mbid: $mbid) {
            name
            type
            secondaryType
            firstReleaseDate { year month day }
            genres { name }
            rating { value votesCount }
            annotation
            alias { name }
        }
    }
"#;

pub const RELEASE_GROUP_NAME_ONLY: &str = r#"
    query ReleaseGroupNameOnly($mbid: [String!]!) {
        releaseGroup(mbid: $mbid) { name }
    }
"#;

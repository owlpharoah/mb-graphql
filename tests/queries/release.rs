pub const RELEASE_BASIC: &str = r#"
    query ReleaseBasic($mbid: [String!]!) {
        release(mbid: $mbid) {
            mbid
            name
            releaseGroup { mbid name }
            labelInfo { catalogNumber label { name } }
        }
    }
"#;

pub const RELEASE_BATCH: &str = r#"
    query ReleaseBatch($mbid: [String!]!) {
        release(mbid: $mbid) {
            mbid
            name
            labelInfo { catalogNumber }
        }
    }
"#;

pub const RELEASE_WITH_MEDIUM: &str = r#"
    query ReleaseWithMedium($mbid: [String!]!) {
        release(mbid: $mbid) {
            name
            medium {
                name
                trackCount
            }
        }
    }
"#;

pub const RELEASE_SECONDARY_FIELDS: &str = r#"
    query ReleaseSecondaryFields($mbid: [String!]!) {
        release(mbid: $mbid) {
            name
            date { year month day }
            asin
            country
            releaseEvents { date { year } country }
            artistCredit { name artist { name } }
            genres { name }
            annotation
            alias { name }
        }
    }
"#;

pub const RELEASE_NAME_ONLY: &str = r#"
    query ReleaseNameOnly($mbid: [String!]!) {
        release(mbid: $mbid) { name }
    }
"#;

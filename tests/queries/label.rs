pub const LABEL_BASIC: &str = r#"
    query LabelBasic($mbid: [String!]!) {
        label(mbid: $mbid) {
            mbid
            name
            area { name isoCode1 }
        }
    }
"#;

pub const LABEL_BATCH: &str = r#"
    query LabelBatch($mbid: [String!]!) {
        label(mbid: $mbid) {
            mbid
            name
            area { name }
        }
    }
"#;

pub const LABEL_WITH_RELEASES: &str = r#"
    query LabelWithReleases($mbid: [String!]!) {
        label(mbid: $mbid) {
            name
            release { name }
        }
    }
"#;

pub const LABEL_SECONDARY_FIELDS: &str = r#"
    query LabelSecondaryFields($mbid: [String!]!) {
        label(mbid: $mbid) {
            name
            type
            ended
            beginDate { year month day }
            endDate { year month day }
            rating { value votesCount }
            genres { name }
            annotation
            ipis
            isnis
            alias { name }
        }
    }
"#;

pub const LABEL_NAME_ONLY: &str = r#"
    query LabelNameOnly($mbid: [String!]!) {
        label(mbid: $mbid) { name }
    }
"#;

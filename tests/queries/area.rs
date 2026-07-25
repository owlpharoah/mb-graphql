pub const AREA_BASIC: &str = r#"
    query AreaBasic($mbid: [String!]!) {
        area(mbid: $mbid) {
            mbid
            name
            ended
            isoCode1
        }
    }
"#;

pub const AREA_BATCH: &str = r#"
    query AreaBatch($mbid: [String!]!) {
        area(mbid: $mbid) {
            mbid
            name
            isoCode1
        }
    }
"#;

pub const AREA_SECONDARY_FIELDS: &str = r#"
    query AreaSecondaryFields($mbid: [String!]!) {
        area(mbid: $mbid) {
            name
            type
            beginDate { year month day }
            endDate { year month day }
            isoCode2
            isoCode3
            tags { name count }
            annotation
            alias { name }
        }
    }
"#;

pub const AREA_NAME_ONLY: &str = r#"
    query AreaNameOnly($mbid: [String!]!) {
        area(mbid: $mbid) { name }
    }
"#;

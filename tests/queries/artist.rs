pub const ARTIST_BASIC: &str = r#"
    query ArtistBasic($mbid: [String!]!) {
        artist(mbid: $mbid) {
            mbid
            name
            gender
            ended
            beginDate { year month day }
        }
    }
"#;

pub const ARTIST_BATCH: &str = r#"
    query ArtistBatch($mbid: [String!]!) {
        artist(mbid: $mbid) {
            mbid
            name
            gender
        }
    }
"#;

pub const ARTIST_RELEASE_GROUPS_AND_RELEASES: &str = r#"
    query ArtistReleaseGroupsAndReleases($mbid: [String!]!) {
        artist(mbid: $mbid) {
            name
            releaseGroups {
                name
                type
                firstReleaseDate { year month day }
            }
            releases {
                name
            }
        }
    }
"#;

pub const ARTIST_SECONDARY_FIELDS: &str = r#"
    query ArtistSecondaryFields($mbid: [String!]!) {
        artist(mbid: $mbid) {
            name
            tags { name count }
            genres { mbid name }
            rating { value votesCount }
            annotation
            area { name }
            beginArea { name }
            endArea { name }
            alias { name sortName type primary }
            ipis
            isnis
        }
    }
"#;

pub const ARTIST_BATCH_WITH_RELEASE_GROUPS: &str = r#"
    query ArtistBatchWithReleaseGroups($mbid: [String!]!) {
        artist(mbid: $mbid) {
            mbid
            name
            releaseGroups { name }
        }
    }
"#;

pub const ARTIST_NAME_ONLY: &str = r#"
    query ArtistNameOnly($mbid: [String!]!) {
        artist(mbid: $mbid) { name }
    }
"#;

use crate::common::{find_by_mbid, run, run_expect_error, test_schema};

const YE: &str = "164f0d73-1234-4e2c-8743-d77bf2191051";
const KIM_KARDASHIAN: &str = "b13c7b85-86bf-41d0-baa7-444f03ec0b38";
const ARCTIC_MONKEYS: &str = "ada7a83c-e3e1-40f1-93f9-3e73dbc9298a";
const EMINEM: &str = "b95ce3ff-3d05-4e87-9e01-c97b66af13d4";

#[tokio::test]
async fn artist_by_mbid_returns_artist() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                artist(mbid: ["{YE}"]) {{
                    mbid
                    name
                    gender
                    ended
                    beginDate {{ year month day }}
                }}
            }}"#
        ),
    )
    .await;

    let artists = data["artist"].as_array().unwrap();
    assert_eq!(artists.len(), 1);

    let ye = find_by_mbid(artists, YE);
    assert_eq!(ye["name"], "Ye");
    assert_eq!(ye["gender"], 1);
    assert_eq!(ye["ended"], false);
    assert_eq!(ye["beginDate"]["year"], 1977);
}

#[tokio::test]
async fn artist_by_multiple_mbids_returns_each_artist() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                artist(mbid: ["{YE}", "{KIM_KARDASHIAN}"]) {{
                    mbid
                    name
                    gender
                }}
            }}"#
        ),
    )
    .await;

    let artists = data["artist"].as_array().unwrap();
    assert_eq!(artists.len(), 2);

    assert_eq!(find_by_mbid(artists, YE)["name"], "Ye");
    assert_eq!(
        find_by_mbid(artists, KIM_KARDASHIAN)["name"],
        "Kim Kardashian"
    );
    assert_eq!(find_by_mbid(artists, KIM_KARDASHIAN)["gender"], 2);
}

#[tokio::test]
async fn artist_release_groups_and_releases_are_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                artist(mbid: ["{ARCTIC_MONKEYS}"]) {{
                    name
                    releaseGroups {{
                        name
                        type
                        firstReleaseDate {{ year month day }}
                    }}
                    releases {{
                        name
                    }}
                }}
            }}"#
        ),
    )
    .await;

    let arctic_monkeys = &data["artist"].as_array().unwrap()[0];
    assert_eq!(arctic_monkeys["name"], "Arctic Monkeys");
    assert!(
        !arctic_monkeys["releaseGroups"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(!arctic_monkeys["releases"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn artist_secondary_fields_resolve_without_error() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                artist(mbid: ["{ARCTIC_MONKEYS}"]) {{
                    name
                    tags {{ name count }}
                    genres {{ mbid name }}
                    rating {{ value votesCount }}
                    annotation
                    area {{ name }}
                    beginArea {{ name }}
                    endArea {{ name }}
                    alias {{ name sortName type primary }}
                    ipis
                    isnis
                }}
            }}"#
        ),
    )
    .await;

    println!("artist secondary fields = {:#?}", data);

    let arctic_monkeys = &data["artist"].as_array().unwrap()[0];
    assert!(arctic_monkeys["tags"].is_array());
    assert!(arctic_monkeys["genres"].is_array());
    assert!(arctic_monkeys["alias"].is_array());
    assert!(arctic_monkeys["ipis"].is_array());
    assert!(arctic_monkeys["isnis"].is_array());
}

#[tokio::test]
async fn artist_by_multiple_mbids_release_groups_are_loaded_for_each() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                artist(mbid: ["{ARCTIC_MONKEYS}", "{EMINEM}"]) {{
                    mbid
                    name
                    releaseGroups {{ name }}
                }}
            }}"#
        ),
    )
    .await;

    let artists = data["artist"].as_array().unwrap();
    assert_eq!(artists.len(), 2);

    assert!(
        !find_by_mbid(artists, ARCTIC_MONKEYS)["releaseGroups"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        !find_by_mbid(artists, EMINEM)["releaseGroups"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn unknown_artist_mbid_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        r#"{ artist(mbid: ["5441c29d-3602-4898-b1a1-b77fa23b8e51"]) { name } }"#,
    )
    .await;

    assert!(data["artist"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn unknown_artist_mbids_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        r#"{
            artist(
                mbid: [
                    "f3bf61f8-97d4-4e52-a73d-2ddbbe8196e8",
                    "c95ce3ff-3d05-4e87-9e01-c97b66af13d4"
                ]
            ) {
                name
            }
        }"#,
    )
    .await;

    assert!(data["artist"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_artist_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(&schema, r#"{ artist(mbid: ["not-a-uuid"]) { name } }"#).await;
}

#[tokio::test]
async fn mixed_valid_and_invalid_artist_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(
        &schema,
        &format!(r#"{{ artist(mbid: ["{YE}", "not-a-uuid"]) {{ name }} }}"#),
    )
    .await;
}

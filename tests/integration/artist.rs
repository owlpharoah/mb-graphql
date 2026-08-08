use serde_json::json;

use crate::common::{find_by_mbid, run, run_expect_error, test_schema};
use crate::queries::{artist, mbids};

#[tokio::test]
async fn artist_by_mbid_returns_artist() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        artist::ARTIST_BASIC,
        json!({ "mbid": [mbids::YE] }),
    )
    .await;

    let artists = data["artist"].as_array().unwrap();
    assert_eq!(artists.len(), 1);

    let ye = find_by_mbid(artists, mbids::YE);
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
        artist::ARTIST_BATCH,
        json!({ "mbid": [mbids::YE, mbids::KIM_KARDASHIAN] }),
    )
    .await;

    let artists = data["artist"].as_array().unwrap();
    assert_eq!(artists.len(), 2);

    assert_eq!(find_by_mbid(artists, mbids::YE)["name"], "Ye");
    assert_eq!(
        find_by_mbid(artists, mbids::KIM_KARDASHIAN)["name"],
        "Kim Kardashian"
    );
    assert_eq!(find_by_mbid(artists, mbids::KIM_KARDASHIAN)["gender"], 2);
}

#[tokio::test]
async fn artist_release_groups_and_releases_are_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        artist::ARTIST_RELEASE_GROUPS_AND_RELEASES,
        json!({ "mbid": [mbids::ARCTIC_MONKEYS] }),
    )
    .await;

    let arctic_monkeys = &data["artist"].as_array().unwrap()[0];
    assert_eq!(arctic_monkeys["name"], "Arctic Monkeys");
    assert!(arctic_monkeys["releaseGroups"].as_array().is_some());
    assert!(arctic_monkeys["releases"].as_array().is_some());
}

#[tokio::test]
async fn artist_secondary_fields_resolve_without_error() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        artist::ARTIST_SECONDARY_FIELDS,
        json!({ "mbid": [mbids::ARCTIC_MONKEYS] }),
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
        artist::ARTIST_BATCH_WITH_RELEASE_GROUPS,
        json!({ "mbid": [mbids::ARCTIC_MONKEYS, mbids::ULTRA_NATE] }),
    )
    .await;

    let artists = data["artist"].as_array().unwrap();
    assert_eq!(artists.len(), 2);

    let am = find_by_mbid(artists, mbids::ARCTIC_MONKEYS);
    assert!(am["releaseGroups"].as_array().is_some());

    let ultra_nate = find_by_mbid(artists, mbids::ULTRA_NATE);
    assert!(ultra_nate["releaseGroups"].as_array().is_some());
}

#[tokio::test]
async fn unknown_artist_mbid_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        artist::ARTIST_NAME_ONLY,
        json!({ "mbid": ["5441c29d-3602-4898-b1a1-b77fa23b8e51"] }),
    )
    .await;

    assert!(data["artist"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn unknown_artist_mbids_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        artist::ARTIST_NAME_ONLY,
        json!({ "mbid": [
            "f3bf61f8-97d4-4e52-a73d-2ddbbe8196e8",
            "c95ce3ff-3d05-4e87-9e01-c97b66af13d4"
        ] }),
    )
    .await;

    assert!(data["artist"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_artist_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(
        &schema,
        artist::ARTIST_NAME_ONLY,
        json!({ "mbid": ["not-a-uuid"] }),
    )
    .await;
}

#[tokio::test]
async fn mixed_valid_and_invalid_artist_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(
        &schema,
        artist::ARTIST_NAME_ONLY,
        json!({ "mbid": [mbids::YE, "not-a-uuid"] }),
    )
    .await;
}

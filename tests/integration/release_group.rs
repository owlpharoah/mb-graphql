use serde_json::json;

use crate::common::{find_by_mbid, run, run_expect_error, test_schema};
use crate::queries::{mbids, release_group};

#[tokio::test]
async fn release_group_by_mbid_returns_release_group() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release_group::RELEASE_GROUP_BASIC,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RG] }),
    )
    .await;

    let groups = data["releaseGroup"].as_array().unwrap();
    assert_eq!(groups.len(), 1);

    let fwn = find_by_mbid(groups, mbids::FAVOURITE_WORST_NIGHTMARE_RG);
    assert_eq!(fwn["name"], "Favourite Worst Nightmare");

    let credits = fwn["artistCredit"].as_array().unwrap();
    assert!(!credits.is_empty());
    assert_eq!(credits[0]["artist"]["name"], "Arctic Monkeys");
}

#[tokio::test]
async fn release_group_by_multiple_mbids_returns_each() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release_group::RELEASE_GROUP_BATCH,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RG, mbids::HUMBUG_RG] }),
    )
    .await;

    let groups = data["releaseGroup"].as_array().unwrap();
    assert_eq!(groups.len(), 2);
    assert_eq!(
        find_by_mbid(groups, mbids::FAVOURITE_WORST_NIGHTMARE_RG)["name"],
        "Favourite Worst Nightmare"
    );
    assert_eq!(find_by_mbid(groups, mbids::HUMBUG_RG)["name"], "Humbug");
}

#[tokio::test]
async fn release_group_releases_are_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release_group::RELEASE_GROUP_WITH_RELEASES,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RG] }),
    )
    .await;

    let fwn = &data["releaseGroup"].as_array().unwrap()[0];
    assert!(!fwn["releases"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn release_group_secondary_fields_resolve_without_error() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release_group::RELEASE_GROUP_SECONDARY_FIELDS,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RG] }),
    )
    .await;

    println!("release group secondary fields = {:#?}", data);

    let fwn = &data["releaseGroup"].as_array().unwrap()[0];
    assert!(fwn["genres"].is_array());
    assert!(fwn["alias"].is_array());
}

#[tokio::test]
async fn unknown_release_group_mbid_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release_group::RELEASE_GROUP_NAME_ONLY,
        json!({ "mbid": ["f113fa38-7908-3ec9-8145-d2455e78a8b3"] }),
    )
    .await;

    assert!(data["releaseGroup"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_release_group_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(
        &schema,
        release_group::RELEASE_GROUP_NAME_ONLY,
        json!({ "mbid": ["not-a-uuid"] }),
    )
    .await;
}

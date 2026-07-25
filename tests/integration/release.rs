use serde_json::json;

use crate::common::{find_by_mbid, run, run_expect_error, test_schema};
use crate::queries::{mbids, release};

#[tokio::test]
async fn release_by_mbid_returns_release() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release::RELEASE_BASIC,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RELEASE] }),
    )
    .await;

    let releases = data["release"].as_array().unwrap();
    assert_eq!(releases.len(), 1);

    let fwn = find_by_mbid(releases, mbids::FAVOURITE_WORST_NIGHTMARE_RELEASE);
    assert_eq!(fwn["name"], "Favourite Worst Nightmare");
    assert_eq!(
        fwn["releaseGroup"]["mbid"],
        mbids::FAVOURITE_WORST_NIGHTMARE_RG
    );

    let label_info = fwn["labelInfo"].as_array().unwrap();
    assert!(!label_info.is_empty());
    assert!(
        label_info
            .iter()
            .any(|li| li["catalogNumber"] == "WIGCD188")
    );
}

#[tokio::test]
async fn release_by_multiple_mbids_returns_each() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release::RELEASE_BATCH,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RELEASE, mbids::HUMBUG_RELEASE] }),
    )
    .await;

    let releases = data["release"].as_array().unwrap();
    assert_eq!(releases.len(), 2);

    let humbug = find_by_mbid(releases, mbids::HUMBUG_RELEASE);
    assert_eq!(humbug["name"], "Humbug");
    assert!(
        humbug["labelInfo"]
            .as_array()
            .unwrap()
            .iter()
            .any(|li| li["catalogNumber"] == "WIGCD220")
    );
}

#[tokio::test]
async fn release_medium_is_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release::RELEASE_WITH_MEDIUM,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RELEASE] }),
    )
    .await;

    let fwn = &data["release"].as_array().unwrap()[0];
    let media = fwn["medium"].as_array().unwrap();
    assert!(!media.is_empty());
    assert!(media[0]["trackCount"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn release_secondary_fields_resolve_without_error() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release::RELEASE_SECONDARY_FIELDS,
        json!({ "mbid": [mbids::FAVOURITE_WORST_NIGHTMARE_RELEASE] }),
    )
    .await;

    println!("release secondary fields = {:#?}", data);

    let fwn = &data["release"].as_array().unwrap()[0];
    assert!(fwn["releaseEvents"].is_array());
    assert!(fwn["genres"].is_array());
    assert!(fwn["alias"].is_array());

    let credits = fwn["artistCredit"].as_array().unwrap();
    assert!(!credits.is_empty());
    assert_eq!(credits[0]["artist"]["name"], "Arctic Monkeys");
}

#[tokio::test]
async fn unknown_release_mbid_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        release::RELEASE_NAME_ONLY,
        json!({ "mbid": ["f68c985d-f18b-4f4a-b7f0-87837cf3fbfa"] }),
    )
    .await;

    assert!(data["release"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_release_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(
        &schema,
        release::RELEASE_NAME_ONLY,
        json!({ "mbid": ["not-a-uuid"] }),
    )
    .await;
}

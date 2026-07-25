use serde_json::json;

use crate::common::{find_by_mbid, run, run_expect_error, test_schema};
use crate::queries::{label, mbids};

#[tokio::test]
async fn label_by_mbid_returns_label() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        label::LABEL_BASIC,
        json!({ "mbid": [mbids::PARLOPHONE] }),
    )
    .await;

    let labels = data["label"].as_array().unwrap();
    assert_eq!(labels.len(), 1);

    let parlophone = find_by_mbid(labels, mbids::PARLOPHONE);
    assert_eq!(parlophone["name"], "Parlophone");
    assert_eq!(parlophone["area"]["name"], "United Kingdom");
}

#[tokio::test]
async fn label_by_multiple_mbids_returns_each() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        label::LABEL_BATCH,
        json!({ "mbid": [mbids::RCA_RECORDS, mbids::PARLOPHONE] }),
    )
    .await;

    let labels = data["label"].as_array().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(
        find_by_mbid(labels, mbids::RCA_RECORDS)["area"]["name"],
        "United States"
    );
    assert_eq!(
        find_by_mbid(labels, mbids::PARLOPHONE)["area"]["name"],
        "United Kingdom"
    );
}

#[tokio::test]
async fn label_releases_are_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        label::LABEL_WITH_RELEASES,
        json!({ "mbid": [mbids::RCA_RECORDS] }),
    )
    .await;

    let rca = &data["label"].as_array().unwrap()[0];
    assert!(!rca["release"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn label_secondary_fields_resolve_without_error() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        label::LABEL_SECONDARY_FIELDS,
        json!({ "mbid": [mbids::RCA_RECORDS] }),
    )
    .await;

    println!("label secondary fields = {:#?}", data);

    let rca = &data["label"].as_array().unwrap()[0];
    assert!(rca["genres"].is_array());
    assert!(rca["alias"].is_array());
    assert!(rca["ipis"].is_array());
    assert!(rca["isnis"].is_array());
}

#[tokio::test]
async fn unknown_label_mbid_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        label::LABEL_NAME_ONLY,
        json!({ "mbid": ["1ca5ed29-e00b-4ea5-b817-0bcca0e04947"] }),
    )
    .await;

    assert!(data["label"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_label_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(
        &schema,
        label::LABEL_NAME_ONLY,
        json!({ "mbid": ["not-a-uuid"] }),
    )
    .await;
}

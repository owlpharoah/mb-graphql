use crate::common::{find_by_mbid, run, run_expect_error, test_schema};

const FIVE_OH_FIVE: &str = "8dee0224-bcf9-4023-a805-9562bafd3450";
const CORNERSTONE: &str = "cea6f31e-bc36-41e6-b37c-c2a6cdfe166a";

#[tokio::test]
async fn recording_by_mbid_returns_recording() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                recording(mbid: ["{FIVE_OH_FIVE}"]) {{
                    mbid
                    name
                    video
                }}
            }}"#
        ),
    )
    .await;

    let recordings = data["recording"].as_array().unwrap();
    assert_eq!(recordings.len(), 1);

    let five_oh_five = find_by_mbid(recordings, FIVE_OH_FIVE);
    assert_eq!(five_oh_five["name"], "505");
}

#[tokio::test]
async fn recording_by_multiple_mbids_returns_each() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                recording(mbid: ["{FIVE_OH_FIVE}", "{CORNERSTONE}"]) {{
                    mbid
                    name
                }}
            }}"#
        ),
    )
    .await;

    let recordings = data["recording"].as_array().unwrap();
    assert_eq!(recordings.len(), 2);
    assert_eq!(find_by_mbid(recordings, FIVE_OH_FIVE)["name"], "505");
    assert_eq!(find_by_mbid(recordings, CORNERSTONE)["name"], "Cornerstone");
}

#[tokio::test]
async fn recording_releases_are_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                recording(mbid: ["{FIVE_OH_FIVE}"]) {{
                    name
                    release {{ name }}
                }}
            }}"#
        ),
    )
    .await;

    let five_oh_five = &data["recording"].as_array().unwrap()[0];
    assert!(!five_oh_five["release"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn recording_secondary_fields_resolve_without_error() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                recording(mbid: ["{FIVE_OH_FIVE}"]) {{
                    name
                    firstReleaseDate {{ year month day }}
                    isrc
                    rating {{ value votesCount }}
                    genres {{ name }}
                    annotation
                    alias {{ name }}
                }}
            }}"#
        ),
    )
    .await;

    println!("recording secondary fields = {:#?}", data);

    let five_oh_five = &data["recording"].as_array().unwrap()[0];
    assert!(five_oh_five["genres"].is_array());
    assert!(five_oh_five["alias"].is_array());
}

#[tokio::test]
async fn recording_artist_credit_returns_artist() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                recording(mbid: ["{FIVE_OH_FIVE}"]) {{
                    artistCredit {{ name artist {{ name }} }}
                }}
            }}"#
        ),
    )
    .await;

    let five_oh_five = &data["recording"].as_array().unwrap()[0];
    let credits = five_oh_five["artistCredit"].as_array().unwrap();
    assert!(!credits.is_empty());
    assert_eq!(credits[0]["artist"]["name"], "Arctic Monkeys");
}

#[tokio::test]
async fn unknown_recording_mbid_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        r#"{ recording(mbid: ["8dee0224-bcf9-4023-a805-9562bafd3451"]) { name } }"#,
    )
    .await;

    assert!(data["recording"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_recording_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(&schema, r#"{ recording(mbid: ["not-a-uuid"]) { name } }"#).await;
}

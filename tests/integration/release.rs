use crate::common::{find_by_mbid, run, run_expect_error, test_schema};

const FAVOURITE_WORST_NIGHTMARE_RELEASE: &str = "f68c985d-f18b-4f4a-b7f0-87837cf3fbf9";
const FAVOURITE_WORST_NIGHTMARE_RELEASE_GROUP: &str = "f113fa38-7908-3ec9-8145-d2455e78a8b2";
const HUMBUG_RELEASE: &str = "a681034b-b886-4df9-aff2-f47efcf96f2f";

#[tokio::test]
async fn release_by_mbid_returns_release() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                release(mbid: ["{FAVOURITE_WORST_NIGHTMARE_RELEASE}"]) {{
                    mbid
                    name
                    releaseGroup {{ mbid name }}
                    labelInfo {{ catalogNumber label {{ name }} }}
                }}
            }}"#
        ),
    )
    .await;

    let releases = data["release"].as_array().unwrap();
    assert_eq!(releases.len(), 1);

    let fwn = find_by_mbid(releases, FAVOURITE_WORST_NIGHTMARE_RELEASE);
    assert_eq!(fwn["name"], "Favourite Worst Nightmare");
    assert_eq!(
        fwn["releaseGroup"]["mbid"],
        FAVOURITE_WORST_NIGHTMARE_RELEASE_GROUP
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
        &format!(
            r#"{{
                release(mbid: ["{FAVOURITE_WORST_NIGHTMARE_RELEASE}", "{HUMBUG_RELEASE}"]) {{
                    mbid
                    name
                    labelInfo {{ catalogNumber }}
                }}
            }}"#
        ),
    )
    .await;

    let releases = data["release"].as_array().unwrap();
    assert_eq!(releases.len(), 2);

    let humbug = find_by_mbid(releases, HUMBUG_RELEASE);
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
        &format!(
            r#"{{
                release(mbid: ["{FAVOURITE_WORST_NIGHTMARE_RELEASE}"]) {{
                    name
                    medium {{
                        name
                        trackCount
                        tracks {{ name position }}
                    }}
                }}
            }}"#
        ),
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
        &format!(
            r#"{{
                release(mbid: ["{FAVOURITE_WORST_NIGHTMARE_RELEASE}"]) {{
                    name
                    date {{ year month day }}
                    asin
                    country
                    releaseEvents {{ date {{ year }} country }}
                    artistCredit {{ name artist {{ name }} }}
                    genres {{ name }}
                    annotation
                    alias {{ name }}
                }}
            }}"#
        ),
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
        r#"{ release(mbid: ["f68c985d-f18b-4f4a-b7f0-87837cf3fbfa"]) { name } }"#,
    )
    .await;

    assert!(data["release"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_release_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(&schema, r#"{ release(mbid: ["not-a-uuid"]) { name } }"#).await;
}

use crate::common::{find_by_mbid, run, run_expect_error, test_schema};

const UNITED_KINGDOM: &str = "8a754a16-0027-3a29-b6d7-2b40ea0481ed";
const UNITED_STATES: &str = "489ce91b-6658-3307-9877-795b68554c98";

#[tokio::test]
async fn area_by_mbid_returns_area() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                area(mbid: ["{UNITED_KINGDOM}"]) {{
                    mbid
                    name
                    ended
                    isoCode1
                }}
            }}"#
        ),
    )
    .await;

    let areas = data["area"].as_array().unwrap();
    assert_eq!(areas.len(), 1);

    let uk = find_by_mbid(areas, UNITED_KINGDOM);
    assert_eq!(uk["name"], "United Kingdom");
    assert_eq!(uk["ended"], false);
    assert!(
        uk["isoCode1"]
            .as_array()
            .unwrap()
            .iter()
            .any(|code| code == "GB")
    );
}

#[tokio::test]
async fn area_by_multiple_mbids_returns_each() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                area(mbid: ["{UNITED_KINGDOM}", "{UNITED_STATES}"]) {{
                    mbid
                    name
                    isoCode1
                }}
            }}"#
        ),
    )
    .await;

    let areas = data["area"].as_array().unwrap();
    assert_eq!(areas.len(), 2);

    let uk = find_by_mbid(areas, UNITED_KINGDOM);
    let us = find_by_mbid(areas, UNITED_STATES);
    assert!(uk["isoCode1"].as_array().unwrap().iter().any(|c| c == "GB"));
    assert!(us["isoCode1"].as_array().unwrap().iter().any(|c| c == "US"));
}

#[tokio::test]
async fn area_secondary_fields_resolve_without_error() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                area(mbid: ["{UNITED_KINGDOM}"]) {{
                    name
                    type
                    beginDate {{ year month day }}
                    endDate {{ year month day }}
                    isoCode2
                    isoCode3
                    tags {{ name count }}
                    annotation
                    alias {{ name }}
                }}
            }}"#
        ),
    )
    .await;

    println!("area secondary fields = {:#?}", data);

    let uk = &data["area"].as_array().unwrap()[0];
    assert!(uk["tags"].is_array());
    assert!(uk["alias"].is_array());
}

#[tokio::test]
async fn unknown_area_mbid_returns_empty_list() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        r#"{ area(mbid: ["8a754a16-0027-3a29-b6d7-2b40ea0481ee"]) { name } }"#,
    )
    .await;

    assert!(data["area"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_area_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(&schema, r#"{ area(mbid: ["not-a-uuid"]) { name } }"#).await;
}

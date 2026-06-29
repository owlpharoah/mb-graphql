use crate::common::{find_by_mbid, run, run_expect_error, test_schema};

const RCA_RECORDS: &str = "1ca5ed29-e00b-4ea5-b817-0bcca0e04946";
const PARLOPHONE: &str = "df7d1c7f-ef95-425f-8eef-445b3d7bcbd9";

#[tokio::test]
async fn label_by_mbid_returns_label() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                label(mbid: ["{PARLOPHONE}"]) {{
                    mbid
                    name
                    area {{ name isoCode1 }}
                }}
            }}"#
        ),
    )
    .await;

    let labels = data["label"].as_array().unwrap();
    assert_eq!(labels.len(), 1);

    let parlophone = find_by_mbid(labels, PARLOPHONE);
    assert_eq!(parlophone["name"], "Parlophone");
    assert_eq!(parlophone["area"]["name"], "United Kingdom");
}

#[tokio::test]
async fn label_by_multiple_mbids_returns_each() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                label(mbid: ["{RCA_RECORDS}", "{PARLOPHONE}"]) {{
                    mbid
                    name
                    area {{ name }}
                }}
            }}"#
        ),
    )
    .await;

    let labels = data["label"].as_array().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(
        find_by_mbid(labels, RCA_RECORDS)["area"]["name"],
        "United States"
    );
    assert_eq!(
        find_by_mbid(labels, PARLOPHONE)["area"]["name"],
        "United Kingdom"
    );
}

#[tokio::test]
async fn label_releases_are_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                label(mbid: ["{RCA_RECORDS}"]) {{
                    name
                    release {{ name }}
                }}
            }}"#
        ),
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
        &format!(
            r#"{{
                label(mbid: ["{RCA_RECORDS}"]) {{
                    name
                    type
                    ended
                    beginDate {{ year month day }}
                    endDate {{ year month day }}
                    rating {{ value votesCount }}
                    genres {{ name }}
                    annotation
                    ipis
                    isnis
                    alias {{ name }}
                }}
            }}"#
        ),
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
        r#"{ label(mbid: ["1ca5ed29-e00b-4ea5-b817-0bcca0e04947"]) { name } }"#,
    )
    .await;

    assert!(data["label"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_label_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(&schema, r#"{ label(mbid: ["not-a-uuid"]) { name } }"#).await;
}

use crate::common::{find_by_mbid, run, run_expect_error, test_schema};

const FAVOURITE_WORST_NIGHTMARE: &str = "f113fa38-7908-3ec9-8145-d2455e78a8b2";
const HUMBUG: &str = "23b7e9e9-8820-4a49-b44f-a7a60e0a7e81";

#[tokio::test]
async fn release_group_by_mbid_returns_release_group() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                releaseGroup(mbid: ["{FAVOURITE_WORST_NIGHTMARE}"]) {{
                    mbid
                    name
                    artistCredit {{
                        name
                        artist {{ name }}
                    }}
                }}
            }}"#
        ),
    )
    .await;

    let groups = data["releaseGroup"].as_array().unwrap();
    assert_eq!(groups.len(), 1);

    let fwn = find_by_mbid(groups, FAVOURITE_WORST_NIGHTMARE);
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
        &format!(
            r#"{{
                releaseGroup(mbid: ["{FAVOURITE_WORST_NIGHTMARE}", "{HUMBUG}"]) {{
                    mbid
                    name
                }}
            }}"#
        ),
    )
    .await;

    let groups = data["releaseGroup"].as_array().unwrap();
    assert_eq!(groups.len(), 2);
    assert_eq!(
        find_by_mbid(groups, FAVOURITE_WORST_NIGHTMARE)["name"],
        "Favourite Worst Nightmare"
    );
    assert_eq!(find_by_mbid(groups, HUMBUG)["name"], "Humbug");
}

#[tokio::test]
async fn release_group_releases_are_loaded() {
    let schema = test_schema().await;

    let data = run(
        &schema,
        &format!(
            r#"{{
                releaseGroup(mbid: ["{FAVOURITE_WORST_NIGHTMARE}"]) {{
                    name
                    releases {{ name }}
                }}
            }}"#
        ),
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
        &format!(
            r#"{{
                releaseGroup(mbid: ["{FAVOURITE_WORST_NIGHTMARE}"]) {{
                    name
                    type
                    secondaryType
                    firstReleaseDate {{ year month day }}
                    genres {{ name }}
                    rating {{ value votesCount }}
                    annotation
                    alias {{ name }}
                }}
            }}"#
        ),
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
        r#"{ releaseGroup(mbid: ["f113fa38-7908-3ec9-8145-d2455e78a8b3"]) { name } }"#,
    )
    .await;

    assert!(data["releaseGroup"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn invalid_release_group_uuid_returns_error() {
    let schema = test_schema().await;

    run_expect_error(
        &schema,
        r#"{ releaseGroup(mbid: ["not-a-uuid"]) { name } }"#,
    )
    .await;
}

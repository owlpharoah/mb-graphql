// tests/artist.rs

use crate::common::test_schema;
use async_graphql::Request;

#[tokio::test]
async fn artist_by_mbid_returns_artist() {
    let schema = test_schema().await;

    let query = r#"
    {
        artist(mbid:"164f0d73-1234-4e2c-8743-d77bf2191051"){
            name
            gender
            ended
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    assert!(response.errors.is_empty());

    let json = response.data.into_json().unwrap();

    assert_eq!(json["artist"]["name"], "Ye");
    assert_eq!(json["artist"]["gender"], 1);
    assert_eq!(json["artist"]["ended"], false);
}

#[tokio::test]
async fn artists_by_mbids_returns_multiple_artists() {
    let schema = test_schema().await;

    let query = r#"
    {
        artists(
            mbids:[
                "164f0d73-1234-4e2c-8743-d77bf2191051",
                "b13c7b85-86bf-41d0-baa7-444f03ec0b38"
            ]
        ){
            name
            gender
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    assert!(response.errors.is_empty());

    let json = response.data.into_json().unwrap();

    let artists = json["artists"].as_array().unwrap();

    assert_eq!(artists.len(), 2);

    assert_eq!(artists[0]["name"], "Ye");
    assert_eq!(artists[0]["gender"], 1);

    assert_eq!(artists[1]["name"], "Kim Kardashian");
    assert_eq!(artists[1]["gender"], 2);
}

#[tokio::test]
async fn artist_release_groups_are_loaded() {
    let schema = test_schema().await;

    let query = r#"
    {
        artist(mbid:"5441c29d-3602-4898-b1a1-b77fa23b8e50"){
            name
            releaseGroup{
                name
                type
                firstReleaseDate{
                    year
                    month
                    day
                }
            }
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    println!("errors = {:#?}", response.errors);
    println!("data = {:#?}", response.data);

    assert!(response.errors.is_empty());

    let json = response.data.into_json().unwrap();

    assert_eq!(json["artist"]["name"], "David Bowie");

    let release_groups = json["artist"]["releaseGroup"].as_array().unwrap();

    assert!(!release_groups.is_empty());
}

#[tokio::test]
async fn artists_release_groups_are_loaded() {
    let schema = test_schema().await;

    let query = r#"
    {
        artists(
            mbids:[
                "5441c29d-3602-4898-b1a1-b77fa23b8e50",
                "b95ce3ff-3d05-4e87-9e01-c97b66af13d4"
            ]
        ){
            name
            releaseGroup{
                name
                type
                firstReleaseDate{
                    year
                    month
                    day
                }
            }
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    println!("errors = {:#?}", response.errors);
    println!("data = {:#?}", response.data);

    assert!(response.errors.is_empty());

    let json = response.data.into_json().unwrap();

    let artists = json["artists"].as_array().unwrap();

    assert_eq!(artists.len(), 2);

    assert!(!artists[0]["releaseGroup"].as_array().unwrap().is_empty());

    assert!(!artists[1]["releaseGroup"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn unknown_artist_returns_null() {
    let schema = test_schema().await;

    let query = r#"
    {
        artist(mbid:"5441c29d-3602-4898-b1a1-b77fa23b8e51"){
            name
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    assert!(response.errors.is_empty());

    let json = response.data.into_json().unwrap();

    assert!(json["artist"].is_null());
}

#[tokio::test]
async fn artists_returns_empty_for_unknown_mbids() {
    let schema = test_schema().await;

    let query = r#"
    {
        artists(
            mbids:[
                "f3bf61f8-97d4-4e52-a73d-2ddbbe8196e8",
                "c95ce3ff-3d05-4e87-9e01-c97b66af13d4"
            ]
        ){
            name
            releaseGroup{
                name
                type
                firstReleaseDate{
                    year
                    month
                    day
                }
            }
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    assert!(response.errors.is_empty());

    let json = response.data.into_json().unwrap();

    let artists = json["artists"].as_array().unwrap();

    assert!(artists.is_empty());
}

#[tokio::test]
async fn invalid_artist_uuid_returns_error() {
    let schema = test_schema().await;

    let query = r#"
    {
        artist(mbid:"not-a-uuid"){
            name
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    assert!(!response.errors.is_empty());
}

#[tokio::test]
async fn invalid_artists_uuid_returns_error() {
    let schema = test_schema().await;

    let query = r#"
    {
        artists(
            mbids:[
                "164f0d73-1234-4e2c-8743-d77bf2191051",
                "not-a-uuid"
            ]
        ){
            name
        }
    }
    "#;

    let response = schema.execute(Request::new(query)).await;

    assert!(!response.errors.is_empty());
}

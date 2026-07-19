mod common;

use reth_custom_db::rpc::sqlite::SqliteEntityApiClient;

use crate::common::get_http_client;
use common::start_sqlite_server;

#[tokio::test]
async fn test_sqlite_save_get() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();

    let value = SqliteEntityApiClient::get(&http_client, "k1".to_string())
        .await
        .unwrap();
    assert_eq!(value, Some("v1".to_string()));
}

#[tokio::test]
async fn test_sqlite_save_overwrite() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v11".to_string())
        .await
        .unwrap();

    let value = SqliteEntityApiClient::get(&http_client, "k1".to_string())
        .await
        .unwrap();
    assert_eq!(value, Some("v11".to_string()));
}

#[tokio::test]
async fn test_sqlite_delete() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    let deleted = SqliteEntityApiClient::delete(&http_client, "k1".to_string())
        .await
        .unwrap();
    assert!(deleted);

    let value = SqliteEntityApiClient::get(&http_client, "k1".to_string())
        .await
        .unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn test_sqlite_get_missing() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let value = SqliteEntityApiClient::get(&http_client, "k_nonexistent".to_string())
        .await
        .unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn test_sqlite_delete_missing() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let deleted = SqliteEntityApiClient::delete(&http_client, "k_nonexistent".to_string())
        .await
        .unwrap();
    assert!(!deleted);
}

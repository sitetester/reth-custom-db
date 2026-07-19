#![allow(dead_code)]

mod common;

use crate::common::{get_http_client, start_reth_server};
use reth_custom_db::rpc::reth::RethEntityApiClient;

#[tokio::test]
async fn test_reth_save_get() {
    let (addr, _) = start_reth_server().await;
    let client = get_http_client(&addr);

    RethEntityApiClient::save(&client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();

    let value = RethEntityApiClient::get(&client, "k1".to_string())
        .await
        .unwrap();
    assert_eq!(value, Some("v1".to_string()));
}

#[tokio::test]
async fn test_reth_save_overwrite() {
    let (addr, _) = start_reth_server().await;
    let client = get_http_client(&addr);
    RethEntityApiClient::save(&client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    RethEntityApiClient::save(&client, "k1".to_string(), "v11".to_string())
        .await
        .unwrap();

    let value = RethEntityApiClient::get(&client, "k1".to_string())
        .await
        .unwrap();
    assert_eq!(value, Some("v11".to_string()));
}

#[tokio::test]
async fn test_reth_delete() {
    let (addr, _) = start_reth_server().await;
    let client = get_http_client(&addr);
    RethEntityApiClient::save(&client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    let deleted = RethEntityApiClient::delete(&client, "k1".to_string())
        .await
        .unwrap();
    assert!(deleted);

    let value = RethEntityApiClient::get(&client, "k1".to_string())
        .await
        .unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn test_reth_get_missing() {
    let (addr, _) = start_reth_server().await;
    let client = get_http_client(&addr);
    let value = RethEntityApiClient::get(&client, "k_nonexistent".to_string())
        .await
        .unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn test_reth_delete_missing() {
    let (addr, _) = start_reth_server().await;
    let client = get_http_client(&addr);
    let deleted = RethEntityApiClient::delete(&client, "k_nonexistent".to_string())
        .await
        .unwrap();
    assert!(!deleted);
}

mod common;

use reth_custom_db::{
    db::{EntityEvent, EventType},
    rpc::sqlite::SqliteEntityApiClient,
};

use crate::common::{get_http_client, get_ws_client};
use common::start_sqlite_server;

#[tokio::test]
async fn test_sqlite_subscribe_save_event() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let ws_client = get_ws_client(&addr).await;

    let mut subscription =
        SqliteEntityApiClient::subscribe_events(&ws_client, Some("k1".to_string()))
            .await
            .unwrap();

    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();

    let event = subscription.next().await.unwrap().unwrap();
    assert_eq!(
        event,
        EntityEvent {
            event: EventType::Saved,
            key: "k1".into(),
            value: Some("v1".into())
        }
    );
}

#[tokio::test]
async fn test_sqlite_subscribe_delete_event() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let ws_client = get_ws_client(&addr).await;

    let mut subscription = SqliteEntityApiClient::subscribe_events(&ws_client, None)
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    let _ = subscription.next().await.unwrap().unwrap(); // consume save event

    SqliteEntityApiClient::delete(&http_client, "k1".to_string())
        .await
        .unwrap();

    let event = subscription.next().await.unwrap().unwrap();
    assert_eq!(
        event,
        EntityEvent {
            event: EventType::Deleted,
            key: "k1".into(),
            value: None
        }
    );
}

#[tokio::test]
async fn test_sqlite_subscribe_key_filter() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let ws_client = get_ws_client(&addr).await;
    let mut subscription =
        SqliteEntityApiClient::subscribe_events(&ws_client, Some("k1".to_string()))
            .await
            .unwrap();

    SqliteEntityApiClient::save(&http_client, "k2".to_string(), "v2".to_string())
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();

    let event = subscription.next().await.unwrap().unwrap();
    assert_eq!(event.key, "k1");
}

#[tokio::test]
async fn test_sqlite_subscribe_all_keys() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let ws_client = get_ws_client(&addr).await;

    let mut subscription = SqliteEntityApiClient::subscribe_events(&ws_client, None)
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    SqliteEntityApiClient::save(&http_client, "k2".to_string(), "v2".to_string())
        .await
        .unwrap();
    SqliteEntityApiClient::save(&http_client, "k3".to_string(), "v3".to_string())
        .await
        .unwrap();

    let event1 = subscription.next().await.unwrap().unwrap();
    let event2 = subscription.next().await.unwrap().unwrap();
    let event3 = subscription.next().await.unwrap().unwrap();

    assert_eq!(event1.key, "k1");
    assert_eq!(event2.key, "k2");
    assert_eq!(event3.key, "k3");
}

#[tokio::test]
async fn test_sqlite_subscribe_no_matching_filter() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let ws_client = get_ws_client(&addr).await;

    let mut subscription =
        SqliteEntityApiClient::subscribe_events(&ws_client, Some("k1".to_string()))
            .await
            .unwrap();

    SqliteEntityApiClient::save(&http_client, "k2".to_string(), "v2".to_string())
        .await
        .unwrap();
    SqliteEntityApiClient::save(&http_client, "k3".to_string(), "v3".to_string())
        .await
        .unwrap();

    let result =
        tokio::time::timeout(std::time::Duration::from_millis(100), subscription.next()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_sqlite_subscribe_multiple_subscribers() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let ws_client = get_ws_client(&addr).await;
    let ws_client2 = get_ws_client(&addr).await;

    let mut subscription1 = SqliteEntityApiClient::subscribe_events(&ws_client, None)
        .await
        .unwrap();
    let mut subscription2 = SqliteEntityApiClient::subscribe_events(&ws_client2, None)
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();

    let event1 = subscription1.next().await.unwrap().unwrap();
    let event2 = subscription2.next().await.unwrap().unwrap();

    assert_eq!(event1.key, "k1");
    assert_eq!(event2.key, "k1");
}

#[tokio::test]
async fn test_sqlite_subscribe_server_survives_client_drop() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);
    let ws_client = get_ws_client(&addr).await;

    let mut subscription = SqliteEntityApiClient::subscribe_events(&ws_client, None)
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    let _ = subscription.next().await.unwrap().unwrap();

    drop(subscription);
    drop(ws_client);

    let ws_client2 = get_ws_client(&addr).await;
    let mut subscription2 = SqliteEntityApiClient::subscribe_events(&ws_client2, None)
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k2".to_string(), "v2".to_string())
        .await
        .unwrap();

    let event = subscription2.next().await.unwrap().unwrap();
    assert_eq!(event.key, "k2");
}

#[tokio::test]
async fn test_sqlite_subscribe_reconnect() {
    let addr = start_sqlite_server().await;
    let http_client = get_http_client(&addr);

    let ws_client1 = get_ws_client(&addr).await;
    let mut subscription1 = SqliteEntityApiClient::subscribe_events(&ws_client1, None)
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k1".to_string(), "v1".to_string())
        .await
        .unwrap();
    let _ = subscription1.next().await.unwrap().unwrap();

    drop(subscription1);
    drop(ws_client1);

    let ws_client2 = get_ws_client(&addr).await;
    let mut subscription2 = SqliteEntityApiClient::subscribe_events(&ws_client2, None)
        .await
        .unwrap();

    SqliteEntityApiClient::save(&http_client, "k2".to_string(), "v2".to_string())
        .await
        .unwrap();

    let event = subscription2.next().await.unwrap().unwrap();
    assert_eq!(
        event,
        EntityEvent {
            event: EventType::Saved,
            key: "k2".into(),
            value: Some("v2".into())
        }
    );
}

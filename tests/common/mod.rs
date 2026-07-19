#![allow(dead_code)]

use jsonrpsee::async_client::Client;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::ws_client::WsClientBuilder;
use reth_custom_db::{
    db::{BlockEvent, MdbxDb, SqliteDb},
    rpc::{
        notifier::EntityEventNotifier,
        reth::{RethEntityApiImpl, RethEntityApiServer},
        sqlite::{SqliteEntityApiImpl, SqliteEntityApiServer},
    },
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

static DB_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub async fn start_sqlite_server() -> SocketAddr {
    let id = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
    let db_path = std::env::temp_dir().join(format!("sqlite_test_{}.db", id));
    let _ = std::fs::remove_file(&db_path);
    let db_conn = SqliteDb::open(db_path.to_str().unwrap()).unwrap();

    let notifier = Arc::new(EntityEventNotifier::new());
    let api = SqliteEntityApiImpl::new(db_conn, notifier);

    let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().unwrap();
    let handle = server.start(api.into_rpc());
    tokio::spawn(handle.stopped());
    addr
}

pub fn get_http_client(addr: &SocketAddr) -> HttpClient {
    let http_uri = format!("http://{addr}");
    HttpClientBuilder::default().build(&http_uri).unwrap()
}

pub async fn get_ws_client(addr: &SocketAddr) -> Client {
    let ws_uri = format!("ws://{addr}");
    WsClientBuilder::default().build(&ws_uri).await.unwrap()
}

pub async fn start_reth_server() -> (SocketAddr, tokio::sync::broadcast::Sender<BlockEvent>) {
    let id = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
    let db_path = std::env::temp_dir().join(format!("reth_test_{}", id));

    let _ = std::fs::remove_dir_all(&db_path);
    let db_conn = MdbxDb::open(db_path.to_str().unwrap()).unwrap();

    let notifier = Arc::new(EntityEventNotifier::new());
    let (block_event_sender, _) = tokio::sync::broadcast::channel(1024);
    let api = RethEntityApiImpl::new(db_conn.into_inner(), notifier, block_event_sender.clone());

    let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().unwrap();
    let handle = server.start(api.into_rpc());
    tokio::spawn(handle.stopped());
    (addr, block_event_sender)
}

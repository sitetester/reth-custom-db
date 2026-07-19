pub mod block_events;
pub mod notifier;
pub mod reth;
pub mod sqlite;

use jsonrpsee::{PendingSubscriptionSink, SubscriptionMessage, SubscriptionSink};
use serde::Serialize;

use crate::db::EntityEvent;

/// Creates a JSON-RPC internal error response using code `-32000` -
/// [spec](https://www.jsonrpc.org/specification#error_object).
/// ```json
/// {
///     "jsonrpc": "2.0",
///     "id": 1,
///     "error": {
///         "code": -32000,
///         "message": "<error>",
///         "data": null
///     }
/// }
/// ```
fn internal_error(msg: String) -> jsonrpsee::types::error::ErrorObject<'static> {
    jsonrpsee::types::error::ErrorObject::owned(-32000, msg, None::<()>)
}

async fn try_send_event<T: Serialize>(sink: &SubscriptionSink, event: &T) -> bool {
    let raw = match serde_json::value::to_raw_value(event) {
        Ok(raw) => raw,
        Err(err) => {
            eprintln!("failed to serialize event: {err}");
            return false;
        }
    };
    let msg = SubscriptionMessage::from(raw);
    sink.send(msg).await.is_ok()
}

/// Spawns a background task that forwards [`EntityEvent`]s from a broadcast channel
/// to a JSON-RPC subscription client.
///
/// In jsonrpsee, a client subscription has two phases:
/// - Pending - client requested subscription, waiting for server to accept/reject
/// - Accepted - server confirmed, now has a SubscriptionSink to push events
///
/// The task accepts the pending subscription, then loops over incoming events,
/// optionally filtering by `key_filter`. It exits when:
/// - The client disconnects or unsubscribes (send fails).
/// - The broadcast channel is closed (all senders dropped).
fn spawn_event_subscription(
    pending: PendingSubscriptionSink,
    mut receiver: tokio::sync::broadcast::Receiver<EntityEvent>,
    key_filter: Option<String>,
) {
    tokio::spawn(async move {
        let sink = match pending.accept().await {
            Ok(sink) => sink,
            Err(_) => return,
        };
        while let Ok(ref event) = receiver.recv().await {
            if let Some(ref filter) = key_filter
                && &event.key != filter
            {
                continue;
            }
            if !try_send_event(&sink, event).await {
                break;
            }
        }
    });
}

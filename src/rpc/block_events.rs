use futures_util::StreamExt;
use reth_ethereum::provider::CanonStateNotification;
use reth_ethereum::provider::CanonStateNotificationStream;

use crate::db::BlockEvent;

/// Forwards canonical state notifications (new blocks / reorgs) into a broadcast channel
/// as [`BlockEvent`]s for RPC subscribers.
pub fn forward_block_events(
    mut notifications: CanonStateNotificationStream,
    executor: &reth_ethereum::tasks::TaskExecutor,
    block_event_sender: tokio::sync::broadcast::Sender<BlockEvent>,
) {
    // Only used at call site. Appears in reth runtime logs if the task panics.
    executor.spawn_critical_task("block-forwarder", async move {
        while let Some(notification) = notifications.next().await {
            let event = block_event_from_notification(&notification);
            let _ = block_event_sender.send(event); // discard error if no subscribers
        }
    });
}

/// Extracts a [`BlockEvent`] from a canonical state notification.
fn block_event_from_notification(notification: &CanonStateNotification) -> BlockEvent {
    let chain = match notification {
        CanonStateNotification::Commit { new } => new,
        CanonStateNotification::Reorg { new, .. } => new,
    };
    // .tip() returns the head of the canonical chain - the block that was just
    // committed (on Commit) or that became the new tip (on Reorg).
    let block = chain.tip();
    let header = block.clone_header(); // block header
    BlockEvent {
        block_number: header.number,
        block_hash: header.hash_slow().to_string(),
        parent_hash: header.parent_hash.to_string(),
        timestamp: header.timestamp,
        tx_count: block.transactions_recovered().count(),
    }
}

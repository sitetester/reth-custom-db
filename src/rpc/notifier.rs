use tokio::sync::broadcast;

use crate::db::EntityEvent;

/// Broadcasts entity events (saved/deleted) to WebSocket subscribers.
pub struct EntityEventNotifier {
    tx: broadcast::Sender<EntityEvent>,
}

impl EntityEventNotifier {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(256);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EntityEvent> {
        self.tx.subscribe()
    }

    /// Called after save and delete operations in both backends:
    /// sqlite.rs — after sqlite_save and sqlite_delete RPC calls
    /// reth.rs — after reth_save and reth_delete RPC calls
    /// It broadcasts the entity event so WebSocket subscribers get notified in real-time.
    pub fn notify(&self, event: EntityEvent) {
        let _ = self.tx.send(event);
    }
}

/// Callers can use either EntityEventNotifier::new() or EntityEventNotifier::default().
impl Default for EntityEventNotifier {
    fn default() -> Self {
        Self::new()
    }
}

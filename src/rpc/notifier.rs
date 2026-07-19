use tokio::sync::broadcast;

use crate::db::EntityEvent;

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

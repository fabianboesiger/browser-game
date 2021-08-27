use tokio_util::time::{DelayQueue, delay_queue::Key};
use chrono::{DateTime, Utc};

struct Event {
    date: DateTime<Utc>,
    id: i64,
    key: Option<Key>,
}

struct Queue {
    queue: DelayQueue<i64>
}

impl Queue {
    pub async fn insert(&mut self, event: &mut Event) {
        //let key = self.queue.insert(event.id, );
    }
}
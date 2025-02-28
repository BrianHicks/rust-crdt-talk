use chrono::{DateTime, Utc};
use std::cmp::{Ord, Ordering};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, Eq)]
pub struct HybridLogicalClock {
    timestamp: DateTime<Utc>,
    counter: u64,
    node_id: Uuid,
}

impl HybridLogicalClock {
    pub fn new(node_id: Uuid) -> Self {
        HybridLogicalClock {
            timestamp: Utc::now(),
            counter: 0,
            node_id,
        }
    }

    pub fn tick(&mut self) {
        let now = Utc::now();
        if now > self.timestamp {
            self.timestamp = now;
            self.counter = 0;
        } else {
            self.counter += 1;
        }
    }
}

impl Ord for HybridLogicalClock {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then(self.counter.cmp(&other.counter))
            .then(self.node_id.cmp(&other.node_id))
    }
}

impl PartialEq for HybridLogicalClock {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.counter == other.counter
            && self.node_id == other.node_id
    }
}

impl fmt::Display for HybridLogicalClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}::{}", self.timestamp, self.counter, self.node_id)
    }
}

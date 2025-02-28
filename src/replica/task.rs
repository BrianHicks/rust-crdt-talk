use crate::crdt::{hlc::HybridLogicalClock, LWWRegister, Merge};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub added: LWWRegister<DateTime<Utc>>,
    pub complete: LWWRegister<bool>,
    pub description: LWWRegister<String>,
}

impl Task {
    pub fn highest_clock(&self) -> &HybridLogicalClock {
        self.added
            .clock()
            .max(self.complete.clock())
            .max(self.description.clock())
    }

    pub fn new(description: String, when: HybridLogicalClock) -> Self {
        Self {
            added: LWWRegister::new(Utc::now(), when.clone()),
            complete: LWWRegister::new(false, when.clone()),
            description: LWWRegister::new(description, when.clone()),
        }
    }
}

impl Merge for Task {
    fn merge_mut(&mut self, other: Self) {
        self.added.merge_mut(other.added);
        self.complete.merge_mut(other.complete);
        self.description.merge_mut(other.description);
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if *self.complete.value() { "[x]" } else { "[ ]" };
        write!(f, "{} {}", status, self.description.value())
    }
}

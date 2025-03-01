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
    #[tracing::instrument(name = "Task::new", skip(when))]
    pub fn new(description: String, when: HybridLogicalClock) -> Self {
        Self {
            added: LWWRegister::new(Utc::now(), when),
            complete: LWWRegister::new(false, when),
            description: LWWRegister::new(description, when),
        }
    }
}

impl Merge for Task {
    #[tracing::instrument(name = "Task::merge_mut", skip(self, other))]
    fn merge_mut(&mut self, other: Self) {
        self.added.merge_mut(other.added);
        self.complete.merge_mut(other.complete);
        self.description.merge_mut(other.description);
    }
}

impl fmt::Display for Task {
    #[tracing::instrument(name = "Task::fmt", skip(self, f))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if *self.complete.value() { "[x]" } else { "[ ]" };
        write!(f, "{} {}", status, self.description.value())
    }
}

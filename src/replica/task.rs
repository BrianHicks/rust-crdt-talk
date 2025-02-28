use crate::crdt::{LWWRegister, Merge};
use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub added: LWWRegister<DateTime<Utc>>,
    pub complete: LWWRegister<bool>,
    pub description: LWWRegister<String>,
}

impl Merge for Task {
    fn merge_mut(&mut self, other: Self) {
        self.added.merge_mut(other.added);
        self.complete.merge_mut(other.complete);
        self.description.merge_mut(other.description);
    }
}

use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if *self.complete.value() { "[x]" } else { "[ ]" };
        write!(f, "{} {}", status, self.description.value())
    }
}

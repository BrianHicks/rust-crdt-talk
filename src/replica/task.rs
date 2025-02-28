use crate::crdt::{LWWRegister, Merge};
use chrono::{DateTime, Utc};

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

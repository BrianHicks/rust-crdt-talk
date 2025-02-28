mod task;

use crate::crdt::{GMap, Merge};
use task::Task;
use uuid::Uuid;

pub struct Replica {
    tasks: GMap<Uuid, Task>,
}

impl Merge for Replica {
    fn merge_mut(&mut self, other: Self) {
        self.tasks.merge_mut(other.tasks);
    }
}

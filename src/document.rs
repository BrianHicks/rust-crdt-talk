mod task;

use crate::crdt::{GMap, HybridLogicalClock, Merge};
use itertools::Itertools;
pub use task::Task;
use uuid::Uuid;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub tasks: GMap<Uuid, Task>,
}

impl Document {
    pub fn tasks(&self) -> impl Iterator<Item = (usize, &Task)> {
        self.tasks
            .iter()
            .map(|t| t.1)
            .sorted_by_key(|task| task.added.value())
            .enumerate()
    }

    pub fn add_task(&mut self, description: String, when: HybridLogicalClock) -> Uuid {
        let id = Uuid::new_v4();

        self.tasks.insert(id, Task::new(description, when));

        id
    }
}

impl Merge for Document {
    fn merge_mut(&mut self, other: Self) {
        self.tasks.merge_mut(other.tasks);
    }
}

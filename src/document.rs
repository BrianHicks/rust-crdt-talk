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
    pub fn tasks(&self) -> impl Iterator<Item = (&Uuid, &Task)> {
        self.tasks
            .iter()
            .sorted_by_key(|(_, task)| task.added.value())
    }

    pub fn add_task(&mut self, description: String, when: HybridLogicalClock) -> Uuid {
        let id = Uuid::new_v4();

        self.tasks.insert(id, Task::new(description, when));

        id
    }

    pub fn update_task_description(
        &mut self,
        id: &Uuid,
        description: String,
        clock: HybridLogicalClock,
    ) -> bool {
        if let Some(task) = self.tasks.get_mut(id) {
            task.description.set(description, clock);

            true
        } else {
            false
        }
    }
}

impl Merge for Document {
    fn merge_mut(&mut self, other: Self) {
        self.tasks.merge_mut(other.tasks);
    }
}

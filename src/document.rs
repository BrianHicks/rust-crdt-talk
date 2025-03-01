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
    #[tracing::instrument(name = "Document::tasks", skip(self))]
    pub fn tasks(&self) -> impl Iterator<Item = (&Uuid, &Task)> {
        self.tasks
            .iter()
            .sorted_by_cached_key(|(_, task)| task.added.value())
    }

    #[tracing::instrument(name = "Document::add_task", skip(self, clock))]
    pub fn add_task(&mut self, description: String, clock: HybridLogicalClock) -> Uuid {
        let id = Uuid::new_v4();

        self.tasks.insert(id, Task::new(description, clock));

        id
    }

    #[tracing::instrument(name = "Document::update_task_description", skip(self, id, clock))]
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

    #[tracing::instrument(name = "Document::complete_task", skip(self, id, clock))]
    pub fn complete_task(&mut self, id: &Uuid, clock: HybridLogicalClock) -> bool {
        if let Some(task) = self.tasks.get_mut(id) {
            task.complete.set(true, clock);

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

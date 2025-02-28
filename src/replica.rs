mod task;

use crate::crdt::{hlc::HybridLogicalClock, GMap, Merge};
use task::Task;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Replica {
    pub id: Uuid,
    pub tasks: GMap<Uuid, Task>,
}

impl Replica {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            tasks: GMap::default(),
        }
    }

    pub fn add_task(&mut self, description: String) -> Uuid {
        let id = Uuid::new_v4();

        self.tasks
            .insert(id, Task::new(description, self.next_clock()));

        id
    }

    fn next_clock(&self) -> HybridLogicalClock {
        let existing_clock = self
            .tasks
            .iter()
            .map(|(_, task)| task.highest_clock())
            .max();

        let next_clock = match existing_clock {
            Some(clock) => HybridLogicalClock::new(self.id).max(clock.next()),
            None => HybridLogicalClock::new(self.id),
        };

        next_clock.claim(self.id);

        next_clock
    }
}

impl Merge for Replica {
    fn merge_mut(&mut self, other: Self) {
        self.tasks.merge_mut(other.tasks);
    }
}

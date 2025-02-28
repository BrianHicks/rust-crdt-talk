mod task;

use crate::crdt::{hlc::HybridLogicalClock, GMap, Merge};
use task::Task;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Replica {
    pub id: Uuid,
    pub clock: HybridLogicalClock,
    pub tasks: GMap<Uuid, Task>,
}

impl Replica {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let clock = HybridLogicalClock::new(id);

        Self {
            id,
            clock,
            tasks: GMap::default(),
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn add_task(&mut self, description: String) -> Uuid {
        let id = Uuid::new_v4();
        let next_clock = self.next_clock();

        self.tasks.insert(id, Task::new(description, next_clock));

        id
    }

    #[tracing::instrument(skip(self))]
    fn next_clock(&mut self) -> HybridLogicalClock {
        self.clock.tick();

        self.clock
    }
}

impl Merge for Replica {
    fn merge_mut(&mut self, other: Self) {
        self.tasks.merge_mut(other.tasks);
    }
}

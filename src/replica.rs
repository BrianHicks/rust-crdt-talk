use crate::crdt::HybridLogicalClock;
use crate::document::{Document, Task};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Replica {
    id: Uuid,
    clock: HybridLogicalClock,
    document: Document,
}

impl Replica {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let clock = HybridLogicalClock::new(id);

        Self {
            id,
            clock,
            document: Document::default(),
        }
    }

    pub fn tasks(&self) -> impl Iterator<Item = (usize, &Task)> {
        self.document.tasks()
    }

    #[tracing::instrument(skip(self))]
    pub fn add_task(&mut self, description: String) -> Uuid {
        let clock = self.next_clock();

        self.document.add_task(description, clock)
    }

    #[tracing::instrument(skip(self))]
    fn next_clock(&mut self) -> HybridLogicalClock {
        self.clock.tick();

        self.clock
    }
}

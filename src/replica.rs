use crate::crdt::{HybridLogicalClock, Merge};
use crate::document::{Document, Task};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Replica {
    id: Uuid,
    clock: HybridLogicalClock,
    document: Document,
}

impl Replica {
    #[tracing::instrument(name = "Replica::new")]
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let clock = HybridLogicalClock::new(id);

        Self {
            id,
            clock,
            document: Document::default(),
        }
    }

    #[tracing::instrument(name = "Replica::tasks", skip(self))]
    pub fn tasks(&self) -> impl Iterator<Item = (&Uuid, &Task)> {
        self.document.tasks()
    }

    #[tracing::instrument(name = "Replica::add_task", skip(self))]
    pub fn add_task(&mut self, description: String) -> Uuid {
        let clock = self.next_clock();

        self.document.add_task(description, clock)
    }

    #[tracing::instrument(name = "Replica::next_clock", skip(self))]
    fn next_clock(&mut self) -> HybridLogicalClock {
        self.clock.tick();

        self.clock
    }

    #[tracing::instrument(name = "Replica::update_task_description", skip(self))]
    pub fn update_task_description(&mut self, id: &Uuid, description: String) -> bool {
        let clock = self.next_clock();

        self.document
            .update_task_description(id, description, clock)
    }

    #[tracing::instrument(name = "Replica::complete_task", skip(self))]
    pub fn complete_task(&mut self, id: &Uuid) -> bool {
        let clock = self.next_clock();

        self.document.complete_task(id, clock)
    }

    pub fn archive_completed_tasks(&mut self) {
        self.document.archive_completed_tasks()
    }

    pub fn merge(&mut self, other: Replica) {
        self.document.merge_mut(other.document);
    }
}

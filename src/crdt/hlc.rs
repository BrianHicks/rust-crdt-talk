use chrono::{DateTime, Utc};
use std::cmp::{Ord, Ordering};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct HybridLogicalClock {
    #[cfg_attr(test, proptest(strategy = "timestamp_strategy()"))]
    timestamp: DateTime<Utc>,

    #[cfg_attr(test, proptest(strategy = "counter_strategy()"))]
    counter: u16,

    #[cfg_attr(test, proptest(strategy = "uuid_strategy()"))]
    node_id: Uuid,
}

#[cfg(test)]
fn timestamp_strategy() -> impl proptest::strategy::Strategy<Value = DateTime<Utc>> {
    use chrono::TimeZone;
    use proptest::prelude::*;

    (1_700_000_000..1_800_000_000_000i64).prop_map(|unix| Utc.timestamp_opt(unix, 0).unwrap())
}

#[cfg(test)]
fn counter_strategy() -> impl proptest::strategy::Strategy<Value = u16> {
    0..=2u16
}

#[cfg(test)]
fn uuid_strategy() -> impl proptest::strategy::Strategy<Value = Uuid> {
    use proptest::prelude::*;

    any::<u128>().prop_map(Uuid::from_u128)
}

impl HybridLogicalClock {
    pub fn new(node_id: Uuid) -> Self {
        HybridLogicalClock {
            timestamp: Utc::now(),
            counter: 0,
            node_id,
        }
    }

    pub fn tick(&mut self) {
        let now = Utc::now();
        if now > self.timestamp {
            self.timestamp = now;
            self.counter = 0;
        } else {
            self.counter += 1;
        }
    }
}

impl Ord for HybridLogicalClock {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then(self.counter.cmp(&other.counter))
            .then(self.node_id.cmp(&other.node_id))
    }
}

impl PartialOrd for HybridLogicalClock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for HybridLogicalClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}::{}", self.timestamp, self.counter, self.node_id)
    }
}

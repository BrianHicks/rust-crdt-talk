use chrono::{DateTime, Utc};
use std::cmp::{Ord, Ordering};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

    (0..=2i64).prop_map(|unix| Utc.timestamp_opt(unix, 0).unwrap())
}

#[cfg(test)]
fn counter_strategy() -> impl proptest::strategy::Strategy<Value = u16> {
    0..=2u16
}

#[cfg(test)]
fn uuid_strategy() -> impl proptest::strategy::Strategy<Value = Uuid> {
    use proptest::prelude::*;

    (0..=2u128).prop_map(Uuid::from_u128)
}

impl HybridLogicalClock {
    #[tracing::instrument(name = "HLC::new")]
    pub fn new(node_id: Uuid) -> Self {
        HybridLogicalClock {
            timestamp: Utc::now(),
            counter: 0,
            node_id,
        }
    }

    #[tracing::instrument(name = "HLC::tick", skip(self))]
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
    #[tracing::instrument(name = "HLC::cmp", skip(self))]
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

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn clocks_are_equal_if_components_are_equal(
            timestamp in timestamp_strategy(),
            counter in counter_strategy(),
            node_id in uuid_strategy(),
        ) {
            let clock = HybridLogicalClock {
                timestamp, counter, node_id
            };

            assert_eq!(clock, clock);
        }
    }

    proptest! {
        #[test]
        fn cmp_timestamps_first(
            timestamp_a in timestamp_strategy(),
            timestamp_b in timestamp_strategy(),
            counter in counter_strategy(),
            node_id in uuid_strategy(),
        ) {
            let greater = HybridLogicalClock {
                timestamp: timestamp_a.max(timestamp_b),
                counter,
                node_id,
            };

            let lesser = HybridLogicalClock {
                timestamp: timestamp_a.min(timestamp_b),
                counter,
                node_id,
            };

            assert!(greater >= lesser, "{greater:?} < {lesser:?}");
        }
    }

    proptest! {
        #[test]
        fn cmp_counters_second(
            timestamp in timestamp_strategy(),
            counter_a in counter_strategy(),
            counter_b in counter_strategy(),
            node_id in uuid_strategy(),
        ) {
            let greater = HybridLogicalClock {
                timestamp,
                counter: counter_a.max(counter_b),
                node_id,
            };

            let lesser = HybridLogicalClock {
                timestamp,
                counter: counter_a.min(counter_b),
                node_id,
            };

            assert!(greater >= lesser, "{greater:?} < {lesser:?}");
        }
    }

    proptest! {
        #[test]
        fn cmp_node_ids_third(
            timestamp in timestamp_strategy(),
            counter in counter_strategy(),
            node_id_a in uuid_strategy(),
            node_id_b in uuid_strategy(),
        ) {
            let greater = HybridLogicalClock {
                timestamp,
                counter,
                node_id: node_id_a.max(node_id_b),
            };

            let lesser = HybridLogicalClock {
                timestamp,
                counter,
                node_id: node_id_a.min(node_id_b),
            };

            assert!(greater >= lesser, "{greater:?} < {lesser:?}");
        }
    }
}

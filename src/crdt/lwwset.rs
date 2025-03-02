use super::{HybridLogicalClock, Merge};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct LWWSet<T: Ord> {
    adds: BTreeMap<T, HybridLogicalClock>,
    removes: BTreeMap<T, HybridLogicalClock>,
}

impl<T: Ord> LWWSet<T> {
    #[tracing::instrument(name = "LWWSet::insert", skip(self, item, clock))]
    pub fn insert(&mut self, item: T, clock: HybridLogicalClock) {
        if Some(&clock) > self.adds.get(&item) {
            self.adds.insert(item, clock);
        }
    }

    #[tracing::instrument(name = "LWWSet::remove", skip(self, item, clock))]
    pub fn remove(&mut self, item: T, clock: HybridLogicalClock) {
        if Some(&clock) > self.removes.get(&item) {
            self.removes.insert(item, clock);
        }
    }

    #[tracing::instrument(name = "LWWSet::contains", skip(self, item))]
    pub fn contains(&self, item: &T) -> bool {
        self.adds.get(item) > self.removes.get(item)
    }
}

impl<T: Ord> Merge for LWWSet<T> {
    #[tracing::instrument(name = "LWWSet::merge_mut", skip(self, other))]
    fn merge_mut(&mut self, other: Self) {
        for (item, clock) in other.adds {
            if self.adds.get(&item) < Some(&clock) {
                self.adds.insert(item, clock);
            }
        }

        for (item, clock) in other.removes {
            if self.removes.get(&item) < Some(&clock) {
                self.removes.insert(item, clock);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::merge;
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_idempotent(v: LWWSet<bool>) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn test_commutative(a: LWWSet<bool>, b: LWWSet<bool>) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn test_associative(a: LWWSet<bool>, b: LWWSet<bool>, c: LWWSet<bool>) {
            merge::test_associative(a, b, c);
        }
    }
}

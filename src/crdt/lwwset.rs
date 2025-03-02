use super::{HybridLogicalClock, Merge};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct LWWSet<T: Ord> {
    adds: BTreeMap<T, HybridLogicalClock>,
    removes: BTreeMap<T, HybridLogicalClock>,
}

impl<T: Ord> LWWSet<T> {
    pub fn contains(&self, item: &T) -> bool {
        self.adds.get(item) > self.removes.get(item)
    }
}

impl<T: Ord> Merge for LWWSet<T> {
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

use crate::merge::Merge;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct GMap<T: Merge>(HashMap<String, T>);

impl<T: Merge> Merge for GMap<T> {
    fn merge_mut(&mut self, other: Self) {
        for (key, value) in other.0 {
            match self.0.entry(key) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().merge_mut(value);
                }
                Entry::Vacant(entry) => {
                    entry.insert(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gset::GSet;
    use crate::merge::{test_associative, test_commutative, test_idempotent};
    use proptest::proptest;

    proptest! {
        #[test]
        fn merge_idempotent(v: GMap<GSet<bool>>) {
            test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn merge_commutative(a: GMap<GSet<bool>>, b: GMap<GSet<bool>>) {
            test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn merge_associative(a: GMap<GSet<bool>>, b: GMap<GSet<bool>>, c: GMap<GSet<bool>>) {
            test_associative(a, b, c);
        }
    }
}

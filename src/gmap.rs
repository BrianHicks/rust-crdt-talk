use crate::merge::Merge;
use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct GMap<K, V>(HashMap<K, V>)
where
    K: Hash + Eq,
    V: Merge;

impl<K, V> Merge for GMap<K, V>
where
    K: Hash + Eq,
    V: Merge,
{
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
    use crate::lww::LWWRegister;
    use proptest::proptest;

    proptest! {
        #[test]
        fn merge_idempotent(v: GMap<bool, LWWRegister<bool>>) {
            crate::merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn merge_commutative(a: GMap<bool, LWWRegister<bool>>, b: GMap<bool, LWWRegister<bool>>) {
            crate::merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn merge_associative(a: GMap<bool, LWWRegister<bool>>, b: GMap<bool, LWWRegister<bool>>, c: GMap<bool, LWWRegister<bool>>) {
            crate::merge::test_associative(a, b, c);
        }
    }
}

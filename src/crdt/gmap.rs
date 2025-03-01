use super::merge::Merge;
use std::collections::{btree_map::Entry, BTreeMap};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct GMap<K, V>(BTreeMap<K, V>)
where
    K: Hash + Ord,
    V: Merge;

impl<K, V> GMap<K, V>
where
    K: Hash + Ord,
    V: Merge,
{
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0.iter()
    }

    #[tracing::instrument(skip(self, key, value))]
    pub fn insert(&mut self, key: K, value: V) {
        match self.0.entry(key) {
            Entry::Occupied(mut existing) => {
                existing.get_mut().merge_mut(value);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(value);
            }
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(&key)
    }
}

impl<K, V> Merge for GMap<K, V>
where
    K: Hash + Ord,
    V: Merge,
{
    fn merge_mut(&mut self, other: Self) {
        for (key, value) in other.0 {
            self.insert(key, value);
        }
    }
}

impl<K, V> Default for GMap<K, V>
where
    K: Hash + Ord,
    V: Merge,
{
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

#[cfg(test)]
mod test {
    use super::super::max::Max;
    use super::super::merge;
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn merge_idempotent(v: GMap<bool, Max<bool>>) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn merge_commutative(a: GMap<bool, Max<bool>>, b: GMap<bool, Max<bool>>) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn merge_associative(a: GMap<bool, Max<bool>>, b: GMap<bool, Max<bool>>, c: GMap<bool, Max<bool>>) {
            merge::test_associative(a, b, c);
        }
    }
}

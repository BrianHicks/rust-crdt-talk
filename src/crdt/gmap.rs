use super::merge::Merge;
use std::collections::{BTreeMap, btree_map::Entry};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct GMap<K: Hash + Ord, V: Merge>(BTreeMap<K, V>);

impl<K: Hash + Ord, V: Merge> GMap<K, V> {
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0.iter()
    }

    #[tracing::instrument(name = "GMap::insert", skip(self, key, value))]
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

    #[tracing::instrument(name = "GMap::get_mut", skip(self, key))]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }
}

impl<K: Hash + Ord, V: Merge> Merge for GMap<K, V> {
    #[tracing::instrument(name = "GMap::merge_mut", skip(self, other))]
    fn merge_mut(&mut self, other: Self) {
        for (key, value) in other.0 {
            self.insert(key, value);
        }
    }
}

impl<K: Hash + Ord, V: Merge> Default for GMap<K, V> {
    #[tracing::instrument(name = "GMap::default")]
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

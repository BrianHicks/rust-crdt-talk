use super::{HybridLogicalClock, LWWSet, Merge};
use std::collections::{btree_map::Entry, BTreeMap};

#[derive(Debug)]
pub struct LWWMap<K, V>
where
    K: Ord,
    V: Merge,
{
    keys: LWWSet<K>,
    values: BTreeMap<K, V>,
}

impl<K, V> LWWMap<K, V>
where
    K: Ord + Clone,
    V: Merge,
{
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.values.iter()
    }

    #[tracing::instrument(name = "LWWMap::insert", skip(self, key, value, clock))]
    pub fn insert(&mut self, key: K, value: V, clock: HybridLogicalClock) {
        self.keys.insert(key.clone(), clock);
        self.insert_value(key, value);
    }

    fn insert_value(&mut self, key: K, value: V) {
        match self.values.entry(key) {
            Entry::Occupied(mut existing) => {
                existing.get_mut().merge_mut(value);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(value);
            }
        }
    }

    #[tracing::instrument(name = "LWWMap::remove", skip(self, key, clock))]
    pub fn remove(&mut self, key: K, clock: HybridLogicalClock) {
        self.values.remove(&key);
        self.keys.remove(key, clock);
    }
}

impl<K, V> Default for LWWMap<K, V>
where
    K: Ord,
    V: Merge,
{
    fn default() -> Self {
        LWWMap {
            keys: LWWSet::default(),
            values: BTreeMap::default(),
        }
    }
}

impl<K, V> Merge for LWWMap<K, V>
where
    K: Ord + Clone,
    V: Merge,
{
    fn merge_mut(&mut self, other: Self) {
        self.keys.merge_mut(other.keys);

        // remove any values with newly-missing keys
        self.values.retain(|key, _| self.keys.contains(key));

        // add and merge any new values
        for (key, value) in other.values {
            self.insert_value(key, value);
        }
    }
}

use super::{HybridLogicalClock, LWWSet, Merge};
use std::collections::{btree_map::Entry, BTreeMap};

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[cfg(test)]
pub fn lwwmap_strategy<K, V>(
    k_strat: impl proptest::strategy::Strategy<Value = K> + 'static,
    v_strat: impl proptest::strategy::Strategy<Value = V> + 'static,
) -> impl proptest::strategy::Strategy<Value = LWWMap<K, V>>
where
    K: Ord + Clone + std::fmt::Debug,
    V: Merge + std::fmt::Debug,
{
    use proptest::collection::vec;
    use proptest::prelude::*;

    vec(
        (
            k_strat,
            v_strat,
            any::<HybridLogicalClock>(),
            any::<Option<HybridLogicalClock>>(),
        ),
        0..2,
    )
    .prop_map(|inserts| {
        let mut map = LWWMap::default();

        for (key, value, clock, removed_at_opt) in inserts {
            map.insert(key.clone(), value, clock);

            if let Some(removed_at) = removed_at_opt {
                map.remove(key.clone(), removed_at);
            }
        }

        map
    })
    .boxed()
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

#[cfg(test)]
mod test {
    use super::super::merge;
    use super::*;
    use crate::crdt::max::Max;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_idempotent(v in lwwmap_strategy(any::<bool>(), any::<Max<bool>>())) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn test_commutative(a in lwwmap_strategy(any::<bool>(), any::<Max<bool>>()), b in lwwmap_strategy(any::<bool>(), any::<Max<bool>>())) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn test_associative(a in lwwmap_strategy(any::<bool>(), any::<Max<bool>>()), b in lwwmap_strategy(any::<bool>(), any::<Max<bool>>()), c in lwwmap_strategy(any::<bool>(), any::<Max<bool>>())) {
            merge::test_associative(a, b, c);
        }
    }
}

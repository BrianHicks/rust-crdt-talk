#[cfg(test)]
use proptest::arbitrary::{Arbitrary, ParamsFor, StrategyFor};

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
            if self.keys.contains(&key) {
                self.insert_value(key, value);
            }
        }
    }
}

#[cfg(test)]
impl<K, V> Arbitrary for LWWMap<K, V>
where
    K: Ord + std::fmt::Debug + Clone + Arbitrary,
    V: Merge + std::fmt::Debug + Arbitrary,
{
    type Parameters = (
        ParamsFor<K>,
        ParamsFor<V>,
        ParamsFor<HybridLogicalClock>,
        ParamsFor<Option<HybridLogicalClock>>,
    );

    type Strategy = proptest::strategy::Map<
        StrategyFor<Vec<(K, V, HybridLogicalClock, Option<HybridLogicalClock>)>>,
        fn(Vec<(K, V, HybridLogicalClock, Option<HybridLogicalClock>)>) -> Self,
    >;

    fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
        use proptest::collection::vec;
        use proptest::prelude::*;

        let (k_param, v_param, add_param, remove_param) = params;

        proptest::strategy::Strategy::prop_map(
            vec(
                (
                    any_with::<K>(k_param),
                    any_with::<V>(v_param),
                    any_with::<HybridLogicalClock>(add_param),
                    any_with::<Option<HybridLogicalClock>>(remove_param),
                ),
                1..4,
            ),
            |items| {
                let mut map = LWWMap::default();

                for (key, value, clock, removed_at_opt) in items {
                    map.insert(key.clone(), value, clock);

                    if let Some(removed_at) = removed_at_opt {
                        map.remove(key.clone(), removed_at);
                    }
                }

                map
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::super::merge;
    use super::*;
    use crate::crdt::max::Max;
    use proptest::prelude::*;
    use uuid::Uuid;

    proptest! {
        #[test]
        fn removing_removes_value(mut base: LWWMap<bool, Max<bool>>, k: bool, v: bool) {
            let mut clock = HybridLogicalClock::new(Uuid::nil());

            base.insert(k, v.into(), clock.clone());

            clock.tick();
            base.remove(k, clock);

            assert!(!base.keys.contains(&k));
            assert!(!base.values.contains_key(&k));
        }
    }

    proptest! {
        #[test]
        fn merge_idempotent(v: LWWMap<bool, Max<bool>>) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn merge_commutative(a: LWWMap<bool, Max<bool>>, b: LWWMap<bool, Max<bool>>) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn merge_associative(a: LWWMap<bool, Max<bool>>, b: LWWMap<bool, Max<bool>>, c: LWWMap<bool, Max<bool>>) {
            merge::test_associative(a, b, c);
        }
    }
}

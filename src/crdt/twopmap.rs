use super::Merge;
use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

#[cfg(test)]
use proptest::arbitrary::{Arbitrary, ParamsFor, StrategyFor};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TwoPMap<K, V>
where
    K: Ord,
    V: Merge,
{
    adds: BTreeMap<K, V>,
    removes: BTreeSet<K>,
}

impl<K, V> TwoPMap<K, V>
where
    K: Ord,
    V: Merge,
{
    pub fn insert(&mut self, key: K, value: V) {
        if self.removes.contains(&key) {
            return;
        }

        match self.adds.entry(key) {
            Entry::Occupied(mut existing) => {
                existing.get_mut().merge_mut(value);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(value);
            }
        }
    }

    pub fn remove(&mut self, key: K) {
        self.adds.remove(&key);
        self.removes.insert(key);
    }
}

impl<K, V> Merge for TwoPMap<K, V>
where
    K: Ord,
    V: Merge,
{
    fn merge_mut(&mut self, mut other: Self) {
        self.removes.append(&mut other.removes);

        for (key, value) in other.adds {
            self.insert(key, value);
        }
        self.adds.retain(|k, _| !self.removes.contains(k))
    }
}

impl<K, V> Default for TwoPMap<K, V>
where
    K: Ord,
    V: Merge,
{
    fn default() -> Self {
        TwoPMap {
            adds: BTreeMap::default(),
            removes: BTreeSet::default(),
        }
    }
}

#[cfg(test)]
impl<K, V> Arbitrary for TwoPMap<K, V>
where
    K: Ord + std::fmt::Debug + Clone + Arbitrary,
    V: Merge + std::fmt::Debug + Arbitrary,
{
    type Parameters = (ParamsFor<K>, ParamsFor<V>);

    type Strategy =
        proptest::strategy::Map<StrategyFor<Vec<(K, V, bool)>>, fn(Vec<(K, V, bool)>) -> Self>;

    fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
        use proptest::collection::vec;
        use proptest::prelude::*;

        let (k_param, v_param) = params;

        proptest::strategy::Strategy::prop_map(
            vec(
                (
                    any_with::<K>(k_param),
                    any_with::<V>(v_param),
                    any::<bool>(),
                ),
                1..4,
            ),
            |items| {
                let mut map = Self::default();

                for (key, value, removed) in items {
                    map.insert(key.clone(), value);

                    if removed {
                        map.remove(key);
                    }
                }

                map
            },
        )
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
        fn merge_idempotent(v: TwoPMap<bool, Max<bool>>) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn merge_commutative(a: TwoPMap<bool, Max<bool>>, b: TwoPMap<bool, Max<bool>>) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn merge_associative(a: TwoPMap<bool, Max<bool>>, b: TwoPMap<bool, Max<bool>>, c: TwoPMap<bool, Max<bool>>) {
            merge::test_associative(a, b, c);
        }
    }
}

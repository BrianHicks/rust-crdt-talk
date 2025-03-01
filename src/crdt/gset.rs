use super::merge::Merge;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct GSet<T: Eq + Ord>(BTreeSet<T>);

impl<T: Eq + Ord> GSet<T> {
    pub fn insert(&mut self, item: T) {
        self.0.insert(item);
    }
}

impl<T: Eq + Ord> Merge for GSet<T> {
    fn merge_mut(&mut self, other: Self) {
        for item in other.0 {
            self.insert(item)
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::merge;
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn merge_idempotent(v: GSet<bool>) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn merge_commutative(a: GSet<bool>, b: GSet<bool>) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn merge_associative(a: GSet<bool>, b: GSet<bool>, c: GSet<bool>) {
            merge::test_associative(a, b, c);
        }
    }
}

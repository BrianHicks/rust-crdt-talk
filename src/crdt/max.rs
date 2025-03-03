use super::Merge;
use proptest_derive::Arbitrary;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Arbitrary)]
pub struct Max<T: Ord>(T);

impl<T: Ord> Merge for Max<T> {
    fn merge_mut(&mut self, other: Self) {
        if other.0 > self.0 {
            self.0 = other.0;
        }
    }
}

impl<T> From<T> for Max<T>
where
    T: Ord,
{
    fn from(t: T) -> Self {
        Max(t)
    }
}

#[cfg(test)]
mod test {
    use super::super::merge;
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn merge_idempotent(v: Max<bool>) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn merge_commutative(a: Max<bool>, b: Max<bool>) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn merge_associative(a: Max<bool>, b: Max<bool>, c: Max<bool>) {
            merge::test_associative(a, b, c);
        }
    }
}

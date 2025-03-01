use super::hlc::HybridLogicalClock;
use super::merge::Merge;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct LWWRegister<T> {
    value: T,
    clock: HybridLogicalClock,
}

impl<T> LWWRegister<T> {
    pub fn new(value: T, timestamp: HybridLogicalClock) -> Self {
        LWWRegister {
            value,
            clock: timestamp,
        }
    }

    #[tracing::instrument(skip(self, value, timestamp))]
    pub fn set(&mut self, value: T, timestamp: HybridLogicalClock) {
        self.value = value;
        self.clock = timestamp;
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    #[cfg(test)]
    pub fn clock(&self) -> &HybridLogicalClock {
        &self.clock
    }
}

impl<T> Merge for LWWRegister<T>
where
    T: Clone + Ord,
{
    fn merge_mut(&mut self, other: Self) {
        if other.clock > self.clock {
            self.set(other.value, other.clock)
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::merge;
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_idempotent(v: LWWRegister<bool>) {
            merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn test_commutative(a: LWWRegister<bool>, b: LWWRegister<bool>) {
            prop_assume!(a.clock() != b.clock());

            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn test_associative(a: LWWRegister<bool>, b: LWWRegister<bool>, c: LWWRegister<bool>) {
            prop_assume!(a.clock() != b.clock());
            prop_assume!(a.clock() != c.clock());
            prop_assume!(b.clock() != c.clock());

            merge::test_associative(a, b, c);
        }
    }
}

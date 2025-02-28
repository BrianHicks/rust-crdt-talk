use super::hlc::HybridLogicalClock;
use super::merge::Merge;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct LWWRegister<T> {
    value: T,
    timestamp: HybridLogicalClock,
}

impl<T> LWWRegister<T> {
    pub fn new(value: T, timestamp: HybridLogicalClock) -> Self {
        LWWRegister { value, timestamp }
    }

    pub fn set(&mut self, value: T, timestamp: HybridLogicalClock) {
        self.value = value;
        self.timestamp = timestamp;
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Merge for LWWRegister<T>
where
    T: Clone + Ord,
{
    fn merge_mut(&mut self, other: Self) {
        if other.timestamp > self.timestamp {
            self.set(other.value, other.timestamp)
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
        fn test_idempotent(v: LWWRegister<bool>) {
           merge::test_idempotent(v);
        }
    }

    proptest! {
        #[test]
        fn test_commutative(a: LWWRegister<bool>, b: LWWRegister<bool>) {
            merge::test_commutative(a, b);
        }
    }

    proptest! {
        #[test]
        fn test_associative(a: LWWRegister<bool>, b: LWWRegister<bool>, c: LWWRegister<bool>) {
            merge::test_associative(a, b, c);
        }
    }
}

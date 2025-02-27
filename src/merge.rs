pub trait Merge {
    fn merge(self, other: Self) -> Self;
}

/// Test that a Merge implementation is idempotent (in other words, merging
/// multiple times should not change the state.)
#[cfg(test)]
pub fn test_idempotent<T>(v: T)
where
    T: Merge + Clone + PartialEq + std::fmt::Debug,
{
    assert_eq!(v.clone().merge(v.clone()), v);
}

/// Test that the implementation is commutative (in other words, the order of
/// merges should not effect the final result.)
#[cfg(test)]
pub fn test_commutative<T>(a: T, b: T)
where
    T: Merge + Clone + PartialEq + std::fmt::Debug,
{
    let ab = a.clone().merge(b.clone());
    let ba = b.merge(a);

    assert_eq!(ab, ba);
}

/// Test that a Merge implementation is associative (in other words, the order
/// in which replicas are merged should not effect the final result.)
#[cfg(test)]
pub fn test_associative<T>(a: T, b: T, c: T)
where
    T: Merge + Clone + PartialEq + std::fmt::Debug,
{
    let ab_c = a.clone().merge(b.clone()).merge(c.clone());
    let a_bc = a.merge(b.merge(c));

    assert_eq!(ab_c, a_bc);
}

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

/// Trait for extracting key from a data structure.

pub trait Key {
    /// Type of the key.
    type Output;

    /// Extract the key from the data structure.
    fn key(&self) -> Self::Output;
}

/// A wrapper for data structures that implements [`Key`](`Key`) trait.
#[derive(Clone, Copy, Debug, Default)]
pub struct Keyed<T>(pub T);

impl<T: Key> From<T> for Keyed<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Key> PartialEq for Keyed<T>
where
    T::Output: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.key().eq(&other.0.key())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.0.key().ne(&other.0.key())
    }
}

impl<T: Key> Eq for Keyed<T> where T::Output: Eq {}

impl<T: Key> PartialOrd for Keyed<T>
where
    T::Output: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.key().partial_cmp(&other.0.key())
    }

    fn lt(&self, other: &Self) -> bool {
        self.0.key().lt(&other.0.key())
    }

    fn le(&self, other: &Self) -> bool {
        self.0.key().le(&other.0.key())
    }

    fn gt(&self, other: &Self) -> bool {
        self.0.key().gt(&other.0.key())
    }

    fn ge(&self, other: &Self) -> bool {
        self.0.key().ge(&other.0.key())
    }
}

impl<T: Key> Ord for Keyed<T>
where
    T::Output: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.key().cmp(&other.0.key())
    }
}

impl<T: Key> Hash for Keyed<T>
where
    T::Output: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.key().hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{self, KeyValuePair};
    use super::{Key, Keyed};

    impl<K: Clone, V> Key for KeyValuePair<K, V> {
        type Output = K;

        fn key(&self) -> Self::Output {
            self.key.clone()
        }
    }

    #[test]
    fn test_partial_eq() {
        tests::test_partial_eq(Keyed);
    }

    #[test]
    fn test_eq() {
        tests::test_eq(Keyed);
    }

    #[test]
    fn test_partial_ord() {
        tests::test_partial_ord(Keyed);
    }

    #[test]
    fn test_ord() {
        tests::test_ord(Keyed);
    }

    #[test]
    fn test_hash() {
        tests::test_hash(Keyed);
    }
}

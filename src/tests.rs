use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

extern crate std;

pub struct KeyValuePair<K, V> {
    pub key: K,
    pub value: V,
}

pub fn test_partial_eq<W: PartialEq>(wrapper: impl Fn(KeyValuePair<i32, i32>) -> W) {
    assert!(wrapper(KeyValuePair { key: 4, value: 3 }).eq(&wrapper(KeyValuePair { key: 4, value: 2 })));

    assert!(wrapper(KeyValuePair { key: 4, value: 3 }).ne(&wrapper(KeyValuePair { key: 3, value: 3 })));
}

pub fn test_eq<W: Eq>(_: impl Fn(KeyValuePair<i32, i32>) -> W) {}

pub fn test_partial_ord<W: PartialOrd>(wrapper: impl Fn(KeyValuePair<f32, i32>) -> W) {
    assert_eq!(
        wrapper(KeyValuePair { key: 4.0, value: 3 }).partial_cmp(&wrapper(KeyValuePair { key: 3.0, value: 3 })),
        Some(Ordering::Greater)
    );

    assert_eq!(
        wrapper(KeyValuePair { key: 4.0, value: 3 }).partial_cmp(&wrapper(KeyValuePair { key: 4.0, value: 3 })),
        Some(Ordering::Equal)
    );

    assert_eq!(
        wrapper(KeyValuePair { key: 4.0, value: 3 }).partial_cmp(&wrapper(KeyValuePair { key: 5.0, value: 3 })),
        Some(Ordering::Less)
    );

    assert_eq!(
        wrapper(KeyValuePair { key: 4.0, value: 3 }).partial_cmp(&wrapper(KeyValuePair {
            key: f32::NAN,
            value: 3
        })),
        None
    );
}

pub fn test_ord<W: Ord>(wrapper: impl Fn(KeyValuePair<i32, i32>) -> W) {
    assert_eq!(
        wrapper(KeyValuePair { key: 4, value: 3 }).cmp(&wrapper(KeyValuePair { key: 3, value: 3 })),
        Ordering::Greater
    );

    assert_eq!(
        wrapper(KeyValuePair { key: 4, value: 3 }).cmp(&wrapper(KeyValuePair { key: 4, value: 3 })),
        Ordering::Equal
    );

    assert_eq!(
        wrapper(KeyValuePair { key: 4, value: 3 }).cmp(&wrapper(KeyValuePair { key: 5, value: 3 })),
        Ordering::Less
    );
}

fn get_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::default();

    value.hash(&mut hasher);

    hasher.finish()
}

pub fn test_hash<W: Hash>(wrapper: impl Fn(KeyValuePair<i32, i32>) -> W) {
    for &key in &[2, 3, 5, 7, 11] {
        assert_eq!(get_hash(&wrapper(KeyValuePair { key, value: 7 })), get_hash(&key))
    }
}

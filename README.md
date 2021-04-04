# keyed

[![.github/workflows/ci.yml](https://github.com/EFanZh/keyed/actions/workflows/ci.yml/badge.svg)](https://github.com/EFanZh/keyed/actions/workflows/ci.yml)
[![docs](https://docs.rs/keyed/badge.svg)](https://docs.rs/keyed)

Make implementing key-based comparison a little bit easier. Usage:

1. Implement [`Key`](`Key`) trait for your type.
2. Wrap your value in a [`Keyed`](`Keyed`) structure.

Example:

```rust
use keyed::{Key, Keyed};

#[derive(Debug)]
struct Item<T> {
    key: i32,
    value: T,
}

impl<T> Key for Item<T> {
    type Output = i32;

    fn key(&self) -> Self::Output {
        self.key
    }
}

let lhs = Keyed(Item { key: 4, value: 3 });
let rhs = Keyed(Item { key: 4, value: 7 });

assert_eq!(lhs, rhs);
```

If your key is a reference to internal data, you can use [`RefKey`](https://docs.rs/keyed/*/keyed/trait.Key.html) trait
and [`RefKeyed`](https://docs.rs/keyed/*/keyed/trait.RefKey.html) wrapper:

```rust
use keyed::{RefKey, RefKeyed};

#[derive(Debug)]
struct Item<T> {
    key: i32,
    value: T,
}

impl<T> RefKey for Item<T> {
    type Output = i32;

    fn key(&self) -> &Self::Output {
        &self.key
    }
}

let lhs = RefKeyed(Item { key: 4, value: 3 });
let rhs = RefKeyed(Item { key: 4, value: 7 });

assert_eq!(lhs, rhs);
```

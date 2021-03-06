//! # keyed
//!
//! Make implementing key-based comparison a little bit easier. Usage:
//!
//! 1. Implement [`Key`](`Key`) trait for your type.
//! 2. Wrap your value in a [`Keyed`](`Keyed`) structure.
//!
//! Example:
//!
//! ```rust
//! use keyed::{Key, Keyed};
//!
//! #[derive(Debug)]
//! struct Item<T> {
//!     key: i32,
//!     value: T,
//! }
//!
//! impl<T> Key for Item<T> {
//!     type Output = i32;
//!
//!     fn key(&self) -> Self::Output {
//!         self.key
//!     }
//! }
//!
//! let lhs = Keyed(Item { key: 4, value: 3 });
//! let rhs = Keyed(Item { key: 4, value: 7 });
//!
//! assert_eq!(lhs, rhs);
//! ```
//!
//! If your key is a reference to internal data, you can use [`RefKey`](`RefKey`) trait and [`RefKeyed`](`RefKeyed`)
//! wrapper:
//!
//! ```rust
//! use keyed::{RefKey, RefKeyed};
//!
//! #[derive(Debug)]
//! struct Item<T> {
//!     key: i32,
//!     value: T,
//! }
//!
//! impl<T> RefKey for Item<T> {
//!     type Output = i32;
//!
//!     fn key(&self) -> &Self::Output {
//!         &self.key
//!     }
//! }
//!
//! let lhs = RefKeyed(Item { key: 4, value: 3 });
//! let rhs = RefKeyed(Item { key: 4, value: 7 });
//!
//! assert_eq!(lhs, rhs);
//! ```

#![no_std]
#![warn(clippy::cargo, clippy::pedantic)]

mod keyed;
mod ref_keyed;

pub use crate::keyed::Key;
pub use crate::keyed::Keyed;
pub use crate::ref_keyed::RefKey;
pub use crate::ref_keyed::RefKeyed;

#[cfg(test)]
mod tests;

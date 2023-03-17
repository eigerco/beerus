#[macro_use]
extern crate alloc;

pub mod with_alloc {
    pub use alloc::{borrow, string, sync, vec, boxed, collections, format, str};
}
pub mod with_std {
    pub use std::{borrow, fmt, mem, string, sync, vec, boxed, format, primitive, str};

    pub mod collections {
        pub use std::collections::{BTreeMap};
    }
}
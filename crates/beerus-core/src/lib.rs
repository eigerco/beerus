#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]

#[cfg(feature = "std")]
include!("./with_std.rs");

#[cfg(not(feature = "std"))]
include!("./without_std.rs");

#[cfg(not(feature = "std"))]
include!("./with_alloc.rs");

pub mod stdlib {
    #[cfg(not(feature = "std"))]
    pub use crate::with_alloc::*;
    #[cfg(feature = "std")]
    pub use crate::with_std::*;
    #[cfg(not(feature = "std"))]
    pub use crate::without_std::*;
}

pub mod config;
pub mod ethers_helper;
pub mod lightclient;
pub mod starknet_helper;

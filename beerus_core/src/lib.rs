// #![warn(
//     clippy::std_instead_of_core,
//     clippy::alloc_instead_of_core,
//     clippy::std_instead_of_alloc
// )]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(async_fn_in_trait)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

pub mod config;
pub mod ethers_helper;
pub mod lightclient;
pub mod starknet_helper;

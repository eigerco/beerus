pub mod client;
pub mod config;
pub mod exe;
pub mod feeder;

#[cfg(not(tarpaulin_include))] // exclude from code-coverage report
pub mod gen;

pub mod proof;

#[cfg(not(target_arch = "wasm32"))]
pub mod rpc;

pub mod util;

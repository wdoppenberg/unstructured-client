#![doc = include_str!("../README.md")]

pub mod client;
pub mod error;
pub mod partition;

pub use client::UnstructuredClient;
pub use partition::{ElementList, PartitionParameters};

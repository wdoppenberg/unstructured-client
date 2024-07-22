#![doc = include_str!("../README.md")]

pub mod client;
mod element;
pub mod error;
mod metadata;
pub mod partition;

pub use client::UnstructuredClient;
pub use element::ElementList;
pub use partition::PartitionParameters;

#![cfg_attr(not(feature = "std"), no_std)]
// TODO #![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![doc = document_features::document_features!()]

extern crate alloc;

mod coordinator;
mod error;
mod signer;

pub use frost_core;

pub use coordinator::*;
pub use error::*;
pub use signer::*;

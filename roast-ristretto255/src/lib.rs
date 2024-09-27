#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![doc = document_features::document_features!()]

mod coordinator {
    /// Represents all possible session statuses.
    pub type SessionStatus = roast_core::SessionStatus<frost_ristretto255::Ristretto255Sha512>;

    /// Represents coordinator.
    pub type Coordinator = roast_core::Coordinator<frost_ristretto255::Ristretto255Sha512>;
}

pub mod error {
    //! Error types.

    /// Represents all possible errors that can occur in FROST protocol.
    pub type FrostError = roast_core::error::FrostError<frost_ristretto255::Ristretto255Sha512>;

    pub use roast_core::error::{MaliciousSignerError, RoastError};

    /// Represents all possible errors that can occur.
    pub type Error = roast_core::error::Error<frost_ristretto255::Ristretto255Sha512>;
}

mod signer {
    /// Represents signer.
    pub type Signer = roast_core::Signer<frost_ristretto255::Ristretto255Sha512>;
}

pub use frost_ristretto255 as frost;

pub use coordinator::*;
pub use signer::*;

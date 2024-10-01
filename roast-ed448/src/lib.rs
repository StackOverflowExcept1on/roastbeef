#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![doc = document_features::document_features!()]

mod coordinator {
    /// Represents all possible session statuses.
    pub type SessionStatus = roast_core::SessionStatus<frost_ed448::Ed448Shake256>;

    /// Represents coordinator.
    pub type Coordinator = roast_core::Coordinator<frost_ed448::Ed448Shake256>;
}

pub mod dkg {
    //! Distributed Key Generation types.

    pub use roast_core::dkg::DkgStatus;

    /// Represents dealer that can be used for Distributed Key Generation.
    pub type Dealer = roast_core::dkg::Dealer<frost_ed448::Ed448Shake256>;

    /// Represents participant of Distributed Key Generation.
    pub type Participant = roast_core::dkg::Participant<frost_ed448::Ed448Shake256>;
}

pub mod error {
    //! Error types.

    /// Represents all possible errors that can occur in FROST protocol.
    pub type FrostError = roast_core::error::FrostError<frost_ed448::Ed448Shake256>;

    pub use roast_core::error::{DkgError, MaliciousSignerError, RoastError};

    /// Represents all possible errors that can occur.
    pub type Error = roast_core::error::Error<frost_ed448::Ed448Shake256>;
}

mod signer {
    /// Represents signer.
    pub type Signer = roast_core::Signer<frost_ed448::Ed448Shake256>;
}

pub use frost_ed448 as frost;

pub use coordinator::*;
pub use signer::*;

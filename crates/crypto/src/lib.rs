//! This is Spacedrive's `crypto` crate. It handles cryptographic operations
//! such as key hashing, encryption/decryption, key management and much more.
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::correctness)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

pub mod crypto;
pub mod error;
pub mod keys;
pub mod primitives;
pub mod protected;
pub mod types;
pub mod util;

#[cfg(feature = "encoding")]
pub mod encoding;

#[cfg(feature = "async")]
pub mod fs;

#[cfg(feature = "encoding")]
pub mod header;

// Re-export so they can be used elsewhere/cleaner `use` declarations
pub use self::error::{Error, Result};
pub use aead::Payload;
pub use protected::Protected;
pub use zeroize::Zeroize;

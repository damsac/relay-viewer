//! relay-viewer core library
//!
//! All business logic lives here. No platform-specific code.
//! Native layers (iOS/Android) are thin UI shells that render state from this crate.

pub mod error;
pub mod models;
pub mod relay;
pub mod store;

pub use error::Error;
pub use models::*;
pub use relay::RelayClient;
pub use store::Store;

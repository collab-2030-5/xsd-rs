mod config;
mod error;
mod helpers;
mod traits;

/// base types that can be polymorphic
pub mod base;
/// struct definitions from the XSD
pub mod structs;

pub(crate) use helpers::*;

pub use config::*;
pub use error::*;
pub use traits::*;
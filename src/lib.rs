//! This crate provides macros for getting compile time information.
//!
//! You can get the compile time either as
//! [`time::Date`], [`time::Time`],
//! [`time::OffsetDateTime`], string, or UNIX timestamp.
//!
//! You can get the Rust compiler version either as
//! [`semver::Version`] or string,
//! and the individual version parts as integer literals or strings, respectively.
//!
//! # Example
//!
//! ```
//! let compile_datetime = compile_time::datetime_str!();
//! let rustc_version = compile_time::rustc_version_str!();
//!
//! println!("Compiled using Rust {rustc_version} on {compile_datetime}.");
//! ```
#![no_std]

pub use compile_time_macros::*;

/// The host platform triple.
pub const HOST: &str = env!("HOST");

/// The target platform triple.
pub const TARGET: &str = env!("TARGET");

#[doc(hidden)]
pub mod __re_exports {
  #[cfg(feature = "version")]
  pub use semver;
  #[cfg(feature = "time")]
  pub use time;
}

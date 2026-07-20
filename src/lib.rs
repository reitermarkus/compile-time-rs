//! This crate provides macros for getting compile time information.
//!
//! You can get the compile time either as
//! [`time::Date`](time03::Date), [`time::Time`](time03::Time),
//! [`time::OffsetDateTime`](time03::OffsetDateTime), string, or UNIX timestamp.
//!
//! You can get the Rust compiler version either as
//! [`semver::Version`] or string,
//! and the individual version parts as integer literals or strings, respectively.
//!
//! You can run arbitrary command at compile time and get its output as bytes or string.
//!
//! # Examples
//!
//! Getting the compile time and Rust version:
//!
//! ```rust
//! const COMPILE_DATETIME: &str = compile_time::datetime_str!();
//! const RUSTC_VERSION: &str = compile_time::rustc_version_str!();
//!
//! println!("Compiled using Rust {RUSTC_VERSION} on {COMPILE_DATETIME}.");
//! ```
//!
//! Running an arbitrary command at compile time:
//!
//! ```rust
//! const MAGIC_NUMBER: &[u8] = compile_time::command_bytes!("echo", "42");
//!
//! assert_eq!(MAGIC_NUMBER, b"42\n");
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
  pub use time03 as time;
}

#[doc = include_str!("../ReadMe.md")]
#[doc(hidden)]
pub fn __readme_doc_test() {}

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
//! const MAGIC_NUMBER: &str = compile_time::command_str!("echo", "42");
//!
//! assert_eq!(MAGIC_NUMBER, "42\n");
//! ```
#![no_std]

pub use compile_time_macros::*;

mod constants {
  include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

/// The host platform triple.
///
/// # Examples
///
/// When building on Apple Silicon macOS:
///
/// ```
/// # #[cfg(all(host_arch = "aarch64", host_os = "macos"))]
/// assert_eq!(compile_time::HOST, "aarch64-apple-darwin");
/// ```
///
/// When building on x86 64-bit Linux:
///
/// ```
/// # #[cfg(all(host_arch = "x86_64", host_os = "linux", host_env = "gnu"))]
/// assert_eq!(compile_time::HOST, "x86_64-unknown-linux-gnu");
/// ```
///
/// When building on x86 64-bit Windows:
///
/// ```
/// # #[cfg(all(host_arch = "x86_64", host_os = "windows", host_env = "msvc"))]
/// assert_eq!(compile_time::HOST, "x86_64-pc-windows-msvc");
/// ```
pub const HOST: &str = constants::HOST;

/// The target platform triple.
///
/// # Examples
///
/// When building for Apple Silicon macOS:
///
/// ```
/// # #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
/// assert_eq!(compile_time::TARGET, "aarch64-apple-darwin");
/// ```
///
/// When building for x86 64-bit Linux:
///
/// ```
/// # #[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
/// assert_eq!(compile_time::TARGET, "x86_64-unknown-linux-gnu");
/// ```
///
/// When building for x86 64-bit Windows:
///
/// ```
/// # #[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "mscv"))]
/// assert_eq!(compile_time::TARGET, "x86_64-pc-windows-msvc2");
/// ```
pub const TARGET: &str = constants::TARGET;

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

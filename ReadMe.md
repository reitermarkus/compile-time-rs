# `compile-time`

[![Crates.io](https://img.shields.io/crates/v/compile-time.svg)](https://crates.io/crates/compile-time)
[![Documentation](https://docs.rs/compile-time/badge.svg)](https://docs.rs/compile-time)

This crate provides macros for getting compile time information.

You can get the compile time either as `time::Date`, `time::Time`,
`time::OffsetDateTime`, string, or UNIX timestamp.

You can get the Rust compiler version either as `semver::Version` or string,
and the individual version parts as integer literals or strings, respectively.

You can run arbitrary command at compile time and get its output as bytes or string.

# Examples

Getting the compile time and Rust version:

```rust
const COMPILE_DATETIME: &str = compile_time::datetime_str!();
const RUSTC_VERSION: &str = compile_time::rustc_version_str!();

println!("Compiled using Rust {RUSTC_VERSION} on {COMPILE_DATETIME}.");
```

Running an arbitrary command at compile time:

```rust
const MAGIC_NUMBER: &str = compile_time::command_str!("echo", "42");

assert_eq!(MAGIC_NUMBER, "42\n");
```

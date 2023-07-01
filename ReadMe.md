# `compile-time`

[![Crates.io](https://img.shields.io/crates/v/compile-time.svg)](https://crates.io/crates/compile-time)
[![Documentation](https://docs.rs/compile-time/badge.svg)](https://docs.rs/compile-time)

This crate provides macros for getting compile time information.

You can get the compile time either as `time::Date`, `time::Time`,
`time::OffsetDateTime`, string, or UNIX timestamp.

You can get the Rust compiler version either as `semver::Version` or string,
and the individual version parts as integer literals or strings, respectively.

# Example

```rust
let compile_datetime = compile_time::datetime_str!();
let rustc_version = compile_time::rustc_version_str!();

println!("Compiled using Rust {rustc_version} on {compile_datetime}.");
```

//! This crate provides macros for getting compile time information.
//!
//! This crate should not be used directly; use [`compile-time`](https://docs.rs/compile-time) instead.

extern crate proc_macro;

#[cfg(any(feature = "time", feature = "version"))]
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
#[cfg(feature = "time")]
use time::{macros::format_description, OffsetDateTime};

mod command;

#[cfg(feature = "time")]
static COMPILE_TIME: Lazy<OffsetDateTime> = Lazy::new(OffsetDateTime::now_utc);
#[cfg(feature = "version")]
static RUSTC_VERSION: Lazy<rustc_version::Result<rustc_version::Version>> = Lazy::new(rustc_version::version);

/// Compile date as `time::Date`.
///
/// # Example
///
/// ```
/// const COMPILE_DATE: time::Date = compile_time::date!();
///
/// let year = COMPILE_DATE.year();
/// let month = COMPILE_DATE.month();
/// let day = COMPILE_DATE.day();
/// println!("Compiled on {month} {day}, {year}.");
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn date(_input: TokenStream) -> TokenStream {
  let date = COMPILE_TIME.date();

  let year = date.year();
  let month = format_ident!("{}", format!("{:?}", date.month()));
  let day = date.day();

  let time_prefix = quote! { ::compile_time::__re_exports::time };
  quote! {
    const {
      match #time_prefix::Date::from_calendar_date(#year, #time_prefix::Month::#month, #day) {
        Ok(date) => date,
        _ => ::core::unreachable!(),
      }
    }
  }
  .into()
}

/// Compile date as `&'static str` in `yyyy-MM-dd` format.
///
/// # Example
///
/// ```
/// const COMPILE_DATE: time::Date = compile_time::date!();
///
/// let year = COMPILE_DATE.year();
/// let month: u8 = COMPILE_DATE.month().into();
/// let day = COMPILE_DATE.day();
/// let date_string = format!("{year:04}-{month:02}-{day:02}");
///
/// assert_eq!(compile_time::date_str!(), date_string);
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn date_str(_input: TokenStream) -> TokenStream {
  let date = COMPILE_TIME.date();

  let fmt = format_description!("[year]-[month]-[day]");
  let date_str = date.format(&fmt).unwrap();

  quote! { #date_str }.into()
}

/// Compile time as `time::Time`.
///
/// # Example
///
/// ```
/// const COMPILE_TIME: time::Time = compile_time::time!();
///
/// let hour = COMPILE_TIME.hour();
/// let minute = COMPILE_TIME.minute();
/// let second = COMPILE_TIME.second();
/// println!("Compiled at {hour:02}:{minute:02}:{second:02}.");
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn time(_input: TokenStream) -> TokenStream {
  let time = COMPILE_TIME.time();

  let hour = time.hour();
  let minute = time.minute();
  let second = time.second();

  let time_prefix = quote! { ::compile_time::__re_exports::time };
  quote! {
    const {
      match #time_prefix::Time::from_hms(#hour, #minute, #second) {
        Ok(time) => time,
        _ => ::core::unreachable!(),
      }
    }
  }
  .into()
}

/// Compile time as `&'static str` in `hh:mm:ss` format.
///
/// # Example
///
/// ```
/// const COMPILE_TIME: time::Time = compile_time::time!();
///
/// let hour = COMPILE_TIME.hour();
/// let minute = COMPILE_TIME.minute();
/// let second = COMPILE_TIME.second();
/// let time_string = format!("{hour:02}:{minute:02}:{second:02}");
///
/// assert_eq!(compile_time::time_str!(), time_string);
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn time_str(_input: TokenStream) -> TokenStream {
  let time = COMPILE_TIME.time();

  let fmt = format_description!("[hour]:[minute]:[second]");
  let time_str = time.format(&fmt).unwrap();

  quote! { #time_str }.into()
}

/// Compile date and time as `time::OffsetDateTime`.
///
/// # Example
///
/// ```
/// const COMPILE_DATETIME: time::OffsetDateTime = compile_time::datetime!();
///
/// let year = COMPILE_DATETIME.year();
/// let month = COMPILE_DATETIME.month();
/// let day = COMPILE_DATETIME.day();
/// let hour = COMPILE_DATETIME.hour();
/// let minute = COMPILE_DATETIME.minute();
/// let second = COMPILE_DATETIME.second();
/// println!("Compiled at {hour:02}:{minute:02}:{second:02} on {month} {day}, {year}.");
/// #
/// # // Evaluation is only done once.
/// # std::thread::sleep(std::time::Duration::from_secs(1));
/// # assert_eq!(COMPILE_DATETIME, compile_time::datetime!());
/// #
/// # // Additional sanity check.
/// # let now = time::OffsetDateTime::now_utc();
/// # let yesterday = now.saturating_sub(time::Duration::days(1));
/// # assert!(COMPILE_DATETIME > yesterday);
/// # assert!(COMPILE_DATETIME < now);
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn datetime(_input: TokenStream) -> TokenStream {
  let datetime = *COMPILE_TIME;

  let year = datetime.year();
  let month = format_ident!("{}", format!("{:?}", datetime.month()));
  let day = datetime.day();

  let hour = datetime.hour();
  let minute = datetime.minute();
  let second = datetime.second();

  let time_prefix = quote! { ::compile_time::__re_exports::time };
  let date = quote! {
    match #time_prefix::Date::from_calendar_date(#year, #time_prefix::Month::#month, #day) {
      Ok(date) => date,
      _ => ::core::unreachable!(),
    }
  };

  let time = quote! {
    match #time_prefix::Time::from_hms(#hour, #minute, #second) {
      Ok(time) => time,
      _ => ::core::unreachable!(),
    }
  };

  quote! {
    const { #time_prefix::PrimitiveDateTime::new(#date, #time).assume_utc() }
  }
  .into()
}

/// Compile time as `&'static str` in `yyyy-MM-ddThh:mm:ssZ` format.
///
/// # Example
///
/// ```
/// const COMPILE_DATE_STRING: &str = compile_time::date_str!();
/// const COMPILE_TIME_STRING: &str = compile_time::time_str!();
///
/// let datetime_string = format!("{COMPILE_DATE_STRING}T{COMPILE_TIME_STRING}Z");
/// assert_eq!(compile_time::datetime_str!(), datetime_string);
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn datetime_str(_input: TokenStream) -> TokenStream {
  let datetime = *COMPILE_TIME;

  let fmt = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");
  let datetime_str = datetime.format(&fmt).unwrap();

  quote! { #datetime_str }.into()
}

/// Compile date and time as UNIX timestamp in seconds.
///
/// # Example
///
/// ```
/// const COMPILE_DATETIME: time::OffsetDateTime = compile_time::datetime!();
///
/// assert_eq!(compile_time::unix!(), COMPILE_DATETIME.unix_timestamp());
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn unix(_input: TokenStream) -> TokenStream {
  let datetime = *COMPILE_TIME;

  let unix_timestamp = proc_macro2::Literal::i64_unsuffixed(datetime.unix_timestamp());

  quote! {
    #unix_timestamp
  }
  .into()
}

/// Rust compiler version as `semver::Version`.
///
/// # Example
///
/// ```
/// let rustc_version: semver::Version = compile_time::rustc_version!();
/// assert_eq!(rustc_version, rustc_version::version().unwrap());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version(_item: TokenStream) -> TokenStream {
  let rustc_version::Version { major, minor, patch, pre, build } = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version,
    Err(err) => panic!("Failed to get version: {}", err),
  };

  let semver_prefix = quote! { ::compile_time::__re_exports::semver };
  let pre = if pre.is_empty() {
    quote! { #semver_prefix::Prerelease::EMPTY }
  } else {
    let pre = pre.as_str();
    quote! {
      if let Ok(pre) = #semver_prefix::Prerelease::new(#pre) {
        pre
      } else {
        ::core::unreachable!()
      }
    }
  };

  let build = if build.is_empty() {
    quote! { #semver_prefix::BuildMetadata::EMPTY }
  } else {
    let build = build.as_str();
    quote! {
      if let Ok(build) = #semver_prefix::BuildMetadata::new(#build) {
        build
      } else {
        ::core::unreachable!()
      }
    }
  };

  quote! {
    #semver_prefix::Version {
      major: #major,
      minor: #minor,
      patch: #patch,
      pre: #pre,
      build: #build,
    }
  }
  .into()
}

/// Rust compiler version as `&'static str`.
///
/// # Example
///
/// ```
/// const RUSTC_VERSION_STRING: &str = compile_time::rustc_version_str!();
/// assert_eq!(RUSTC_VERSION_STRING, compile_time::rustc_version_str!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_str(_item: TokenStream) -> TokenStream {
  let rustc_version = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version,
    Err(err) => panic!("Failed to get version: {}", err),
  };

  let rustc_version_string = rustc_version.to_string();
  quote! { #rustc_version_string }.into()
}

/// Rust compiler major version as integer literal.
///
/// # Example
///
/// ```
/// let rustc_version: semver::Version = compile_time::rustc_version!();
/// assert_eq!(rustc_version.major, compile_time::rustc_version_major!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_major(_item: TokenStream) -> TokenStream {
  let major = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version.major,
    Err(err) => panic!("Failed to get version: {}", err),
  };

  proc_macro2::Literal::u64_unsuffixed(major).to_token_stream().into()
}

/// Rust compiler minor version as integer literal.
///
/// # Example
///
/// ```
/// let rustc_version: semver::Version = compile_time::rustc_version!();
/// assert_eq!(rustc_version.minor, compile_time::rustc_version_minor!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_minor(_item: TokenStream) -> TokenStream {
  let minor = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version.minor,
    Err(err) => panic!("Failed to get version: {}", err),
  };

  proc_macro2::Literal::u64_unsuffixed(minor).to_token_stream().into()
}

/// Rust compiler patch version as integer literal.
///
/// # Example
///
/// ```
/// let rustc_version: semver::Version = compile_time::rustc_version!();
/// assert_eq!(rustc_version.minor, compile_time::rustc_version_minor!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_patch(_item: TokenStream) -> TokenStream {
  let patch = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version.patch,
    Err(err) => panic!("Failed to get version: {}", err),
  };

  proc_macro2::Literal::u64_unsuffixed(patch).to_token_stream().into()
}

/// Rust compiler pre version as `&'static str`.
///
/// # Example
///
/// ```
/// let rustc_version: semver::Version = compile_time::rustc_version!();
/// assert_eq!(rustc_version.pre.as_str(), compile_time::rustc_version_pre!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_pre(_item: TokenStream) -> TokenStream {
  let pre = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version.pre.as_str(),
    Err(err) => panic!("Failed to get version: {}", err),
  };

  quote! { #pre }.into()
}

/// Rust compiler build version as `&'static str`.
///
/// # Example
///
/// ```
/// let rustc_version: semver::Version = compile_time::rustc_version!();
/// assert_eq!(rustc_version.build.as_str(), compile_time::rustc_version_build!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_build(_item: TokenStream) -> TokenStream {
  let build = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version.build.as_str(),
    Err(err) => panic!("Failed to get version: {}", err),
  };

  quote! { #build }.into()
}

/// Rust compiler build version as `&'static str`.
///
/// # Example
///
/// ```
/// let magic_number: &[u8] = compile_time::command_bytes!("echo", "42");
/// assert_eq!(magic_number, b"42\n");
/// ```
#[proc_macro]
#[cfg(feature = "version")]
pub fn command_bytes(input: TokenStream) -> TokenStream {
  use proc_macro2::Span;
  use syn::{parse_macro_input, LitByteStr};

  use command::CommandInput;

  let input = parse_macro_input!(input as CommandInput);
  match command::output(input) {
    Ok(output) => {
      let output = LitByteStr::new(&output, Span::call_site());
      quote! { #output }.into()
    },
    Err(e) => e,
  }
}

/// Rust compiler build version as `&'static str`.
///
/// # Example
///
/// ```
/// let magic_number: &str = compile_time::command_str!("echo", "42");
/// assert_eq!(magic_number, "42\n");
/// ```
#[proc_macro]
#[cfg(feature = "version")]
pub fn command_str(input: TokenStream) -> TokenStream {
  use proc_macro2::Span;
  use syn::{parse_macro_input, LitStr};

  use command::CommandInput;

  let input = parse_macro_input!(input as CommandInput);
  match command::output(input) {
    Ok(output) => {
      let output_string = match String::from_utf8(output) {
        Ok(string) => string,
        Err(_) => {
          return quote! {
            ::core::compile_error!("Command output is not valid UTF-8.")
          }
          .into()
        },
      };

      let output = LitStr::new(&output_string, Span::call_site());
      quote! { #output }.into()
    },
    Err(e) => e,
  }
}

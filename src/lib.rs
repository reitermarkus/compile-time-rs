//! This crate provides macros for getting compile time information.
//!
//! You can get the compile time either as
//! [`time::Date`](time::Date), [`time::Time`](time::Time),
//! [`time::OffsetDateTime`](time::OffsetDateTime), string, or UNIX timestamp.
//!
//! You can get the Rust compiler version either as
//! [`semver::Version`](https://docs.rs/semver/1/semver/struct.Version.html) or string,
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

extern crate proc_macro;

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use time::{macros::format_description, OffsetDateTime};

static COMPILE_TIME: Lazy<OffsetDateTime> = Lazy::new(OffsetDateTime::now_utc);
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
#[proc_macro]
pub fn date(_item: TokenStream) -> TokenStream {
  let date = COMPILE_TIME.date();

  let year = date.year();
  let month = format_ident!("{}", format!("{:?}", date.month()));
  let day = date.day();

  quote! {
    match ::time::Date::from_calendar_date(#year, ::time::Month::#month, #day) {
      Ok(date) => date,
      _ => ::core::unreachable!(),
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
#[proc_macro]
pub fn date_str(_item: TokenStream) -> TokenStream {
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
#[proc_macro]
pub fn time(_item: TokenStream) -> TokenStream {
  let time = COMPILE_TIME.time();

  let hour = time.hour();
  let minute = time.minute();
  let second = time.second();

  quote! {
    match ::time::Time::from_hms(#hour, #minute, #second) {
      Ok(time) => time,
      _ => ::core::unreachable!(),
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
#[proc_macro]
pub fn time_str(_item: TokenStream) -> TokenStream {
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
#[proc_macro]
pub fn datetime(_item: TokenStream) -> TokenStream {
  let datetime = *COMPILE_TIME;

  let year = datetime.year();
  let month = format_ident!("{}", format!("{:?}", datetime.month()));
  let day = datetime.day();

  let hour = datetime.hour();
  let minute = datetime.minute();
  let second = datetime.second();

  let date = quote! {
    match ::time::Date::from_calendar_date(#year, ::time::Month::#month, #day) {
      Ok(date) => date,
      _ => ::core::unreachable!(),
    }
  };

  let time = quote! {
    match ::time::Time::from_hms(#hour, #minute, #second) {
      Ok(time) => time,
      _ => ::core::unreachable!(),
    }
  };

  quote! {
    ::time::PrimitiveDateTime::new(#date, #time).assume_utc()
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
#[proc_macro]
pub fn datetime_str(_item: TokenStream) -> TokenStream {
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
#[proc_macro]
pub fn unix(_item: TokenStream) -> TokenStream {
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
#[proc_macro]
pub fn rustc_version(_item: TokenStream) -> TokenStream {
  let rustc_version::Version { major, minor, patch, pre, build } = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version,
    Err(err) => panic!("Failed to get version: {}", err),
  };

  let pre = if pre.is_empty() {
    quote! { ::semver::Prerelease::EMPTY }
  } else {
    let pre = pre.as_str();
    quote! {
      if let Ok(pre) = ::semver::Prerelease::new(#pre) {
        pre
      } else {
        ::core::unreachable!()
      }
    }
  };

  let build = if build.is_empty() {
    quote! { ::semver::BuildMetadata::EMPTY }
  } else {
    let build = build.as_str();
    quote! {
      if let Ok(build) = ::semver::BuildMetadata::new(#build) {
        build
      } else {
        ::core::unreachable!()
      }
    }
  };

  quote! {
    ::semver::Version {
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
#[proc_macro]
pub fn rustc_version_build(_item: TokenStream) -> TokenStream {
  let build = match &*RUSTC_VERSION {
    Ok(rustc_version) => rustc_version.build.as_str(),
    Err(err) => panic!("Failed to get version: {}", err),
  };

  quote! { #build }.into()
}

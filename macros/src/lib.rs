//! This crate provides macros for getting compile time information.
//!
//! This crate should not be used directly; use [`compile-time`](https://docs.rs/compile-time) instead.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};

#[cfg(feature = "command")]
mod command;
#[cfg(feature = "time")]
mod time;
#[cfg(feature = "version")]
mod version;

/// Returns the compile date as [`time::Date`](time03::Date).
///
/// By default, the returned date is in UTC, call `date!(local)`
/// to return the local date.
///
/// # Example
///
/// Get the UTC date:
///
/// ```
/// # use time03 as time;
/// const COMPILE_DATE: time::Date = compile_time::date!();
///
/// let year = COMPILE_DATE.year();
/// let month = COMPILE_DATE.month();
/// let day = COMPILE_DATE.day();
/// println!("Compiled on {month} {day}, {year}.");
/// ```
///
/// Get the date for the local time zone:
///
/// ```
/// # use time03 as time;
/// const COMPILE_DATE: time::Date = compile_time::date!(local);
///
/// let year = COMPILE_DATE.year();
/// let month = COMPILE_DATE.month();
/// let day = COMPILE_DATE.day();
/// println!("Compiled on {month} {day}, {year}.");
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn date(input: TokenStream) -> TokenStream {
  use syn::parse_macro_input;

  use time::TimeInput;

  let input = parse_macro_input!(input as TimeInput);
  let now = match input.now() {
    Ok(now) => now,
    Err(err) => return err.into_compile_error().into(),
  };

  let date = now.date();

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

/// Returns the compile date as `&'static str` in `yyyy-MM-dd` format.
///
/// By default, the returned date is in UTC, call `date_str!(local)`
/// to return the local date.
///
/// # Example
///
/// Get the UTC date:
///
/// ```
/// # use time03 as time;
/// const COMPILE_DATE: time::Date = compile_time::date!();
///
/// let year = COMPILE_DATE.year();
/// let month: u8 = COMPILE_DATE.month().into();
/// let day = COMPILE_DATE.day();
/// let date_string = format!("{year:04}-{month:02}-{day:02}");
///
/// assert_eq!(compile_time::date_str!(), date_string);
/// ```
///
/// Get the date for the local time zone:
///
/// ```
/// # use time03 as time;
/// const COMPILE_DATE: time::Date = compile_time::date!(local);
///
/// let year = COMPILE_DATE.year();
/// let month: u8 = COMPILE_DATE.month().into();
/// let day = COMPILE_DATE.day();
/// let date_string = format!("{year:04}-{month:02}-{day:02}");
///
/// assert_eq!(compile_time::date_str!(local), date_string);
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn date_str(input: TokenStream) -> TokenStream {
  use syn::parse_macro_input;
  use time03::format_description;

  use time::TimeInput;

  let input = parse_macro_input!(input as TimeInput);
  let now = match input.now() {
    Ok(now) => now,
    Err(err) => return err.into_compile_error().into(),
  };

  let date = now.date();

  let fmt = format_description::parse_borrowed::<3>("[year]-[month]-[day]").unwrap();
  let date_str = date.format(&fmt).unwrap();

  quote! { #date_str }.into()
}

/// Returns the compile time as [`time::Time`](time03::Time).
///
/// By default, the returned time is in UTC, call `time!(local)`
/// to return the local time.
///
/// # Example
///
/// Get the UTC time:
///
/// ```
/// # use time03 as time;
/// const COMPILE_TIME: time::Time = compile_time::time!();
///
/// let hour = COMPILE_TIME.hour();
/// let minute = COMPILE_TIME.minute();
/// let second = COMPILE_TIME.second();
/// println!("Compiled at {hour:02}:{minute:02}:{second:02}.");
/// ```
///
/// Get the time for the local time zone:
///
/// ```
/// # use time03 as time;
/// const COMPILE_TIME: time::Time = compile_time::time!(local);
///
/// let hour = COMPILE_TIME.hour();
/// let minute = COMPILE_TIME.minute();
/// let second = COMPILE_TIME.second();
/// println!("Compiled at {hour:02}:{minute:02}:{second:02}.");
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn time(input: TokenStream) -> TokenStream {
  use syn::parse_macro_input;

  use time::TimeInput;

  let input = parse_macro_input!(input as TimeInput);
  let now = match input.now() {
    Ok(now) => now,
    Err(err) => return err.into_compile_error().into(),
  };

  let time = now.time();

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

/// Returns the compile time as `&'static str` in `hh:mm:ss` format.
///
/// By default, the returned time is in UTC, call `time_str!(local)`
/// to return the local time.
///
/// # Example
///
/// Get the UTC time:
///
/// ```
/// # use time03 as time;
/// const COMPILE_TIME: time::Time = compile_time::time!();
///
/// let hour = COMPILE_TIME.hour();
/// let minute = COMPILE_TIME.minute();
/// let second = COMPILE_TIME.second();
/// let time_string = format!("{hour:02}:{minute:02}:{second:02}");
///
/// assert_eq!(compile_time::time_str!(), time_string);
/// ```
///
/// Get the time for the local time zone:
///
/// ```
/// # use time03 as time;
/// const COMPILE_TIME: time::Time = compile_time::time!(local);
///
/// let hour = COMPILE_TIME.hour();
/// let minute = COMPILE_TIME.minute();
/// let second = COMPILE_TIME.second();
/// let time_string = format!("{hour:02}:{minute:02}:{second:02}");
///
/// assert_eq!(compile_time::time_str!(local), time_string);
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn time_str(input: TokenStream) -> TokenStream {
  use syn::parse_macro_input;
  use time03::format_description;

  use time::TimeInput;

  let input = parse_macro_input!(input as TimeInput);
  let now = match input.now() {
    Ok(now) => now,
    Err(err) => return err.into_compile_error().into(),
  };

  let time = now.time();

  let fmt = format_description::parse_borrowed::<3>("[hour]:[minute]:[second]").unwrap();
  let time_str = time.format(&fmt).unwrap();

  quote! { #time_str }.into()
}

/// Returns the compile date and time as [`time::OffsetDateTime`](time03::OffsetDateTime).
///
/// By default, the returned datetime is in UTC, call `datetime!(local)`
/// to return the local date and time.
///
/// # Example
///
/// Get the UTC date and time:
///
/// ```
/// # use time03 as time;
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
///
/// Get the date and time for the local time zone:
///
/// ```
/// # use time03 as time;
/// const COMPILE_DATETIME: time::OffsetDateTime = compile_time::datetime!(local);
///
/// let year = COMPILE_DATETIME.year();
/// let month = COMPILE_DATETIME.month();
/// let day = COMPILE_DATETIME.day();
/// let hour = COMPILE_DATETIME.hour();
/// let minute = COMPILE_DATETIME.minute();
/// let second = COMPILE_DATETIME.second();
/// println!("Compiled at {hour:02}:{minute:02}:{second:02} on {month} {day}, {year}.");
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn datetime(input: TokenStream) -> TokenStream {
  use syn::parse_macro_input;

  use time::TimeInput;

  let input = parse_macro_input!(input as TimeInput);
  let now = match input.now() {
    Ok(now) => now,
    Err(err) => return err.into_compile_error().into(),
  };

  let datetime = now;

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

/// Returns the compile date and time as `&'static str` in `yyyy-MM-ddThh:mm:ssZ` format.
///
/// By default, the returned datetime is in UTC, call `datetime_str!(local)`
/// to return the local date and time.
///
/// # Example
///
/// Get the UTC date and time:
///
/// ```
/// const COMPILE_DATE_STRING: &str = compile_time::date_str!();
/// const COMPILE_TIME_STRING: &str = compile_time::time_str!();
///
/// let datetime_string = format!("{COMPILE_DATE_STRING}T{COMPILE_TIME_STRING}Z");
/// assert_eq!(compile_time::datetime_str!(), datetime_string);
/// ```
///
/// Get the date and time for the local time zone:
///
/// ```
/// const COMPILE_DATE_STRING: &str = compile_time::date_str!(local);
/// const COMPILE_TIME_STRING: &str = compile_time::time_str!(local);
///
/// let datetime_string = format!("{COMPILE_DATE_STRING}T{COMPILE_TIME_STRING}Z");
/// assert_eq!(compile_time::datetime_str!(local), datetime_string);
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn datetime_str(input: TokenStream) -> TokenStream {
  use syn::parse_macro_input;
  use time03::format_description;

  use time::TimeInput;

  let input = parse_macro_input!(input as TimeInput);
  let now = match input.now() {
    Ok(now) => now,
    Err(err) => return err.into_compile_error().into(),
  };

  let datetime = now;

  let fmt = format_description::parse_borrowed::<3>("[year]-[month]-[day]T[hour]:[minute]:[second]Z").unwrap();
  let datetime_str = datetime.format(&fmt).unwrap();

  quote! { #datetime_str }.into()
}

/// Returns the compile date and time as UNIX timestamp in seconds.
///
/// # Example
///
/// ```
/// # use time03 as time;
/// const COMPILE_DATETIME: time::OffsetDateTime = compile_time::datetime!();
///
/// assert_eq!(compile_time::unix!(), COMPILE_DATETIME.unix_timestamp());
/// ```
#[cfg(feature = "time")]
#[proc_macro]
pub fn unix(_input: TokenStream) -> TokenStream {
  let datetime = time::utc();

  let unix_timestamp = proc_macro2::Literal::i64_unsuffixed(datetime.unix_timestamp());

  quote! {
    #unix_timestamp
  }
  .into()
}

/// Returns the Rust compiler version as [`semver::Version`].
///
/// # Example
///
/// ```
/// const RUSTC_VERSION: semver::Version = compile_time::rustc_version!();
/// assert_eq!(RUSTC_VERSION, rustc_version::version().unwrap());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version(_item: TokenStream) -> TokenStream {
  match version::rustc() {
    Ok(rustc_version::Version { major, minor, patch, pre, build }) => {
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
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Returns the Rust compiler version as `&'static str`.
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
  match version::rustc() {
    Ok(rustc_version) => {
      let rustc_version_string = rustc_version.to_string();
      quote! { #rustc_version_string }
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Returns the Rust compiler major version as integer literal.
///
/// # Example
///
/// ```
/// const RUSTC_VERSION: semver::Version = compile_time::rustc_version!();
/// assert_eq!(RUSTC_VERSION.major, compile_time::rustc_version_major!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_major(_item: TokenStream) -> TokenStream {
  match version::rustc() {
    Ok(rustc_version) => {
      let major = rustc_version.major;
      proc_macro2::Literal::u64_unsuffixed(major).to_token_stream()
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Returns the Rust compiler minor version as integer literal.
///
/// # Example
///
/// ```
/// const RUSTC_VERSION: semver::Version = compile_time::rustc_version!();
/// assert_eq!(RUSTC_VERSION.minor, compile_time::rustc_version_minor!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_minor(_item: TokenStream) -> TokenStream {
  match version::rustc() {
    Ok(rustc_version) => {
      let minor = rustc_version.minor;
      proc_macro2::Literal::u64_unsuffixed(minor).to_token_stream()
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Returns the Rust compiler patch version as integer literal.
///
/// # Example
///
/// ```
/// const RUSTC_VERSION: semver::Version = compile_time::rustc_version!();
/// assert_eq!(RUSTC_VERSION.minor, compile_time::rustc_version_minor!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_patch(_item: TokenStream) -> TokenStream {
  match version::rustc() {
    Ok(rustc_version) => {
      let patch = rustc_version.patch;
      proc_macro2::Literal::u64_unsuffixed(patch).to_token_stream()
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Returns the Rust compiler pre version as `&'static str`.
///
/// # Example
///
/// ```
/// const RUSTC_VERSION: semver::Version = compile_time::rustc_version!();
/// assert_eq!(RUSTC_VERSION.pre.as_str(), compile_time::rustc_version_pre!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_pre(_item: TokenStream) -> TokenStream {
  match version::rustc() {
    Ok(rustc_version) => {
      let pre = rustc_version.pre.as_str();
      quote! { #pre }
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Returns the Rust compiler build version as a `&'static str`.
///
/// # Example
///
/// ```
/// const RUSTC_VERSION: semver::Version = compile_time::rustc_version!();
/// assert_eq!(RUSTC_VERSION.build.as_str(), compile_time::rustc_version_build!());
/// ```
#[cfg(feature = "version")]
#[proc_macro]
pub fn rustc_version_build(_input: TokenStream) -> TokenStream {
  match version::rustc() {
    Ok(rustc_version) => {
      let build = rustc_version.build.as_str();
      quote! { #build }
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Runs the given command and returns its standard output as a `&'static [u8]`.
///
/// Produces a compile error if the command fails or of the output is not valid UTF-8.
///
/// # Example
///
/// ```
/// const MAGIC_NUMBER: &[u8] = compile_time::command_bytes!("echo", "42");
///
/// assert_eq!(MAGIC_NUMBER, b"42\n");
/// ```
#[proc_macro]
#[cfg(feature = "command")]
pub fn command_bytes(input: TokenStream) -> TokenStream {
  use proc_macro2::Span;
  use syn::{parse_macro_input, LitByteStr};

  use command::CommandInput;

  let input = parse_macro_input!(input as CommandInput);
  match command::output(input) {
    Ok(output) => {
      let output = LitByteStr::new(&output, Span::call_site());
      quote! { #output }
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

/// Runs the given command and returns its standard output as a `&'static str`.
///
/// Produces a compile error if the command fails or of the output is not valid UTF-8.
///
/// # Example
///
/// ```
/// const MAGIC_NUMBER: &str = compile_time::command_str!("echo", "42");
///
/// assert_eq!(MAGIC_NUMBER, "42\n");
/// ```
#[proc_macro]
#[cfg(feature = "command")]
pub fn command_str(input: TokenStream) -> TokenStream {
  use proc_macro2::Span;
  use syn::{parse_macro_input, LitStr};

  use command::CommandInput;

  let input = parse_macro_input!(input as CommandInput);
  match command::output(input) {
    Ok(output) => match String::from_utf8(output) {
      Ok(output_string) => {
        let output = LitStr::new(&output_string, Span::call_site());
        quote! { #output }
      },
      Err(_) => quote! {
        ::core::compile_error!("Command output is not valid UTF-8.")
      },
    },
    Err(err) => err.into_compile_error(),
  }
  .into()
}

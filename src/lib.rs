//! This crate provides macros for getting the compile time, at compile time, either as
//! [`time::Date`](time::Date), [`time::Time`](time::Time),
//! [`time::OffsetDateTime`](time::OffsetDateTime), string, or UNIX timestamp.
//!
//! # Example
//!
//! ```
//! const COMPILE_DATE: time::Date = compile_time::date!();
//! const COMPILE_TIME: time::Time = compile_time::time!();
//! const COMPILE_DATETIME: time::OffsetDateTime = compile_time::datetime!();
//!
//! const COMPILE_DATE_STRING: &str = compile_time::date_str!();
//! const COMPILE_TIME_STRING: &str = compile_time::time_str!();
//! const COMPILE_DATETIME_STRING: &str = compile_time::datetime_str!();
//!
//! // Evaluation is only done once.
//! # std::thread::sleep(std::time::Duration::from_secs(1));
//! assert_eq!(COMPILE_DATETIME, compile_time::datetime!());
//!
//! // Date string is formatted as yyyy-MM-dd.
//! let year = COMPILE_DATETIME.year();
//! let month: u8 = COMPILE_DATETIME.month().into();
//! let day = COMPILE_DATETIME.day();
//! let date_string = format!("{year:04}-{month:02}-{day:02}");
//! assert_eq!(COMPILE_DATE_STRING, date_string);
//!
//! // Time is formatted as hh:mm::ss.
//! let hour = COMPILE_DATETIME.hour();
//! let minute = COMPILE_DATETIME.minute();
//! let second = COMPILE_DATETIME.second();
//! let time_string = format!("{hour:02}:{minute:02}:{second:02}");
//! assert_eq!(COMPILE_TIME_STRING, time_string);
//!
//! // Date-time is formatted as yyyy-MM-ddThh:mm::ssZ.
//! let datetime_string = format!("{date_string}T{time_string}Z");
//! assert_eq!(COMPILE_DATETIME_STRING, datetime_string);
//!
//! // UNIX time in seconds.
//! assert_eq!(COMPILE_DATETIME.unix_timestamp(), compile_time::unix!());
//! #
//! # // Additional sanity check.
//! # let now = time::OffsetDateTime::now_utc();
//! # let yesterday = now.saturating_sub(time::Duration::days(1));
//! # assert!(COMPILE_DATETIME > yesterday);
//! # assert!(COMPILE_DATETIME < now);
//! ```

extern crate proc_macro;

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use time::{macros::format_description, OffsetDateTime};

static COMPILE_TIME: Lazy<OffsetDateTime> = Lazy::new(OffsetDateTime::now_utc);

/// Compile date as `time::Date`.
#[proc_macro]
pub fn date(_item: TokenStream) -> TokenStream {
  let date = COMPILE_TIME.date();

  let year = date.year();
  let month = format_ident!("{}", format!("{:?}", date.month()));
  let day = date.day();

  let output = quote! {
    match ::time::Date::from_calendar_date(#year, ::time::Month::#month, #day) {
      Ok(date) => date,
      _ => ::core::unreachable!(),
    }
  };

  proc_macro::TokenStream::from(output)
}

/// Compile date as `&'static str` in `YYY-MM-DD` format.
#[proc_macro]
pub fn date_str(_item: TokenStream) -> TokenStream {
  let date = COMPILE_TIME.date();

  let fmt = format_description!("[year]-[month]-[day]");
  let date_str = date.format(&fmt).unwrap();

  let output = quote! {
    #date_str
  };

  proc_macro::TokenStream::from(output)
}

/// Compile time as `time::Time`.
#[proc_macro]
pub fn time(_item: TokenStream) -> TokenStream {
  let time = COMPILE_TIME.time();

  let hour = time.hour();
  let minute = time.minute();
  let second = time.second();

  let output = quote! {
    match ::time::Time::from_hms(#hour, #minute, #second) {
      Ok(time) => time,
      _ => ::core::unreachable!(),
    }
  };

  proc_macro::TokenStream::from(output)
}

/// Compile time as `&'static str` in `hh:mm:ss` format.
#[proc_macro]
pub fn time_str(_item: TokenStream) -> TokenStream {
  let time = COMPILE_TIME.time();

  let fmt = format_description!("[hour]:[minute]:[second]");
  let time_str = time.format(&fmt).unwrap();

  let output = quote! {
    #time_str
  };

  proc_macro::TokenStream::from(output)
}

/// Compile date and time as `time::OffsetDateTime`.
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

  let output = quote! {
    ::time::PrimitiveDateTime::new(#date, #time).assume_utc()
  };

  proc_macro::TokenStream::from(output)
}

/// Compile time as `&'static str` in `yyyy-MM-ddThh:mm:ssZ` format.
#[proc_macro]
pub fn datetime_str(_item: TokenStream) -> TokenStream {
  let datetime = *COMPILE_TIME;

  let fmt = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");
  let datetime_str = datetime.format(&fmt).unwrap();

  let output = quote! {
    #datetime_str
  };

  proc_macro::TokenStream::from(output)
}

/// Compile date and time as UNIX timestamp in seconds.
#[proc_macro]
pub fn unix(_item: TokenStream) -> TokenStream {
  let datetime = *COMPILE_TIME;

  let unix_timestamp = proc_macro2::Literal::i64_unsuffixed(datetime.unix_timestamp());

  let output = quote! {
    #unix_timestamp
  };

  proc_macro::TokenStream::from(output)
}

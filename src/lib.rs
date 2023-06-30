extern crate proc_macro;

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use time::{macros::format_description, OffsetDateTime};

static COMPILE_TIME: Lazy<OffsetDateTime> = Lazy::new(|| OffsetDateTime::now_utc());

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

/// Compile time as `&'static str` in `YYYY-MM-DDThh:mm:ssZ` format.
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

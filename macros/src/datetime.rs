use once_cell::sync::Lazy;
use proc_macro2::Span;
use syn::{parse::Parse, Ident, Token};
use time::{OffsetDateTime, UtcOffset};

pub static COMPILE_TIME: Lazy<OffsetDateTime> = Lazy::new(OffsetDateTime::now_utc);

pub fn utc() -> OffsetDateTime {
  *COMPILE_TIME
}

fn local() -> Result<OffsetDateTime, syn::Error> {
  let offset = match UtcOffset::local_offset_at(*COMPILE_TIME) {
    Ok(offset) => offset,
    Err(err) => return Err(syn::Error::new(Span::call_site(), err)),
  };

  Ok(COMPILE_TIME.to_offset(offset))
}

pub struct TimeInput {
  local: bool,
}

impl TimeInput {
  pub fn now(&self) -> Result<OffsetDateTime, syn::Error> {
    if self.local {
      local()
    } else {
      Ok(utc())
    }
  }
}

impl Parse for TimeInput {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let local = if input.is_empty() {
      false
    } else {
      let arg = input.parse::<Ident>()?;

      arg == "local"
    };

    if !input.is_empty() {
      input.parse::<Token![,]>()?;
    }

    if !input.is_empty() {
      return Err(syn::Error::new(input.span(), "Unexpected argument."));
    }

    Ok(Self { local })
  }
}

#[cfg(test)]
mod tests {
  use proc_macro2::TokenStream;
  use quote::quote;

  use super::TimeInput;

  #[test]
  fn parse_no_args() {
    let tokens: TokenStream = quote!().into();

    let input: TimeInput = syn::parse2(tokens).unwrap();

    assert_eq!(input.local, false);
  }

  #[test]
  fn parse_local() {
    let tokens: TokenStream = quote!(local).into();

    let input: TimeInput = syn::parse2(tokens).unwrap();

    assert_eq!(input.local, true);
  }

  #[test]
  fn parse_local_trailing_comma() {
    let tokens: TokenStream = quote!(local).into();

    let input: TimeInput = syn::parse2(tokens).unwrap();

    assert_eq!(input.local, true);
  }
}

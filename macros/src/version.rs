use std::sync::LazyLock;

use proc_macro2::Span;

static RUSTC_VERSION: LazyLock<rustc_version::Result<rustc_version::Version>> = LazyLock::new(rustc_version::version);

pub fn rustc() -> Result<&'static rustc_version::Version, syn::Error> {
  match &*RUSTC_VERSION {
    Ok(version) => Ok(version),
    Err(err) => Err(syn::Error::new(Span::call_site(), format!("Failed to get Rust version: {err}"))),
  }
}

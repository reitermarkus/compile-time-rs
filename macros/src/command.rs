use std::process::Command;

use macro_string::MacroString;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, Token};

pub struct CommandInput {
  command: String,
  args: Vec<String>,
}

impl Parse for CommandInput {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let command = input.parse::<MacroString>()?.eval()?;

    let args = if input.is_empty() {
      Vec::new()
    } else {
      input.parse::<Token![,]>()?;
      let arg_strings = syn::punctuated::Punctuated::<MacroString, Token![,]>::parse_terminated(input)?;

      let mut args = Vec::new();
      for arg_string in arg_strings {
        let arg = arg_string.eval()?;
        args.push(arg);
      }

      args
    };

    Ok(Self { command, args })
  }
}

pub fn output(input: CommandInput) -> Result<Vec<u8>, TokenStream> {
  let CommandInput { command, args } = input;

  let mut cmd = Command::new(&command);
  cmd.args(&args);

  let output = match cmd.output() {
    Ok(output) => output,
    Err(e) => {
      let error_message = format!("Command `{} {}` failed: {}", command, args.join(" "), e);
      return Err(quote! { ::compile_error!(#error_message) }.into());
    },
  };

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let error_message = format!("Command `{} {}` failed:\n{}", command, args.join(" "), stderr);
    return Err(quote! { ::compile_error!(#error_message) }.into());
  }

  Ok(output.stdout)
}

#[cfg(test)]
mod tests {
  use proc_macro2::TokenStream;
  use quote::quote;

  use super::CommandInput;

  #[test]
  fn parse_no_args() {
    let tokens: TokenStream = quote!("command").into();

    let input: CommandInput = syn::parse2(tokens).unwrap();

    assert_eq!(input.command, "command");
    assert!(input.args.is_empty());
  }

  #[test]
  fn parse_no_args_trailing_comma() {
    let tokens: TokenStream = quote!("command",).into();

    let input: CommandInput = syn::parse2(tokens).unwrap();

    assert_eq!(input.command, "command");
    assert!(input.args.is_empty());
  }

  #[test]
  fn parse_no_comma() {
    let tokens: TokenStream = quote!("command", "arg").into();

    let input: CommandInput = syn::parse2(tokens).unwrap();

    assert_eq!(input.command, "command");
    assert_eq!(input.args, ["arg"]);
  }

  #[test]
  fn parse_trailing_comma() {
    let tokens: TokenStream = quote!("command", "arg",).into();

    let input: CommandInput = syn::parse2(tokens).unwrap();

    assert_eq!(input.command, "command");
    assert_eq!(input.args, ["arg"]);
  }
}

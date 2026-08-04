use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  let host = env::var("HOST").unwrap();
  let target = env::var("TARGET").unwrap();

  let mut f = File::create(out_dir.join("constants.rs")).unwrap();
  writeln!(f, r"pub const HOST: &str = {host:?};").unwrap();
  writeln!(f, r"pub const TARGET: &str = {target:?};").unwrap();

  let (host_arch, host) = host.split_once("-").unwrap();
  let (host_vendor, host_os_and_env) = host.split_once("-").unwrap();
  let (host_os, host_env) = host_os_and_env.split_once("-").unwrap_or((host_os_and_env, ""));
  let host_os = if host_os == "darwin" { "macos" } else { host_os };

  // Needed for doc tests.
  println!("cargo::rustc-cfg=host_arch={host_arch:?}");
  println!(r#"cargo::rustc-check-cfg=cfg(host_arch, values("aarch64", "x86_64"))"#);
  println!("cargo::rustc-cfg=host_vendor={host_vendor:?}");
  println!(r#"cargo::rustc-check-cfg=cfg(host_vendor, values("apple", "pc"))"#);
  println!("cargo::rustc-cfg=host_os={host_os:?}");
  println!(r#"cargo::rustc-check-cfg=cfg(host_os, values("macos", "linux", "windows"))"#);
  println!("cargo::rustc-cfg=host_env={host_env:?}");
  println!(r#"cargo::rustc-check-cfg=cfg(host_env, values("gnu", "msvc"))"#);
}

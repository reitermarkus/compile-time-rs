use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  let host = env::var("HOST").unwrap();
  let target = env::var("TARGET").unwrap();

  let mut f = File::create(out_dir.join("constants.rs")).unwrap();
  writeln!(f, r"pub const HOST: &str = {host:?};").unwrap();
  writeln!(f, r"pub const TARGET: &str = {target:?};").unwrap();
}

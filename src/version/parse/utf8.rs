//! Helper module for splitting the first `char` off a `&str` in `const`.
//!
//! See <https://github.com/rust-lang/rust/blob/4e701dc6ba63a40c908a173df269a55e1fd6297e/library/core/src/str/validations.rs>
//! for the original (non-`const`) implementation.

/// Mask of the value bits of a continuation byte.
const CONT_MASK: u8 = 0b0011_1111;

/// Returns the initial codepoint accumulator for the first byte.
/// The first byte is special, only want bottom 5 bits for width 2, 4 bits
/// for width 3, and 3 bits for width 4.
#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
  (byte & (0x7F >> width)) as u32
}

/// Returns the value of `ch` updated with continuation byte `byte`.
#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
  (ch << 6) | (byte & CONT_MASK) as u32
}

/// Reads the first char of a string slice.
#[inline]
pub(super) const fn first_char(s: &str) -> Option<char> {
  let bytes = s.as_bytes();

  // Decode UTF-8
  let x = if bytes.is_empty() {
    return None;
  } else {
    bytes[0]
  };
  if x < 128 {
    // NOTE(unwrap): `x` is an ASCII character.
    return Some(char::from_u32(x as u32).unwrap());
  }

  // Multibyte case follows
  // Decode from a byte combination out of: [[[x y] z] w]
  // NOTE: Performance is sensitive to the exact formulation here
  let init = utf8_first_byte(x, 2);
  let y = bytes[1];
  let mut ch = utf8_acc_cont_byte(init, y);
  if x >= 0xE0 {
    // [[x y z] w] case
    // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
    let z = bytes[2];
    let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
    ch = init << 12 | y_z;
    if x >= 0xF0 {
      // [x y z w] case
      // use only the lower 3 bits of `init`
      let w = bytes[3];
      ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
    }
  }

  // NOTE(unwrap): `x` is a Unicode character.
  Some(char::from_u32(ch).unwrap())
}

pub(super) const fn split_off_first_char(s: &str) -> Option<(char, &str)> {
  if let Some(c) = first_char(s) {
    // NOTE(unwrap): `s` is an UTF-8 string, so splitting it at the length of the first `char` is safe.
    let s = s.split_at_checked(c.len_utf8()).unwrap().1;
    return Some((c, s))
  }

  None
}

#[cfg(test)]
mod tests {
  use super::{first_char, split_off_first_char};

  #[test]
  fn test_first_char() {
    assert_eq!(first_char("💯abc"), Some('💯'));
  }

  #[test]
  fn test_split_off_first_char() {
    assert_eq!(split_off_first_char("💯abc"), Some(('💯', "abc")));
  }
}

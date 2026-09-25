use core::fmt;

use super::{BuildMetadata, Prerelease, Version};

mod utf8;
use utf8::split_off_first_char;
mod error;
use error::{Error, ErrorKind};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum Position {
  Major,
  Minor,
  Patch,
  Pre,
  Build,
}

impl fmt::Display for Position {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str(match self {
      Position::Major => "major version number",
      Position::Minor => "minor version number",
      Position::Patch => "patch version number",
      Position::Pre => "pre-release identifier",
      Position::Build => "build metadata",
    })
  }
}

impl Version {
  #[doc(hidden)]
  pub const fn from_str(text: &'static str) -> Result<Self, Error> {
    if text.is_empty() {
      return Err(Error::new(ErrorKind::Empty));
    }

    let mut pos = Position::Major;
    let (major, text) = match numeric_identifier(text, pos) {
      Ok((major, text)) => (major, text),
      Err(err) => return Err(err),
    };
    let text = match dot(text, pos) {
      Ok(text) => text,
      Err(err) => return Err(err),
    };

    pos = Position::Minor;
    let (minor, text) = match numeric_identifier(text, pos) {
      Ok((minor, text)) => (minor, text),
      Err(err) => return Err(err),
    };
    let text = match dot(text, pos) {
      Ok(text) => text,
      Err(err) => return Err(err),
    };

    pos = Position::Patch;
    let (patch, text) = match numeric_identifier(text, pos) {
      Ok((patch, text)) => (patch, text),
      Err(err) => return Err(err),
    };

    if text.is_empty() {
      return Ok(Version::new(major, minor, patch));
    }

    const fn strip_prefix_char(s: &str, prefix: char) -> Option<&str> {
      if let Some((c, s)) = split_off_first_char(s) {
        if c == prefix {
          Some(s)
        } else {
          None
        }
      } else {
        None
      }
    }

    let (pre, text) = if let Some(text) = strip_prefix_char(text, '-') {
      pos = Position::Pre;
      let (pre, text) = match prerelease_identifier(text) {
        Ok((pre, text)) => (pre, text),
        Err(err) => return Err(err),
      };
      if pre.is_empty() {
        return Err(Error::new(ErrorKind::EmptySegment(pos)));
      }
      (pre, text)
    } else {
      (Prerelease::EMPTY, text)
    };

    let (build, text) = if let Some(text) = strip_prefix_char(text, '+') {
      pos = Position::Build;
      let (build, text) = match build_identifier(text) {
        Ok((build, text)) => (build, text),
        Err(err) => return Err(err),
      };
      if build.is_empty() {
        return Err(Error::new(ErrorKind::EmptySegment(pos)));
      }
      (build, text)
    } else {
      (BuildMetadata::EMPTY, text)
    };

    if let Some(unexpected) = utf8::first_char(text) {
      return Err(Error::new(ErrorKind::UnexpectedCharAfter(pos, unexpected)));
    }

    Ok(Version { major, minor, patch, pre, build })
  }
}

const fn prerelease_identifier(input: &'static str) -> Result<(Prerelease, &'static str), Error> {
  match identifier(input, Position::Pre) {
    Ok((string, rest)) => {
      let identifier = string;
      Ok((Prerelease { identifier }, rest))
    },
    Err(err) => Err(err),
  }
}

const fn build_identifier(input: &'static str) -> Result<(BuildMetadata, &'static str), Error> {
  match identifier(input, Position::Build) {
    Ok((string, rest)) => {
      let identifier = string;
      Ok((BuildMetadata { identifier }, rest))
    },
    Err(err) => Err(err),
  }
}

const fn identifier(input: &str, pos: Position) -> Result<(&str, &str), Error> {
  let mut accumulated_len = 0;
  let mut segment_len = 0;
  let mut segment_has_nondigit = false;

  let bytes = input.as_bytes();
  loop {
    let boundary =
      if accumulated_len + segment_len < bytes.len() { Some(bytes[accumulated_len + segment_len]) } else { None };

    match boundary {
      Some(b'A'..=b'Z') | Some(b'a'..=b'z') | Some(b'-') => {
        segment_len += 1;
        segment_has_nondigit = true;
      },
      Some(b'0'..=b'9') => {
        segment_len += 1;
      },
      boundary => {
        if segment_len == 0 {
          if accumulated_len == 0 && !matches!(boundary, Some(b'.')) {
            return Ok(("", input));
          } else {
            return Err(Error::new(ErrorKind::EmptySegment(pos)));
          }
        }
        if matches!(pos, Position::Pre)
          && segment_len > 1
          && !segment_has_nondigit
          && matches!(utf8::first_char(input.split_at(accumulated_len).1), Some('0'))
        {
          return Err(Error::new(ErrorKind::LeadingZero(pos)));
        }
        accumulated_len += segment_len;
        if matches!(boundary, Some(b'.')) {
          accumulated_len += 1;
          segment_len = 0;
          segment_has_nondigit = false;
        } else {
          return Ok(input.split_at(accumulated_len));
        }
      },
    }
  }
}

const fn dot(input: &str, pos: Position) -> Result<&str, Error> {
  let Some((c, rest)) = utf8::split_off_first_char(input) else {
    return Err(Error::new(ErrorKind::UnexpectedEnd(pos)))
  };

  if c == '.' {
    return Ok(rest)
  }

  Err(Error::new(ErrorKind::UnexpectedCharAfter(pos, c)))
}

const fn numeric_identifier(input: &str, pos: Position) -> Result<(u64, &str), Error> {
  let mut len = 0;
  let mut value = 0u64;

  let bytes = input.as_bytes();
  while len < bytes.len() {
    let digit = bytes[len];

    if digit < b'0' || digit > b'9' {
      break;
    }
    if value == 0 && len > 0 {
      return Err(Error::new(ErrorKind::LeadingZero(pos)));
    }
    match value.checked_mul(10) {
      Some(v) => match v.checked_add((digit - b'0') as u64) {
        Some(sum) => value = sum,
        None => return Err(Error::new(ErrorKind::Overflow(pos))),
      },
      None => return Err(Error::new(ErrorKind::Overflow(pos))),
    }
    len += 1;
  }

  let (_, remaining_input) = input.split_at(len);

  if len > 0 {
    Ok((value, remaining_input))
  } else {
    if let Some(unexpected) = utf8::first_char(remaining_input) {
      Err(Error::new(ErrorKind::UnexpectedChar(pos, unexpected)))
    } else {
      Err(Error::new(ErrorKind::UnexpectedEnd(pos)))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{numeric_identifier, Position, Version};

  #[test]
  fn parse_numeric() {
    let (n, s) = numeric_identifier("888.0.0-beta.1", Position::Major).unwrap();
    assert_eq!(n, 888);
    assert_eq!(s, ".0.0-beta.1");
  }

  #[test]
  fn parse_const() {
    let version = const {
      if let Ok(version) = Version::from_str("1.2.3-beta.1+asdf") {
        version
      } else {
        panic!()
      }
    };
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.pre.as_str(), "beta.1");
    assert_eq!(version.build.as_str(), "asdf");
  }
}

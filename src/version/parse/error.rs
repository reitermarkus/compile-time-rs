use core::fmt;

use super::Position;

pub(super) enum ErrorKind {
  Empty,
  UnexpectedEnd(Position),
  UnexpectedChar(Position, char),
  UnexpectedCharAfter(Position, char),
  LeadingZero(Position),
  Overflow(Position),
  EmptySegment(Position),
}

#[doc(hidden)]
pub struct Error {
  kind: ErrorKind,
}

impl Error {
  pub(super) const fn new(kind: ErrorKind) -> Self {
    Error { kind }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match &self.kind {
      ErrorKind::Empty => formatter.write_str("empty string, expected a semver version"),
      ErrorKind::UnexpectedEnd(pos) => {
        write!(formatter, "unexpected end of input while parsing {}", pos)
      },
      ErrorKind::UnexpectedChar(pos, ch) => {
        write!(formatter, "unexpected character {} while parsing {}", ch, pos,)
      },
      ErrorKind::UnexpectedCharAfter(pos, ch) => {
        write!(formatter, "unexpected character {} after {}", ch, pos,)
      },
      ErrorKind::LeadingZero(pos) => {
        write!(formatter, "invalid leading zero in {}", pos)
      },
      ErrorKind::Overflow(pos) => {
        write!(formatter, "value of {} exceeds u64::MAX", pos)
      },
      ErrorKind::EmptySegment(pos) => {
        write!(formatter, "empty identifier segment in {}", pos)
      },
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("Error(\"")?;
    fmt::Display::fmt(self, formatter)?;
    formatter.write_str("\")")?;
    Ok(())
  }
}

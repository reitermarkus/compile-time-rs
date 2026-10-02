//! A `const` parser for SemVer versions.

use core::fmt;

mod parse;

/// SemVer version as defined by <https://semver.org>.
///
/// This type exists because [`semver::Version`] cannot be `const`-constructed.
/// The API exposed by this type is a subset of [`semver::Version`].
#[allow(missing_docs)]
pub struct Version {
  pub major: u64,
  pub minor: u64,
  pub patch: u64,
  pub pre: Prerelease,
  pub build: BuildMetadata,
}

impl Version {
  pub(self) const fn new(major: u64, minor: u64, patch: u64) -> Self {
    Version { major, minor, patch, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY }
  }
}

impl fmt::Display for Version {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

    if !self.pre.is_empty() {
      write!(f, "-{}", self.pre.as_str())?;
    }

    if !self.build.is_empty() {
      write!(f, "+{}", self.build.as_str())?;
    }

    Ok(())
  }
}

/// Optional pre-release identifier on a version string. This comes after `-` in a SemVer version, as in `1.0.0-alpha.1`.
pub struct Prerelease {
  identifier: &'static str,
}

impl Prerelease {
  pub(self) const EMPTY: Self = Self { identifier: "" };

  #[doc(hidden)]
  pub const fn from_str(identifier: &'static str) -> Self {
    match parse::prerelease_identifier(identifier) {
      Ok((pre_release, rest)) if rest.is_empty() => pre_release,
      _ => panic!("Failed to parse pre-release."),
    }
  }

  /// Returns the build metadata as a string.
  pub const fn as_str(&self) -> &'static str {
    self.identifier
  }

  pub(self) const fn is_empty(&self) -> bool {
    self.identifier.is_empty()
  }
}

/// Optional build metadata identifier. This comes after `+` in a SemVer version, as in `0.8.1+zstd.1.5.0`.
pub struct BuildMetadata {
  identifier: &'static str,
}

impl BuildMetadata {
  pub(self) const EMPTY: Self = Self { identifier: "" };

  #[doc(hidden)]
  pub const fn from_str(identifier: &'static str) -> Self {
    match parse::build_identifier(identifier) {
      Ok((pre_release, rest)) if rest.is_empty() => pre_release,
      _ => panic!("Failed to parse build metadata."),
    }
  }

  /// Returns the build metadata as a string.
  pub const fn as_str(&self) -> &'static str {
    self.identifier
  }

  pub(self) const fn is_empty(&self) -> bool {
    self.identifier.is_empty()
  }
}

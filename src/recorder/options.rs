//! Record fuse iterator, [`Recorder`] options.
//!
//! [`Recorder`]: super::Recorder

mod private_builder {
    pub trait Sealed: Sized {}

    impl Sealed for &mut super::Options {}

    impl Sealed for super::Options {}
}

/// Options builder methods.
///
/// Allow building with either a mutable reference or the value itself.
pub trait Builder: private_builder::Sealed {
    #[must_use]
    fn with_at_least_one(self, at_least_one: bool) -> Self;

    #[must_use]
    fn with_trailing_delimiter(self, trailing_delimiter: bool) -> Self;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
    at_least_one: bool,
    trailing_delimiter: bool,
}

impl Options {
    pub const DEFAULT: Self = Self {
        at_least_one: false,
        trailing_delimiter: false,
    };

    #[inline]
    #[must_use]
    pub const fn at_least_one(&self) -> bool {
        self.at_least_one
    }

    #[inline]
    pub fn at_least_one_mut(&mut self) -> &mut bool {
        &mut self.at_least_one
    }

    #[inline]
    pub fn set_at_least_one(&mut self, at_least_one: bool) {
        self.at_least_one = at_least_one;
    }

    #[inline]
    #[must_use]
    pub const fn trailing_delimiter(&self) -> bool {
        self.trailing_delimiter
    }

    #[inline]
    pub fn trailing_delimiter_mut(&mut self) -> &mut bool {
        &mut self.trailing_delimiter
    }

    #[inline]
    pub fn set_trailing_delimiter(&mut self, trailing_delimiter: bool) {
        self.trailing_delimiter = trailing_delimiter;
    }
}

impl Builder for &mut Options {
    #[inline]
    fn with_at_least_one(self, at_least_one: bool) -> Self {
        self.set_at_least_one(at_least_one);
        self
    }

    #[inline]
    fn with_trailing_delimiter(self, trailing_delimiter: bool) -> Self {
        self.set_trailing_delimiter(trailing_delimiter);
        self
    }
}

impl Builder for Options {
    #[inline]
    fn with_at_least_one(mut self, at_least_one: bool) -> Self {
        self.set_at_least_one(at_least_one);
        self
    }

    #[inline]
    fn with_trailing_delimiter(mut self, trailing_delimiter: bool) -> Self {
        self.set_trailing_delimiter(trailing_delimiter);
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::DEFAULT
    }
}

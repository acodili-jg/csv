mod private_builder {
    pub trait Sealed: Sized {}

    impl Sealed for &mut super::Options {}

    impl Sealed for super::Options {}
}

pub trait Builder: private_builder::Sealed {
    #[must_use]
    fn with_allow_controls(self, allow_controls: bool) -> Self;

    /// # Errors
    ///
    /// See [`Options::set_delimiter`]
    fn with_delimiter(self, delimiter: char) -> Result<Self, Error>;

    #[must_use]
    fn with_line_break(self, line_break: LineBreak) -> Self;

    /// # Errors
    ///
    /// See [`Options::set_quote`]
    fn with_quote(self, quote: char) -> Result<Self, Error>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
    allow_controls: bool,
    delimiter: char,
    line_break: LineBreak,
    quote: char,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::EnumIs)]
pub enum LineBreak {
    #[default]
    Crlf,
    Cr,
    Lf,
    Any,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("options error: {kind}")]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::Display)]
#[non_exhaustive]
pub enum ErrorKind {
    #[strum(to_string = "{0} is a control code character")]
    ControlChar(char),
    #[strum(to_string = "{0} is already used as delimiter")]
    UsedByDelimiter(char),
    #[strum(to_string = "{0} is already used as quote")]
    UsedByQuote(char),
}

impl Error {
    #[inline]
    #[must_use]
    pub const fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl LineBreak {
    #[inline]
    #[must_use]
    pub const fn contains(self, ch: char) -> bool {
        matches!(
            (self, ch),
            (Self::Crlf | Self::Cr | Self::Any, '\r') | (Self::Crlf | Self::Lf | Self::Any, '\n')
        )
    }
}

impl Options {
    pub(super) const DEFAULT: Self = Self {
        allow_controls: false,
        delimiter: ',',
        line_break: LineBreak::Crlf,
        quote: '"',
    };

    #[inline]
    #[must_use]
    pub const fn allow_controls(&self) -> bool {
        self.allow_controls
    }

    #[inline]
    pub fn allow_controls_mut(&mut self) -> &mut bool {
        &mut self.allow_controls
    }

    #[inline]
    pub fn set_allow_controls(&mut self, allow_controls: bool) {
        self.allow_controls = allow_controls;
    }

    #[inline]
    #[must_use]
    pub const fn delimiter(&self) -> char {
        self.delimiter
    }

    /// # Errors
    ///
    /// Setting the delimiter will fail if the given characters is a
    /// [`char::is_control`] or is already used as a [`Self::quote`].
    #[inline]
    pub fn set_delimiter(&mut self, delimiter: char) -> Result<(), Error> {
        if delimiter.is_control() {
            Err(Error::new(ErrorKind::ControlChar(delimiter)))
        } else if delimiter == self.quote {
            return Err(Error::new(ErrorKind::UsedByQuote(delimiter)));
        } else {
            self.delimiter = delimiter;
            Ok(())
        }
    }

    #[inline]
    #[must_use]
    pub const fn line_break(&self) -> LineBreak {
        self.line_break
    }

    #[inline]
    pub fn line_break_mut(&mut self) -> &mut LineBreak {
        &mut self.line_break
    }

    #[inline]
    pub fn set_line_break(&mut self, line_break: LineBreak) {
        self.line_break = line_break;
    }

    #[inline]
    #[must_use]
    pub const fn quote(&self) -> char {
        self.quote
    }

    /// # Errors
    ///
    /// Setting the quote will fail if the given characters is a
    /// [`char::is_control`] or is already used as a [`Self::delimiter`].
    #[inline]
    pub fn set_quote(&mut self, quote: char) -> Result<(), Error> {
        if quote.is_control() {
            Err(Error::new(ErrorKind::ControlChar(quote)))
        } else if quote == self.delimiter {
            return Err(Error::new(ErrorKind::UsedByDelimiter(quote)));
        } else {
            self.quote = quote;
            Ok(())
        }
    }
}

impl Builder for &mut Options {
    #[inline]
    fn with_allow_controls(self, allow_controls: bool) -> Self {
        self.set_allow_controls(allow_controls);
        self
    }

    #[inline]
    fn with_delimiter(self, delimiter: char) -> Result<Self, Error> {
        self.set_delimiter(delimiter)?;
        Ok(self)
    }

    #[inline]
    fn with_line_break(self, line_break: LineBreak) -> Self {
        self.set_line_break(line_break);
        self
    }

    #[inline]
    fn with_quote(self, quote: char) -> Result<Self, Error> {
        self.set_quote(quote)?;
        Ok(self)
    }
}

impl Builder for Options {
    #[inline]
    fn with_allow_controls(mut self, allow_controls: bool) -> Self {
        self.set_allow_controls(allow_controls);
        self
    }

    #[inline]
    fn with_delimiter(mut self, delimiter: char) -> Result<Self, Error> {
        self.set_delimiter(delimiter)?;
        Ok(self)
    }

    #[inline]
    fn with_line_break(mut self, line_break: LineBreak) -> Self {
        self.set_line_break(line_break);
        self
    }

    #[inline]
    fn with_quote(mut self, quote: char) -> Result<Self, Error> {
        self.set_quote(quote)?;
        Ok(self)
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::DEFAULT
    }
}

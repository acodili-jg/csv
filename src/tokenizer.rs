pub mod core;
pub mod options;

use crate::token::Token;
use core::next;
pub use options::Options;
use std::iter::{Enumerate, Peekable};

/// Converting to a csv token iterator.
#[allow(private_bounds)]
pub trait Into
where
    Self: Iterator<Item = char> + Sized,
{
    #[inline]
    fn csv_tokens(self) -> Tokenizer<Self, &'static Options> {
        self.csv_tokens_custom(&Options::DEFAULT)
    }

    #[inline]
    fn csv_tokens_custom<O>(self, options: O) -> Tokenizer<Self, O> {
        Tokenizer {
            iter: self.enumerate().peekable(),
            options,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tokenizer<I, O>
where
    I: Iterator<Item = char>,
{
    iter: Peekable<Enumerate<I>>,
    options: O,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("Parse Token Error at {at}: {kind}")]
pub struct Error {
    at: usize,
    kind: ErrorKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::Display)]
#[non_exhaustive]
pub enum ErrorKind {
    #[strum(
        to_string = "unexpected control code character; either remove it, or allow control codes"
    )]
    Control,
    #[strum(to_string = "unexpected carriage return '\\r'. Possible resolution:
 - complete a line break by appending '\\n'
 - remove the carriage return
 - quote the field containing the carriage return
 - enable cross-platform line breaks; or
 - enable control codes")]
    CarriageReturn,
    #[strum(to_string = "unexpected line feed '\\n'. Possible resolution:
 - complete a line break by prepending '\\r'
 - remove the line feed
 - quote the field containing the line feed
 - enable cross-platform line breaks; or
 - enable control codes")]
    LineFeed,
    #[strum(to_string = "quote ended too early; quotes must enclose the entire field")]
    EarlyQuote,
    #[strum(
        to_string = "quote in unquoted string; a field must be quoted to contain quotes and are escaped by having twos"
    )]
    LateQuote,
}

impl Error {
    #[inline]
    #[must_use]
    pub const fn new(at: usize, kind: ErrorKind) -> Self {
        Self { at, kind }
    }

    #[inline]
    #[must_use]
    pub const fn at(&self) -> &usize {
        &self.at
    }

    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl<I> Into for I where I: Iterator<Item = char> + Sized {}

impl<'a, I> Iterator for Tokenizer<I, &'a Options>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self::next(&mut self.iter, self.options)
    }
}

impl<'a, I> Iterator for Tokenizer<I, &'a mut Options>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self::next(&mut self.iter, self.options)
    }
}

impl<I> Iterator for Tokenizer<I, Options>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self::next(&mut self.iter, &self.options)
    }
}

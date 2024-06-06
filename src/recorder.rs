pub mod core;
pub mod options;

use crate::token::Token;
use core::Position;
pub use options::Options;
use std::iter::{FusedIterator, Peekable};

#[derive(Clone, Debug)]
pub struct Recorder<I, O>
where
    I: Iterator<Item = Token>,
{
    iter: Peekable<I>,
    pos: Position,
    options: O,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("parse field: {kind}")]
pub struct Error {
    at: usize,
    kind: ErrorKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::Display)]
#[non_exhaustive]
pub enum ErrorKind {
    #[strum(to_string = "delimiter at the end of a line. To resolve, either:
 - quote the empty field
 - enable trailing delimiters
 - remove the trailing delimiter")]
    TrailingDelimiter,
    #[strum(
        to_string = "field tokens needs to be separated. Either insert a delimiter or a line break."
    )]
    UndelimitedFields,
}

/// Converting to a csv token iterator.
#[allow(private_bounds)]
pub trait Into
where
    Self: Iterator<Item = Token> + Sized,
{
    fn csv_record(self) -> Recorder<Self, &'static Options> {
        Recorder {
            iter: self.peekable(),
            pos: Position::default(),
            options: &Options::DEFAULT,
        }
    }

    fn csv_record_custom<O>(self, options: O) -> Recorder<Self, O> {
        Recorder {
            iter: self.peekable(),
            pos: Position::default(),
            options,
        }
    }
}

impl<I> Into for I where I: Iterator<Item = Token> {}

impl Error {
    #[inline]
    #[must_use]
    pub const fn new(at: usize, kind: ErrorKind) -> Self {
        Self { at, kind }
    }

    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl<I, O> FusedIterator for Recorder<I, O>
where
    I: Iterator<Item = Token>,
    Self: Iterator,
{
}

impl<'a, I> Iterator for Recorder<I, &'a Options>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        core::next(&mut self.iter, &mut self.pos, self.options)
    }
}

impl<'a, I> Iterator for Recorder<I, &'a mut Options>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        core::next(&mut self.iter, &mut self.pos, self.options)
    }
}

impl<I> Iterator for Recorder<I, Options>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        core::next(&mut self.iter, &mut self.pos, &self.options)
    }
}

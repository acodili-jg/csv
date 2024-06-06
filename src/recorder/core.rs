use std::iter::Peekable;

use crate::token::{Kind, Token};

use super::{Error, ErrorKind, Options};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Position {
    #[default]
    Start,
    Middle,
    End,
}

#[inline]
pub fn next<I>(
    iter: &mut Peekable<I>,
    pos: &mut Position,
    options: &Options,
) -> Option<Result<String, Error>>
where
    I: Iterator<Item = Token>,
{
    match pos {
        Position::Start => next_at_start(iter, pos, options.at_least_one()),
        Position::Middle => next_at_middle(iter, pos, options.trailing_delimiter()),
        Position::End => None,
    }
}

fn next_at_start<I>(
    iter: &mut Peekable<I>,
    pos: &mut Position,
    at_least_one: bool,
) -> Option<Result<String, Error>>
where
    I: Iterator<Item = Token>,
{
    Some(Ok(match iter.peek().map(Token::kind) {
        Some(Kind::Field(_)) => {
            *pos = Position::Middle;
            unsafe { next_field_unchecked(iter) }
        }
        Some(Kind::Delimiter) => {
            *pos = Position::Middle;
            String::new()
        }
        Some(Kind::LineBreak) | None => {
            *pos = Position::End;
            at_least_one.then(String::new)?
        }
    }))
}

fn next_at_middle<I>(
    iter: &mut Peekable<I>,
    pos: &mut Position,
    trailing_delimiter: bool,
) -> Option<Result<String, Error>>
where
    I: Iterator<Item = Token>,
{
    match iter.next() {
        Some(Token {
            idx,
            kind: Kind::Field(_),
        }) => Some(Err(Error::new(idx, ErrorKind::UndelimitedFields))),
        Some(Token {
            idx,
            kind: Kind::Delimiter,
        }) => match iter.peek() {
            Some(Token {
                idx: _,
                kind: Kind::Field(_),
            }) => Some(Ok(unsafe { next_field_unchecked(iter) })),
            Some(Token {
                idx: _,
                kind: Kind::Delimiter,
            }) => Some(Ok(String::new())),
            Some(Token {
                idx: _,
                kind: Kind::LineBreak,
            })
            | None => {
                *pos = Position::End;
                Some(if trailing_delimiter {
                    Ok(String::new())
                } else {
                    Err(Error::new(idx, ErrorKind::TrailingDelimiter))
                })
            }
        },
        Some(Token {
            idx: _,
            kind: Kind::LineBreak,
        })
        | None => {
            *pos = Position::End;
            None
        }
    }
}

unsafe fn next_field_unchecked<I>(iter: &mut I) -> String
where
    I: Iterator<Item = Token>,
{
    unsafe {
        iter.next()
            .unwrap_unchecked()
            .kind
            .try_as_field()
            .unwrap_unchecked()
    }
}

use std::iter::{Enumerate, Peekable};

use crate::token::{Kind, Token};

use super::{options::LineBreak, Error, ErrorKind, Options};

/// Tokenizes one or more characters.
///
/// # Panics
///
/// If `QUOTE == DELIMITER` or if either `QUOTE` or `DELIMITER` is a control
/// code character or equaled either one of the line terminating characters
/// `'\r'` or `'\n'`.
#[inline]
pub fn next<I>(iter: &mut Peekable<Enumerate<I>>, options: &Options) -> Option<Result<Token, Error>>
where
    I: Iterator<Item = char>,
{
    let (idx, ch): (usize, char) = iter.next()?;

    Some(next_some(iter, options, idx, ch))
}

fn next_some<I>(
    iter: &mut Peekable<Enumerate<I>>,
    options: &Options,
    idx: usize,
    ch: char,
) -> Result<Token, Error>
where
    I: Iterator<Item = char>,
{
    #[allow(clippy::if_same_then_else)]
    let kind = if ch == options.quote() {
        next_escaped(iter, options)?
    } else if ch == options.delimiter() {
        Kind::Delimiter
    } else if ch == '\r' {
        next_carriage_return(iter, options, idx, |_| Kind::LineBreak)?
    } else if ch == '\n' {
        next_line_feed(options, idx, |_| Kind::LineBreak)?
    } else {
        next_non_escaped(iter, options, ch)?
    };

    Ok(Token::new(idx, kind))
}

fn next_escaped<I>(iter: &mut Peekable<Enumerate<I>>, options: &Options) -> Result<Kind, Error>
where
    I: Iterator<Item = char>,
{
    let mut buf = String::new();
    while let Some((idx, ch)) = iter.next() {
        if ch == '\r' {
            next_carriage_return(iter, options, idx, |ch| buf.push(ch))?;
        } else if ch == '\n' {
            next_line_feed(options, idx, |ch| buf.push(ch))?;
        } else if ch.is_control() {
            return Err(Error::new(idx, ErrorKind::Control));
        } else if ch == options.quote() {
            if let Some((_, ch)) = iter.next_if(|(_, ch)| *ch == options.quote()) {
                buf.push(ch);
            } else {
                break;
            }
        } else {
            buf.push(ch);
        }
    }
    if let Some((idx, _)) =
        iter.next_if(|(_, ch)| *ch != options.delimiter() && !matches!(ch, '\r' | '\n'))
    {
        return Err(Error::new(idx, ErrorKind::EarlyQuote));
    }
    Ok(Kind::Field(buf))
}

fn next_carriage_return<F, I, R>(
    iter: &mut Peekable<Enumerate<I>>,
    options: &Options,
    idx: usize,
    mut f: F,
) -> Result<R, Error>
where
    F: FnMut(char) -> R,
    I: Iterator<Item = char>,
{
    let r1 = if options.line_break().contains('\r') {
        f('\r')
    } else {
        return Err(Error::new(idx, ErrorKind::CarriageReturn));
    };
    match (options.line_break(), iter.next_if(|(_, ch)| *ch == '\n')) {
        (LineBreak::Crlf | LineBreak::Any, Some((_, ch))) => Ok(f(ch)),
        (LineBreak::Cr | LineBreak::Any, None) => Ok(r1),
        (LineBreak::Lf, _) => unreachable!(),
        (LineBreak::Crlf, None) => Err(Error::new(idx, ErrorKind::CarriageReturn)),
        (LineBreak::Cr, Some((idx, _))) => Err(Error::new(idx, ErrorKind::LineFeed)),
    }
}

fn next_line_feed<F, R>(options: &Options, idx: usize, f: F) -> Result<R, Error>
where
    F: FnOnce(char) -> R,
{
    if options.line_break().contains('\n') {
        Ok(f('\n'))
    } else {
        Err(Error::new(idx, ErrorKind::LineFeed))
    }
}

fn next_non_escaped<I>(
    iter: &mut Peekable<Enumerate<I>>,
    options: &Options,
    first: char,
) -> Result<Kind, Error>
where
    I: Iterator<Item = char>,
{
    let mut buf = first.to_string();
    while let Some((idx, ch)) =
        iter.next_if(|(_, ch)| *ch != options.delimiter() && !options.line_break().contains(*ch))
    {
        if ch == options.quote() {
            return Err(Error::new(idx, ErrorKind::LateQuote));
        }
        if !options.allow_controls() && ch.is_control() {
            let kind = match ch {
                '\r' => ErrorKind::CarriageReturn,
                '\n' => ErrorKind::LineFeed,
                _ch => ErrorKind::Control,
            };
            return Err(Error::new(idx, kind));
        }
        buf.push(ch);
    }
    Ok(Kind::Field(buf))
}

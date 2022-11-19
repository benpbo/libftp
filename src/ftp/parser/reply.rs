use nom::{
    branch::alt,
    bytes::streaming::{tag, take, take_until},
    character::streaming::{char, crlf},
    multi::many_till,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

use super::common::space;
use crate::ftp::{Reply, Text};

pub fn reply(i: &[u8]) -> IResult<&[u8], Reply> {
    let (i, (code, seperator, first_line)) = tuple((code, alt((space, hyphen)), text_line))(i)?;
    let (i, text) = if seperator == '-' {
        let (i, (mut lines, last_line)) =
            many_till(text_line, preceded(pair(tag(code), space), text_line))(i)?;

        lines.insert(0, first_line);

        (i, Text::MultiLine { lines, last_line })
    } else {
        (i, Text::SingleLine { line: first_line })
    };

    Ok((i, Reply { code, text }))
}

fn code(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take(3usize)(i)
}

fn text_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_until("\r\n"), crlf)(i)
}

fn hyphen(i: &[u8]) -> IResult<&[u8], char> {
    char('-')(i)
}

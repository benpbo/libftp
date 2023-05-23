use nom::{
    branch::alt,
    bytes::streaming::{tag, take, take_until},
    character::streaming::char,
    multi::many_till,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

use super::common::{crlf, space};
use crate::reply::{Reply, Text};

pub fn reply(i: &[u8]) -> IResult<&[u8], Reply> {
    let (i, (code, seperator, first_line)) = tuple((code, alt((space, hyphen)), text_line))(i)?;
    let (i, text) = if seperator == '-' {
        let (i, (lines, last_line)) =
            many_till(text_line, preceded(pair(tag(code), space), text_line))(i)?;

        let lines: Vec<Vec<u8>> = [first_line]
            .into_iter()
            .chain(lines)
            .map(|line| line.to_vec())
            .collect();

        (
            i,
            Text::MultiLine {
                lines,
                last_line: last_line.to_vec(),
            },
        )
    } else {
        (
            i,
            Text::SingleLine {
                line: first_line.to_vec(),
            },
        )
    };

    Ok((i, Reply { code, text }))
}

fn code(i: &[u8]) -> IResult<&[u8], [u8; 3]> {
    let (i, code) = take(3usize)(i)?;
    Ok((
        i,
        code.try_into().map_err(|_| {
            nom::Err::Failure(nom::error::Error {
                input: code,
                code: nom::error::ErrorKind::Fail,
            })
        })?,
    ))
}

fn text_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_until("\r\n"), crlf)(i)
}

fn hyphen(i: &[u8]) -> IResult<&[u8], char> {
    char('-')(i)
}

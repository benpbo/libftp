use nom::{bytes::streaming::tag, character::streaming::char, IResult};

pub fn comma(i: &[u8]) -> IResult<&[u8], char> {
    char(',')(i)
}

pub fn space(i: &[u8]) -> IResult<&[u8], char> {
    char(' ')(i)
}

/// Replaces the `nom::character::streaming::crlf` function because
/// of issue #1365 on GitHub where it always returns
/// `Err(nom::Err::Needed(2))` when incomplete.
/// This method returns the correct `nom::Err::Needed` thanks to
/// the `nom::bytes::streaming::tag` function.
pub fn crlf(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("\r\n")(i)
}

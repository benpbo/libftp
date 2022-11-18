use nom::{character::streaming::char, IResult};

pub fn comma(i: &[u8]) -> IResult<&[u8], char> {
    char(',')(i)
}

pub fn space(i: &[u8]) -> IResult<&[u8], char> {
    char(' ')(i)
}

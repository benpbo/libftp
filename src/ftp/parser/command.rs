use std::net::Ipv4Addr;

use nom::{
    branch::alt,
    bytes::streaming::{tag_no_case, take_while1},
    character::streaming::u8,
    combinator::opt,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

use super::common::{comma, crlf, space};
use crate::ftp::{
    command::FormatControl, Command, FileStructureKind, RepresentationTypeKind, TransferModeKind,
};

pub fn command(i: &[u8]) -> IResult<&[u8], Command> {
    let (i, name) = command_name(i)?;
    let name = name.to_ascii_uppercase();

    macro_rules! parse {
        ($command: expr) => {{
            let (i, _) = crlf(i)?;
            Ok((i, $command))
        }};
        ($command: expr, [$parser: expr]) => {{
            let (i, parsed) = opt(delimited(space, $parser, crlf))(i)?;
            Ok((i, $command(parsed.map(|value| value.into()))))
        }};
        ($command: expr, $parser: expr) => {{
            let (i, parsed) = delimited(space, $parser, crlf)(i)?;
            Ok((i, $command(parsed.into())))
        }};
    }

    match &name[..] {
        // USER <SP> <username> <CRLF>
        b"USER" => parse!(Command::UserName, username),
        // PASS <SP> <password> <CRLF>
        b"PASS" => parse!(Command::Password, password),
        // ACCT <SP> <account-information> <CRLF>
        b"ACCT" => parse!(Command::Account, account_information),
        // CWD  <SP> <pathname> <CRLF>
        b"CWD" => parse!(Command::ChangeWorkingDirectory, pathname),
        // CDUP <CRLF>
        b"CDUP" => parse!(Command::ChangeToParentDirectory),
        // SMNT <SP> <pathname> <CRLF>
        b"SNMT" => parse!(Command::StructureMount, pathname),
        // QUIT <CRLF>
        b"QUIT" => parse!(Command::Logout),
        // REIN <CRLF>
        b"REIN" => parse!(Command::Reinitialize),
        // PORT <SP> <host-port> <CRLF>
        b"PORT" => parse!(
            |(address, port)| Command::DataPort(address, port),
            host_port
        ),
        // PASV <CRLF>
        b"PASV" => parse!(Command::Passive),
        // TYPE <SP> <type-code> <CRLF>
        b"TYPE" => parse!(Command::RepresentationType, type_code),
        // STRU <SP> <structure-code> <CRLF>
        b"STRU" => parse!(Command::FileStructure, structure_code),
        // MODE <SP> <mode-code> <CRLF>
        b"MODE" => parse!(Command::TransferMode, mode_code),
        // RETR <SP> <pathname> <CRLF>
        b"RETR" => parse!(Command::Retrieve, pathname),
        // STOR <SP> <pathname> <CRLF>
        b"STOR" => parse!(Command::Store, pathname),
        // STOU <CRLF>
        b"STOU" => parse!(Command::StoreUnique),
        // APPE <SP> <pathname> <CRLF>
        b"APPE" => parse!(Command::Append, pathname),
        // ALLO <SP> <decimal-integer>
        //     [<SP> R <SP> <decimal-integer>] <CRLF>
        b"ALLO" => parse!(
            |(a, b)| Command::Allocate(a, b),
            pair(
                decimal_integer,
                opt(preceded(
                    tuple((space, tag_no_case(b"R"), space)),
                    decimal_integer
                )),
            )
        ),
        // REST <SP> <marker> <CRLF>
        b"REST" => parse!(Command::Restart, marker),
        // RNFR <SP> <pathname> <CRLF>
        b"RNFR" => parse!(Command::RenameFrom, pathname),
        // RNTO <SP> <pathname> <CRLF>
        b"RNTO" => parse!(Command::RenameTo, pathname),
        // ABOR <CRLF>
        b"ABOR" => parse!(Command::Abort),
        // DELE <SP> <pathname> <CRLF>
        b"DELE" => parse!(Command::Delete, pathname),
        // RMD  <SP> <pathname> <CRLF>
        b"RMD" => parse!(Command::RemoveDirectory, pathname),
        // MKD  <SP> <pathname> <CRLF>
        b"MKD" => parse!(Command::MakeDirectory, pathname),
        // PWD  <CRLF>
        b"PWD" => parse!(Command::PrintWorkingDirectory),
        // LIST [<SP> <pathname>] <CRLF>
        b"LIST" => parse!(Command::List, [pathname]),
        // NLST [<SP> <pathname>] <CRLF>
        b"NLST" => parse!(Command::NameList, [pathname]),
        // SITE <SP> <string> <CRLF>
        b"SITE" => parse!(Command::SiteParameters, string),
        // SYST <CRLF>
        b"SYST" => parse!(Command::System),
        // STAT [<SP> <pathname>] <CRLF>
        b"STAT" => parse!(Command::Status, [pathname]),
        // HELP [<SP> <string>] <CRLF>
        b"HELP" => parse!(Command::Help, [string]),
        // NOOP <CRLF>
        b"NOOP" => parse!(Command::Noop),
        _ => {
            unreachable!("All command name variants are specified by the `command_name()` function")
        }
    }
}

fn command_name(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        // Three character commands need to come first for
        // more indicitive `Needed` value when incomplete.
        tag_no_case("CWD"),
        tag_no_case("MKD"),
        tag_no_case("PWD"),
        tag_no_case("RMD"),
        tag_no_case("ABOR"),
        tag_no_case("ACCT"),
        tag_no_case("ALLO"),
        tag_no_case("APPE"),
        tag_no_case("CDUP"),
        tag_no_case("DELE"),
        tag_no_case("HELP"),
        tag_no_case("LIST"),
        tag_no_case("MODE"),
        tag_no_case("NLST"),
        tag_no_case("NOOP"),
        tag_no_case("PASS"),
        tag_no_case("PASV"),
        tag_no_case("PORT"),
        tag_no_case("QUIT"),
        tag_no_case("REIN"),
        alt((
            tag_no_case("REST"),
            tag_no_case("RETR"),
            tag_no_case("RNFR"),
            tag_no_case("RNTO"),
            tag_no_case("SITE"),
            tag_no_case("SMNT"),
            tag_no_case("STAT"),
            tag_no_case("STOR"),
            tag_no_case("STOU"),
            tag_no_case("STRU"),
            tag_no_case("SYST"),
            tag_no_case("TYPE"),
            tag_no_case("USER"),
        )),
    ))(i)
}

// <username> ::= <string>
fn username(i: &[u8]) -> IResult<&[u8], &[u8]> {
    string(i)
}

// <password> ::= <string>
fn password(i: &[u8]) -> IResult<&[u8], &[u8]> {
    string(i)
}

// <account-information> ::= <string>
fn account_information(i: &[u8]) -> IResult<&[u8], &[u8]> {
    string(i)
}

// <string> ::= <char> | <char><string>
fn string(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_char)(i)
}

// <char> ::= any of the 128 ASCII characters except <CR> and <LF>
fn is_char(c: u8) -> bool {
    c.is_ascii() && c != b'\r' && c != b'\n'
}

// <marker> ::= <pr-string>
fn marker(i: &[u8]) -> IResult<&[u8], &[u8]> {
    pr_string(i)
}

// <pr-string> ::= <pr-char> | <pr-char><pr-string>
fn pr_string(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_pr_char)(i)
}

// <pr-char> ::= printable characters, any ASCII code 33 through 126
fn is_pr_char(c: u8) -> bool {
    c.is_ascii_graphic()
}

// <byte-size> ::= <number>
fn byte_size(i: &[u8]) -> IResult<&[u8], u8> {
    number(i)
}

// <host-port> ::= <host-number>,<port-number>
fn host_port(i: &[u8]) -> IResult<&[u8], (Ipv4Addr, u16)> {
    let (i, (ip, port)) = separated_pair(host_number, comma, port_number)(i)?;
    Ok((i, (ip, port)))
}

// <host-number> ::= <number>,<number>,<number>,<number>
fn host_number(i: &[u8]) -> IResult<&[u8], Ipv4Addr> {
    let (i, ((a, b), (c, d))) = separated_pair(
        separated_pair(number, comma, number),
        comma,
        separated_pair(number, comma, number),
    )(i)?;

    Ok((i, Ipv4Addr::new(a, b, c, d)))
}

// <port-number> ::= <number>,<number>
fn port_number(i: &[u8]) -> IResult<&[u8], u16> {
    let (i, (a, b)) = separated_pair(number, comma, number)(i)?;
    Ok((i, u16::from_be_bytes([a, b])))
}

// <number> ::= any decimal integer 1 through 255
fn number(i: &[u8]) -> IResult<&[u8], u8> {
    u8(i)
}

// <form-code> ::= N | T | C
fn form_code(i: &[u8]) -> IResult<&[u8], FormatControl> {
    let (i, code) = alt((tag_no_case(b"N"), tag_no_case(b"T"), tag_no_case(b"C")))(i)?;

    Ok((
        i,
        match code {
            b"n" | b"N" => FormatControl::NonPrint,
            b"t" | b"T" => FormatControl::Telnet,
            b"c" | b"C" => FormatControl::Carriage,
            _ => unreachable!("All options should be exhausted by the previous parser"),
        },
    ))
}

/// <type-code> ::= A [<sp> <form-code>]
///               | E [<sp> <form-code>]
///               | I
///               | L <sp> <byte-size>
fn type_code(i: &[u8]) -> IResult<&[u8], RepresentationTypeKind> {
    let (i, code) = alt((
        tag_no_case(b"A"),
        tag_no_case(b"E"),
        tag_no_case(b"I"),
        tag_no_case(b"L"),
    ))(i)?;

    Ok(match code {
        b"a" | b"A" => {
            let (i, form) = opt(preceded(space, form_code))(i)?;
            (i, RepresentationTypeKind::Ascii(form))
        }
        b"e" | b"E" => {
            let (i, form) = opt(preceded(space, form_code))(i)?;
            (i, RepresentationTypeKind::Ebcdic(form))
        }
        b"i" | b"I" => (i, RepresentationTypeKind::Image),
        b"l" | b"L" => {
            let (i, size) = preceded(space, byte_size)(i)?;
            (i, RepresentationTypeKind::LocalByte(size))
        }
        _ => unreachable!("All options should be exhausted by the previous parser"),
    })
}

// <structure-code> ::= F | R | P
fn structure_code(i: &[u8]) -> IResult<&[u8], FileStructureKind> {
    let (i, code) = alt((tag_no_case(b"F"), tag_no_case(b"R"), tag_no_case(b"P")))(i)?;

    Ok((
        i,
        match code {
            b"f" | b"F" => FileStructureKind::File,
            b"r" | b"R" => FileStructureKind::Record,
            b"p" | b"P" => FileStructureKind::Page,
            _ => unreachable!("All options should be exhausted by the previous parser"),
        },
    ))
}

// <mode-code> ::= S | B | C
fn mode_code(i: &[u8]) -> IResult<&[u8], TransferModeKind> {
    let (i, code) = alt((tag_no_case(b"S"), tag_no_case(b"B"), tag_no_case(b"C")))(i)?;

    Ok((
        i,
        match code {
            b"s" | b"S" => TransferModeKind::Stream,
            b"b" | b"B" => TransferModeKind::Block,
            b"c" | b"C" => TransferModeKind::Compressed,
            _ => unreachable!("All options should be exhausted by the previous parser"),
        },
    ))
}

// <pathname> ::= <string>
fn pathname(i: &[u8]) -> IResult<&[u8], &[u8]> {
    string(i)
}

// <decimal-integer> ::= any decimal integer
fn decimal_integer(i: &[u8]) -> IResult<&[u8], i64> {
    nom::number::streaming::be_i64(i)
}

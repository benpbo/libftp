use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};
use std::num::NonZeroUsize;

use libftp::reply::Text;
use libftp::{parser::parse_reply, reply::Reply};
use nom;

type ClientResult<T, E> = Result<T, ClientError<E>>;

pub struct Client {
    data: BufReader<TcpStream>,
}

impl Client {
    pub fn connect(addr: &SocketAddr) -> ClientResult<(Self, Text), ConnectionErrorReplyCode> {
        let stream = TcpStream::connect(addr).map_err(|err| ClientError::Connection(err))?;
        let reader = BufReader::new(stream);
        let mut instance = Self { data: reader };

        let reply = instance.read_reply()?;
        return match reply.code {
            [b'1', b'2', b'0'] => Err(ClientError::Reply {
                code: ConnectionErrorReplyCode::NotReady,
                text: reply.text,
            }),
            [b'2', b'2', b'0'] => Ok((instance, reply.text)),
            [b'4', b'2', b'1'] => Err(ClientError::Reply {
                code: ConnectionErrorReplyCode::NotAvailable,
                text: reply.text,
            }),
            _ => Err(ClientError::Read(ClientReadError::UnexpectedReply(reply))),
        };
    }

    fn read_reply(&mut self) -> Result<Reply, ClientReadError> {
        let (consumed, reply) = {
            let buffer = self
                .data
                .fill_buf()
                .map_err(|err| ClientReadError::Io(err))?;
            let (unparsed, reply) = parse_reply(buffer).map_err(|err| match err {
                nom::Err::Incomplete(needed) => ClientReadError::BufferTooSmall(match needed {
                    nom::Needed::Unknown => None,
                    nom::Needed::Size(size) => Some(size),
                }),
                nom::Err::Error(inner) => ClientReadError::InvalidReply(inner.input.to_vec()),
                nom::Err::Failure(inner) => ClientReadError::InvalidReply(inner.input.to_vec()),
            })?;

            (buffer.len() - unparsed.len(), reply)
        };

        self.data.consume(consumed);

        Ok(reply)
    }
}

#[derive(Debug)]
pub enum ConnectionErrorReplyCode {
    NotReady = 120,
    NotAvailable = 421,
}

#[derive(Debug)]
pub enum ClientError<C> {
    // Received a valid reply that implies an error
    Reply { code: C, text: Text },
    // An error occured while reading a reply
    Read(ClientReadError),
    // An error occured while sending a command
    Write(ClientWriteError),
    // Unable to connect to the host
    Connection(std::io::Error),
}

#[derive(Debug)]
pub enum ClientReadError {
    // Received a reply that was unexpected in the current context
    UnexpectedReply(Reply),
    // Received an invalid reply from the server
    InvalidReply(Vec<u8>),
    // Client's internal buffer is to small to read the full response
    BufferTooSmall(Option<NonZeroUsize>),
    // An IO error occured while reading a reply
    Io(std::io::Error),
}

impl<C> From<ClientReadError> for ClientError<C> {
    fn from(v: ClientReadError) -> Self {
        ClientError::Read(v)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ClientWriteError {
    // An IO error occured while writing a command
    Io(std::io::Error),
}

impl<C> From<ClientWriteError> for ClientError<C> {
    fn from(v: ClientWriteError) -> Self {
        Self::Write(v)
    }
}

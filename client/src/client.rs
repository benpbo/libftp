use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};

use libftp::command::Command;
use libftp::reply::Text;
use libftp::serializer::{CommandSerializer, Serializer};
use libftp::{parser::parse_reply, reply::Reply};
use nom;

type ClientResult<T, E> = Result<T, ClientError<E>>;

pub struct Client {
    reader: BufReader<TcpStream>,
    writer: CommandSerializer<TcpStream>,
}

impl Client {
    pub fn connect(addr: &SocketAddr) -> ClientResult<(Self, Text), ConnectionErrorReplyCode> {
        let stream = TcpStream::connect(addr).map_err(|err| ClientError::Connection(err))?;
        let (reader, writer) =
            Client::read_write_pair(stream).map_err(|err| ClientError::Connection(err))?;
        let mut instance = Self { reader, writer };

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

    pub fn login(&mut self, username: &str) -> ClientResult<Text, LoginErrorReplyCode> {
        let user = Command::UserName(username.into());
        self.writer
            .serialize(&user)
            .map_err(|err| ClientWriteError::Io(err))?;

        let reply = self.read_reply()?;
        match reply.code {
            [b'2', b'3', b'0'] => Ok(reply.text),
            [b'3', b'3', b'1'] => Err(ClientError::Reply {
                code: LoginErrorReplyCode::RequirePassword,
                text: reply.text,
            }),
            [b'3', b'3', b'2'] => Err(ClientError::Reply {
                code: LoginErrorReplyCode::RequireAccount,
                text: reply.text,
            }),
            [b'5', b'3', b'0'] => Err(ClientError::Reply {
                code: LoginErrorReplyCode::NotLoggedIn,
                text: reply.text,
            }),
            _ => Err(ClientReadError::UnexpectedReply(reply).into()),
        }
    }

    fn read_write_pair(
        stream: TcpStream,
    ) -> std::io::Result<(BufReader<TcpStream>, CommandSerializer<TcpStream>)> {
        let cloned = stream.try_clone()?;
        Ok((BufReader::new(stream), CommandSerializer::new(cloned)))
    }

    fn read_reply(&mut self) -> Result<Reply, ClientReadError> {
        let mut reply_buffer: Vec<u8> = Vec::new();
        loop {
            let read_amount = {
                let buffer = self.reader.fill_buf()?;
                reply_buffer.extend_from_slice(buffer);
                buffer.len()
            };

            return match parse_reply(&reply_buffer) {
                Ok((unparsed, reply)) => {
                    self.reader.consume(read_amount - unparsed.len());
                    Ok(reply)
                }
                Err(nom::Err::Incomplete(_)) => {
                    self.reader.consume(read_amount);
                    continue;
                }
                Err(nom::Err::Error(inner) | nom::Err::Failure(inner)) => {
                    Err(ClientReadError::InvalidReply(inner.input.into()))
                }
            };
        }
    }
}

#[derive(Debug)]
pub enum ConnectionErrorReplyCode {
    NotReady = 120,
    NotAvailable = 421,
}

#[derive(Debug)]
pub enum LoginErrorReplyCode {
    RequirePassword = 331,
    RequireAccount = 332,
    NotLoggedIn = 530,
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
    // An IO error occured while reading a reply
    Io(std::io::Error),
}

impl<C> From<ClientReadError> for ClientError<C> {
    fn from(v: ClientReadError) -> Self {
        ClientError::Read(v)
    }
}

impl From<std::io::Error> for ClientReadError {
    fn from(value: std::io::Error) -> Self {
        ClientReadError::Io(value)
    }
}

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

use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};
use std::num::NonZeroUsize;

use libftp::{parser::parse_reply, reply::Reply};
use nom;

pub enum ClientError {
    // Corresponds to 120 reponse from server
    NotReady(Reply),
    // Corresponds to 421 response from server
    ServiceNotAvailable,
    // Received a reply that was unexpected in the current context
    UnexpectedReply(Reply),
    // Received an invalid reply from the server
    InvalidReply(Vec<u8>),
    // Client's internal buffer is to small to read the full response
    BufferTooSmall(Option<NonZeroUsize>),
    // An IO error occured
    Io(std::io::Error),
}

pub struct Client {
    data: BufReader<TcpStream>,
}

impl Client {
    pub fn connect(addr: &SocketAddr) -> Result<Self, ClientError> {
        let stream = TcpStream::connect(addr).map_err(|err| ClientError::Io(err))?;
        let reader = BufReader::new(stream);
        let mut instance = Self { data: reader };

        let reply = instance.read_reply()?;
        match reply.code {
            [b'1', b'2', b'0'] => todo!("Parse 'nnn' minutes from response"),
            [b'2', b'2', b'0'] => Ok(instance),
            [b'4', b'2', b'1'] => Err(ClientError::ServiceNotAvailable),
            _ => Err(ClientError::UnexpectedReply(reply)),
        }
    }

    fn read_reply(&mut self) -> Result<Reply, ClientError> {
        let (consumed, reply) = {
            let buffer = self.data.fill_buf().map_err(|err| ClientError::Io(err))?;
            let (unparsed, reply) = parse_reply(buffer).map_err(|err| match err {
                nom::Err::Incomplete(needed) => ClientError::BufferTooSmall(match needed {
                    nom::Needed::Unknown => None,
                    nom::Needed::Size(size) => Some(size),
                }),
                nom::Err::Error(inner) => ClientError::InvalidReply(inner.input.to_vec()),
                nom::Err::Failure(inner) => ClientError::InvalidReply(inner.input.to_vec()),
            })?;

            (buffer.len() - unparsed.len(), reply)
        };

        self.data.consume(consumed);

        Ok(reply)
    }
}

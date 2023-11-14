use std::net::{TcpStream, SocketAddr};

pub struct Client {
    #[allow(dead_code)]
    stream: TcpStream
}

impl Client {
    pub fn connect(addr: &SocketAddr) -> std::io::Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(addr)?
        })
    }
}


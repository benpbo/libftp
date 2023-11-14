use std::net::{SocketAddr, TcpListener};

use clap::Parser;

mod proxy;

#[derive(Parser)]
struct Cli {
    // Address to listen for incoming connections.
    host: SocketAddr,
    // Target to forward commands to and receive replies from.
    target: SocketAddr,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let listener = TcpListener::bind(args.host)?;
    for stream in listener.incoming() {
        proxy::proxy_connection(stream?, &args.target)?;
    }

    Ok(())
}

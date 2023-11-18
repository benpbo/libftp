use std::net::SocketAddr;

use clap::Parser;
use client::{Client, ClientError};
use libftp::reply::Text;

mod client;

#[derive(Parser)]
struct Cli {
    // Address to listen for incoming connections.
    host: SocketAddr,
}

fn main() {
    let args = Cli::parse();
    let _client = match Client::connect(&args.host) {
        Ok((client, welcome_message)) => {
            match welcome_message {
                Text::SingleLine { line } => println!("{}", String::from_utf8(line).unwrap()),
                Text::MultiLine { lines, last_line } => {
                    for line in lines {
                        println!("{}", String::from_utf8(line).unwrap())
                    }

                    println!("{}", String::from_utf8(last_line).unwrap())
                }
            }
            client
        }
        Err(ClientError::ServiceNotAvailable) => {
            println!("Service not available");
            return;
        }
        Err(_) => {
            println!("An error occured while trying to connect");
            return;
        }
    };
}

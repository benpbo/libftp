use clap::Parser;
use client::Client;
use libftp::reply::Text;
use std::net::SocketAddr;

mod client;

#[derive(Parser)]
struct Cli {
    // Address to listen for incoming connections.
    host: SocketAddr,
}

fn main() {
    let args = Cli::parse();
    let _client = match Client::connect(&args.host) {
        Ok((client, text)) => {
            print_text(text);
            client
        }
        Err(client::ClientError::Reply { text, .. }) => {
            print_text(text);
            return;
        }
        Err(_) => {
            println!("An error occured while trying to connect");
            return;
        }
    };
}

fn print_text(text: Text) {
    match text {
        Text::SingleLine { line } => println!("{}", String::from_utf8(line).unwrap()),
        Text::MultiLine { lines, last_line } => {
            for line in lines {
                println!("{}", String::from_utf8(line).unwrap())
            }

            println!("{}", String::from_utf8(last_line).unwrap())
        }
    }
}

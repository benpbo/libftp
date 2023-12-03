use clap::Parser;
use client::Client;
use libftp::reply::Text;
use std::{io::Write, net::SocketAddr};

mod client;

#[derive(Parser)]
struct Cli {
    // Address to listen for incoming connections.
    host: SocketAddr,
}

fn main() {
    let args = Cli::parse();
    let mut client = match Client::connect(&args.host) {
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

    print!("User: ");
    std::io::stdout().flush().unwrap();
    let stdin = std::io::stdin();
    let username = {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input
    };

    let login_text = client.login(&username).unwrap();
    println!("Logged in as {username}");
    print_text(login_text);
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

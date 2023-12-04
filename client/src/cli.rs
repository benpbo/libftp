use crate::client::{Client, ClientError};
use clap::Parser;
use libftp::reply::Text;
use std::{io::Write, net::SocketAddr};

#[derive(Parser)]
pub struct Cli {
    // Address to listen for incoming connections.
    host: SocketAddr,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn repl(&self) -> std::io::Result<()> {
        let mut client = match Client::connect(&self.host) {
            Ok((client, text)) => {
                print_text(text);
                client
            }
            Err(ClientError::Reply { text, .. }) => {
                print_text(text);
                return Ok(());
            }
            Err(_) => {
                println!("An error occured while trying to connect");
                return Ok(());
            }
        };

        print!("User: ");
        std::io::stdout().flush()?;
        let stdin = std::io::stdin();
        let username = {
            let mut input = String::new();
            stdin.read_line(&mut input)?;
            input
        };

        match client.login(&username) {
            Ok(login_text) => {
                println!("Logged in as {username}");
                print_text(login_text);
            }
            Err(_) => todo!(),
        }

        Ok(())
    }
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

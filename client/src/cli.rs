use crate::client::{Client, ClientError};
use clap::Parser;
use libftp::reply::Text;
use std::net::SocketAddr;

#[derive(Parser)]
pub struct Cli {
    // Address to listen for incoming connections.
    host: SocketAddr,

    // Username to login with
    #[clap(long)]
    username: Option<String>,

    // Password to login with together with the username
    #[clap(long)]
    password: Option<String>,
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
            Err(e) => {
                print_error(e);
                return Ok(());
            }
        };

        match (&self.username, &self.password) {
            (None, _) => (),
            (Some(username), None) => match client.login(&username) {
                Ok(login_text) => {
                    println!("Logged in as {username}");
                    print_text(login_text);
                }
                Err(e) => print_error(e),
            },
            (Some(username), Some(password)) => {
                match client.login_with_password(&username, &password) {
                    Ok(login_text) => {
                        println!("Logged in as {username}");
                        print_text(login_text);
                    }
                    Err(e) => print_error(e),
                }
            }
        };

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

fn print_error<C>(error: ClientError<C>) {
    match error {
        ClientError::Reply { text, .. } => {
            println!("Received an erroneous reply");
            print_text(text);
        }
        ClientError::Read(e) => {
            println!("An error occured while trying to read response");
            dbg!(e);
        }
        ClientError::Write(e) => {
            println!("An error occured while trying to write to server");
            dbg!(e);
        }
        ClientError::Connection(e) => {
            println!("An error occured while trying to connect");
            dbg!(e);
        }
    }
}

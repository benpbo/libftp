use cli::Cli;

mod cli;
mod client;

fn main() -> std::io::Result<()> {
    let cli = Cli::new();
    cli.repl()
}

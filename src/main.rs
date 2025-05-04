use clap::{Parser, Subcommand};
use subcommand::Runnable;
mod constants;
mod subcommand;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
}

fn main() {
    let cli = Cli::parse();

    let cmd = match cli.command {
        Commands::Init => subcommand::init::InitCommand::new(),
    };
    match cmd.run() {
        Ok(s) => {
            println!("Success: {s}");
        }
        Err(e) => {
            let message = e.to_string();
            println!("Error: {message}");
        }
    }
}

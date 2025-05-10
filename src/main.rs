use crate::command::RgitCommand;
use clap::{Parser, Subcommand};
mod constants;
mod command;
mod result;
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
    let result = cmd.run();
    println!("{}", result);
}

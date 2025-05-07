use clap::{Parser, Subcommand};
use constants::RgitCommand;
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
    let results = cmd.run();
    for result in &results {
        println!("{}", result);
    }
}

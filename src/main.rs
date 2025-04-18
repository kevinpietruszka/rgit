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

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    let cmd = match cli.command {
        Commands::Init => { subcommand::init::InitCommand::new() }
    };
    cmd.run()
}

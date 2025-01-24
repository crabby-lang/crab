mod core;
mod commands;
mod authentication;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "crab", about = "Crab Builder for Crabby")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { name: Option<String> },
    New { name: String },
    Build,
    Run,
    Clean,
    Publish { username: String, password: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::init(name),
        Commands::New { name } => commands::new(name),
        Commands::Build => commands::build(),
        Commands::Run => commands::run(),
        Commands::Clean => commands::clean(),
        Commands::Publish { username, password } => {
            if let Err(e) = authentication::publish(&username, &password).await {
                eprintln!("Publish failed: {}", e);
            }
        }
    }
}

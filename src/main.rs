mod authentication;
mod commands;
mod core;
mod error;
mod ui;

use clap::{Parser, Subcommand};
use error::CrabResult;
use std::process;

#[derive(Parser)]
#[command(name = "crab", about = "Crab Builder for Crabby")]
#[command(author = "Crabby Contributors")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        name: Option<String>,
    },
    New {
        name: String,
    },
    Build,
    Run,
    Clean,
    Test,
    Watch,
    Publish {
        username: String,
        password: String,
    },
    Login {
        username: String,
        password: String,
    },
    Logout,
}

#[tokio::main]
async fn main() {
    ui::print_header();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { name } => commands::init(name),
        Commands::New { name } => commands::new(name),
        Commands::Build => commands::build(),
        Commands::Run => commands::run(),
        Commands::Clean => commands::clean(),
        Commands::Test => commands::test(),
        Commands::Watch => commands::watch(),
        Commands::Publish { username, password } => {
            authentication::publish(&username, &password).await
        }
        Commands::Login { username, password } => {
            authentication::login(&username, &password).await.map(|_| ())
        }
        Commands::Logout => authentication::logout().await,
    };

    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            ui::print_error(&format!("{}", e));
            process::exit(e.exit_code());
        }
    }
}

// Commands that should be available:
// List
// Add
// List gone
// List to be done
// List to be done or done in x min

// Other:
// Save in files
// notifications

use clap::{Parser, Subcommand};

/// A CLI for remembering what you need to do today
#[derive(Debug, Parser)]
#[command(name = "todayiwill")]
#[command(version, about = "A CLI for remembering what you need to do today", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add appointment
    Add,
    /// List the appointments to come
    List,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add => {
            println!("Add action to be implemented.")
        },
        Commands::List => {
            println!("List action to be implemented.")
        }
    }
}

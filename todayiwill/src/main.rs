// Commands that should be available:
// List
// Add
// List gone
// List to be done
// List to be done or done in x min

// Other:
// Save in files
// notifications

use appointment::{add, helper, Appointment, AppointmentTime, Config};
use clap::{Parser, Subcommand};
extern crate dirs;

mod appointment;

use crate::appointment::list;

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
    Add {
        /// Appointment description
        #[arg(short, long)]
        description: String,

        /// Appointment time
        #[arg(short, long)]
        time: String,
    },
    /// List the appointments to come
    List,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add { description, time } => {
            let result = helper::parse_time(&time);
            let (hour, minutes) = match result {
                Some((hour, minutes)) => (hour, minutes),
                None => {
                    println!("You entered a non-valid time.");
                    return;
                }
            };
            add::add_appointment(
                Appointment::new(description, AppointmentTime::new(hour, minutes)),
                Config::default(),
            )
        }
        Commands::List => list::display_list(Config::default()),
    }
}

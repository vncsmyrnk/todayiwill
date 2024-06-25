use appointment::{add, clear, helper, Appointment, AppointmentTime, Config};
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
    /// Clear all the appointments added until now
    Clear,
    /// List the appointments to come
    List,
}

fn main() {
    let args = Cli::parse();
    let config = Config::default();

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

            let appointment_time = match AppointmentTime::new(hour, minutes) {
                Ok(at) => at,
                Err(error) => {
                    println!("Appointment time invalid. {}", error);
                    return;
                }
            };
            add::add_appointment(
                Appointment::new(description, appointment_time),
                config,
            )
        }
        Commands::List => list::display_list(config),
        Commands::Clear => clear::clear_appointments(config),
    }
}

use std::process;

use clap::{Parser, Subcommand};

extern crate chrono;
extern crate dirs;

mod appointment;

use appointment::{add, clear, helper, list, Appointment, AppointmentTime, Config};

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
    List {
        /// Current time, defaults to system time
        #[arg(short, long, value_parser=AppointmentTime::from, default_value_t=AppointmentTime::now())]
        current_time: AppointmentTime,

        /// If informed, all appointments are retrieved
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },
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
                    process::exit(1)
                }
            };

            let appointment_time = match AppointmentTime::new(hour, minutes) {
                Ok(at) => at,
                Err(error) => {
                    println!("Appointment time invalid. {}", error);
                    process::exit(1)
                }
            };
            add::add_appointment(Appointment::new(description, appointment_time), config)
        }
        Commands::List { current_time, all } => {
            let ref_time = match all {
                true => None,
                _ => Some(current_time),
            };
            list::display_list(ref_time, config)
        }
        Commands::Clear => clear::clear_appointments(config),
    }
}

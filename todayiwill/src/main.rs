use std::process;

use chrono::NaiveDate;
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
    /// Add appointment for today
    Add {
        /// Appointment description
        #[arg(short, long, required_unless_present("stdin"))]
        description: Option<String>,

        /// Appointment time
        #[arg(short, long, required_unless_present("stdin"))]
        time: Option<String>,

        /// Current time, defaults to system time
        #[arg(short, long, value_parser=AppointmentTime::from, default_value_t=AppointmentTime::now())]
        current_time: AppointmentTime,

        /// Parses an appointment as a string ["hh:mm appointment content"]
        #[arg(long, required(false), exclusive(true))]
        stdin: Option<String>,
    },
    /// Clear all the appointments added for today
    Clear,
    /// List the appointments to come for today
    List {
        /// Current time, defaults to system time
        #[arg(short, long, value_parser=AppointmentTime::from, default_value_t=AppointmentTime::now())]
        current_time: AppointmentTime,

        /// Show appointments which will expire in X seconds
        #[arg(short, long, default_value_t=-1)]
        expire_in: i32,

        /// If informed, all appointments are retrieved
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },
    /// List the appointments for other days
    History {
        /// Show appointments which will expire in X seconds
        #[arg(short, long, value_parser=helper::str_dmy_to_naive_date)]
        date: NaiveDate,
    },
}

fn main() {
    let args = Cli::parse();
    let config = Config::standard();

    match args.command {
        Commands::Add {
            description,
            time,
            current_time,
            stdin,
        } => {
            if stdin.is_some() {
                let stdin_value = stdin.unwrap();
                let appointment = Appointment::from(&stdin_value).unwrap();
                println!("{appointment}");
                return;
           }

            let description_value = description.expect("Description should be available here");
            let time_value = time.expect("Time should be available here");

            let result = helper::parse_time(&time_value);
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

            if appointment_time <= current_time {
                println!("Given time already passed.");
                process::exit(1)
            }

            match add::add_appointment(Appointment::new(description_value, appointment_time), config) {
                Ok(..) => (),
                Err(error) => {
                    println!("An error occurred. {}", error);
                    process::exit(1)
                }
            }
        }
        Commands::List {
            current_time,
            expire_in,
            all,
        } => {
            let ref_time = match all {
                true => None,
                _ => Some(current_time),
            };
            let ref_expiration = match expire_in {
                -1 => None,
                other => Some(other),
            };
            list::display_list(ref_time, ref_expiration, config)
        }
        Commands::Clear => clear::clear_appointments(config),
        Commands::History { date } => list::display_all_from(date, config),
    }
}

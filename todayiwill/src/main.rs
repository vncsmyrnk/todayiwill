use std::process;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

extern crate chrono;
extern crate dirs;

mod appointment;

use appointment::{
    add, clear, helper,
    list::{AppointmentList, ListOptions},
    Appointment, AppointmentTime, Config,
};

/// A CLI for remembering what you need to do today
#[derive(Debug, Parser)]
#[command(name = "todayiwill")]
#[command(version, about = "A CLI for remembering what you need to do today", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Current time, defaults to system time
    #[arg(short, long, global=true, default_value_t=AppointmentTime::now())]
    current_time: AppointmentTime,
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
        time: Option<AppointmentTime>,

        /// Parses an appointment as a string ("hh:mm appointment content")
        #[arg(long, required(false), exclusive(true))]
        stdin: Option<Appointment>,
    },
    /// Clear all the appointments added for today
    Clear,
    /// List the appointments to come for today
    List {
        /// Show appointments which will expire in X seconds
        #[arg(short, long)]
        expire_in: Option<i32>,

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
    let parse_result = parse_input();
    match parse_result {
        Ok(..) => (),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}

fn parse_input() -> Result<(), String> {
    let config = Config::standard();
    let args = Cli::parse();

    let current_time = args.current_time;

    match args.command {
        Commands::Add {
            description,
            time,
            stdin,
        } => {
            if stdin.is_some() {
                let appointment = stdin.unwrap();
                return add_appointment(appointment);
            }

            let appointment_description =
                description.expect("Description should be available here");
            let appointment_time = time.expect("Time should be available here");

            if appointment_time <= current_time {
                return Err(String::from("Given time already passed."));
            }

            add_appointment(Appointment::new(appointment_description, appointment_time))?;
        }
        Commands::List { expire_in, all } => {
            let mut list =
                AppointmentList::from_path(current_time, &config.appointment_file_path_current_day);
            if list.no_appointments() {
                println!("There are no appointments added for today.");
                return Ok(());
            }

            if !all {
                match expire_in {
                    None => list.filter(ListOptions::ByReferenceTime),
                    Some(value) => list.filter(ListOptions::ByReferenceAndExpireTime(value)),
                };
            }

            if list.no_appointments() {
                println!("No appointments found.")
            } else {
                println!("{list}")
            }
        }
        Commands::Clear => clear::clear_appointments(config),
        Commands::History { date } => {
            let list = AppointmentList::from_path(
                current_time,
                &(config.appointment_file_path_builder)(date),
            );
            if list.no_appointments() {
                println!("There were no appointments added in this day.")
            } else {
                println!("{list}")
            }
        }
    }

    Ok(())
}

fn add_appointment(appointment: Appointment) -> Result<(), String> {
    match add::add_appointment(appointment, Config::standard()) {
        Ok(..) => Ok(()),
        Err(error) => Err(format!("Error while saving appointment: {}", error)),
    }
}

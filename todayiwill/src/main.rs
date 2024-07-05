use std::{io, process};

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

extern crate chrono;
extern crate dirs;

use colored::Colorize;
use todayiwill::appointment::{
    helper::{self, Config},
    list::{AppointmentList, ListOptions},
    Appointment, AppointmentTime,
};

/// A CLI for remembering what you need to do today
#[derive(Debug, Parser)]
#[command(name = "todayiwill")]
#[command(version, about = format!("A CLI for remembering what you need to do today.\nCheckout the project on {} for submitting requests and rating the app.", "https://github.com/vncsmyrnk/todayiwill".underline()))]
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
        #[arg(long, conflicts_with_all(["description", "time"]))]
        stdin: bool,
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
            let mut list = create_list_for_current_day(&current_time, &config);

            let appointment = match stdin {
                true => read_appointment_from_stdin()?,
                false => Appointment::new(
                    description.expect("Description should be available here"),
                    time.expect("Time should be available here"),
                ),
            };

            if appointment.time <= current_time {
                return Err(String::from("Given time already passed."));
            }

            list.add(appointment, &config.appointment_file_path_current_day)?;
            println!("Appointment added successfully.");
        }
        Commands::List { expire_in, all } => {
            let mut list = create_list_for_current_day(&current_time, &config);

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
                println!("No appointments found.");
            } else {
                println!("{list}");
            }
        }
        Commands::Clear => {
            let mut list = create_list_for_current_day(&current_time, &config);
            list.clear(&config.appointment_file_path_current_day)?;
            println!("Appointments cleared successfully.");
        }
        Commands::History { date } => {
            let list = AppointmentList::from_path(
                &current_time,
                &(config.appointment_file_path_builder)(date),
            );
            if list.no_appointments() {
                println!("There were no appointments added in this day.");
            } else {
                println!("{list}");
            }
        }
    }

    Ok(())
}

fn create_list_for_current_day(current_time: &AppointmentTime, config: &Config) -> AppointmentList {
    AppointmentList::from_path(current_time, &config.appointment_file_path_current_day)
}

fn read_appointment_from_stdin() -> Result<Appointment, String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(..) => (),
        Err(error) => return Err(format!("{error}")),
    };
    Appointment::from(&buffer)
}

use chrono::serde::ts_seconds;
use chrono::{DateTime, DurationRound, TimeDelta, Utc};
use clap::{Parser, Subcommand};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        message: String,
    },
    List {
        #[arg(short, long)]
        short: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub(crate) struct Entry {
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>, // TODO: Decide unixtime or timestamp?
    message: String,
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl Entry {
    fn to_daily(val: &Entry) -> Self {
        Self {
            timestamp: val
                .timestamp
                .duration_trunc(TimeDelta::try_days(1).unwrap())
                .unwrap(),
            message: val.message.to_owned(),
        }
    }
    fn merge(&mut self, entry: &Entry) {
        self.message.push_str(". ");
        self.message
            .push_str(capitalize(entry.message.as_str()).as_str());
        self.message = capitalize(&self.message);
    }
}

fn get_ajour_file(clear: bool) -> File {
    let mut path = config_dir().expect("Unable to find ajour file");
    path.push("ajour");
    path.push("ajour.json");
    let path_str = path.clone();
    let error_message = format!("Unable to open file: {:?}", path_str.as_os_str());
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(clear)
        .open(path)
        .expect(&error_message)
}

fn main() {
    let cli = Cli::parse();

    let mut entries: Vec<Entry>;

    let file = get_ajour_file(false);
    let reader = BufReader::new(file);
    entries = serde_json::from_reader(reader).expect("Unable to parse json");
    match &cli.command {
        Some(Commands::Add { message }) => {
            if !message.is_empty() {
                entries.push(Entry {
                    timestamp: Utc::now(),
                    message: message.to_string(),
                });
                let file = get_ajour_file(true);
                let writer = BufWriter::new(file);
                let res = serde_json::to_writer(writer, &entries);
                if res.is_ok() {
                    println!("Ok")
                } else {
                    println!("Not ok")
                }
            } else {
                todo!();
            }
        }
        Some(Commands::List { short }) => {
            if *short {
                let mut dailies = HashMap::<DateTime<Utc>, Entry>::new();

                entries.iter().map(Entry::to_daily).for_each(|e| {
                    if let Some(daily) = dailies.get_mut(&e.timestamp) {
                        daily.merge(&e);
                    } else {
                        dailies.insert(e.timestamp, e);
                    }
                });

                let mut sorted: Vec<_> = dailies.iter().collect();
                sorted.sort_by_key(|a| a.0);

                for (key, value) in sorted.iter() {
                    println!("{}: {}", key.format("%Y-%m-%d"), value.message);
                }
            } else {
                for entry in &entries {
                    println!("{}: {}", entry.timestamp, entry.message);
                }
            }
        }
        None => {
            todo!();
        }
    }
}

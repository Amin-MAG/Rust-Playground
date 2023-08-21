use std::error::Error;
use std::fs::File;
use std::io::Read;

use clap::{Parser, Subcommand};
use serde::Deserialize;

/// A Simple CLI tool to use with CSV and Clap
#[derive(Parser, Debug)]
#[command(name = "MAG-CLI")]
#[command(version = "1.2.3")]
#[command(about = "This CLI tool will read a CSV files of admitted students and do some calculations", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    // The path to the CSV file
    #[arg(short, long)]
    path: String,

    // The action on the data
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// calculate the average of the admitted students
    Average
}

#[derive(Debug, Deserialize)]
struct AdmissionRawRecord {
    #[serde(rename = "admit")]
    admit: f64,
    #[serde(rename = "gre")]
    gre: f64,
    #[serde(rename = "gpa")]
    gpa: f64,
    #[serde(rename = "rank")]
    rank: f64,
}

#[derive(Debug)]
struct Admission {
    admit: bool,
    _gre: i32,
    gpa: f64,
    _rank: i32,
}

impl From<AdmissionRawRecord> for Admission {
    fn from(record: AdmissionRawRecord) -> Self {
        Admission {
            admit: record.admit == 1.0,
            _gre: record.gre as i32,
            gpa: record.gpa,
            _rank: record.rank as i32,
        }
    }
}

// Calculate the average of GPA of admitted students
fn calculate_average(admissions: &Vec<Admission>) -> Result<f64, String> {
    let (sum, n_of_admitted) = admissions.iter()
        .filter(|admission| admission.admit)
        .fold((0.0, 0), |(sum, count), admission| (sum + admission.gpa.clone(), count + 1));

    if n_of_admitted == 0 {
        return Err(String::from("there is no admitted student to calculate the average"));
    }

    Ok(sum / (n_of_admitted as f64))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Say the greeting
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    // Open the CSV file
    let mut csv_file = File::open(args.path)?;

    // Start reading the content of CSV file
    let mut content = String::new();
    csv_file.read_to_string(&mut content)?;

    // Create a CSV reader
    let mut csv_reader = csv::ReaderBuilder::new().
        has_headers(true).
        from_reader(content.as_bytes());

    // Iterate through the records and create instances of Admission
    let admissions: Vec<Admission> = csv_reader
        .deserialize::<AdmissionRawRecord>()
        .map(|result| result.map(Admission::from))
        .collect::<Result<_, _>>()?;

    // Perform the subcommand
    match args.command {
        Commands::Average => {
            let avg = calculate_average(&admissions)?;
            println!("The average is {}", avg);
        }
    }

    Ok(())
}

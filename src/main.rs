extern crate core;

mod definition;

use crate::definition::definition::Definition;
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use std::error::Error;
use std::fs;
use std::io::Error as IOError;
use std::io::ErrorKind::InvalidInput;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    DataUpdate(DataUpdateInput),
}

#[derive(Args)]
struct DataUpdateInput {
    configuration_file: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::DataUpdate(input) => {
            let configuration_file = input
                .configuration_file
                .as_deref()
                .ok_or_else(|| IOError::new(InvalidInput, "Configuration file not provided"))?;

            let contents = fs::read_to_string(configuration_file)?;

            let config: Definition = serde_yaml::from_str(&contents)?;

            println!("Connector configuration file {:?}", config);

            println!("Connector data update command executed successfully!");

            Ok(())
        }
    }
}

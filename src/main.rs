extern crate core;

mod definition;

use crate::definition::definition::Definition;
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::DataUpdate(input) => {
            let configuration_file = input
                .configuration_file
                .as_deref()
                .ok_or_else(|| IOError::new(InvalidInput, "Configuration file not provided"))?;

            let contents = fs::read_to_string(configuration_file)?;

            let definition: Definition = serde_yaml::from_str(&contents)?;

            let source_connection = PgPoolOptions::new()
                .connect(definition.source.url.as_str())
                .await?;

            let target_connection = PgPoolOptions::new()
                .connect(definition.target.url.as_str())
                .await?;

            println!("Connector configuration file {:?}", definition);

            println!("Connector data update command executed successfully!");

            Ok(())
        }
    }
}

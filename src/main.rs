use clap::{Args, Parser, Subcommand};

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
struct DataUpdateInput {}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::DataUpdate(_input) => {
            println!("Connector data update command executed successfully!");
        }
    }
}

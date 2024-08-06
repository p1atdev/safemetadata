mod fetch;
mod metadata;
mod parser;
mod utils;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use parser::LocalParser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show the information of the file
    Show {
        /// The path of the safetensors file
        file_path: String,
        // /// Repository id on HuggingFace hub
        // #[clap(long)]
        // repo_id: Option<String>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{:?}", args);

    match args.command {
        Some(Commands::Show { file_path }) => {
            // let header = parse_header(&file_path)?;

            // println!("{:?}", header);
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}

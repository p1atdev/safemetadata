mod fetch;
mod metadata;
mod parser;
mod utils;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use hf_hub::RepoType;
use parser::{LocalParser, MetadataParser, RemoteParser};

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

        /// Repository id on HuggingFace hub
        #[clap(long)]
        repo_id: Option<String>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{:?}", args);

    match args.command {
        Some(Commands::Show { file_path, repo_id }) => {
            if repo_id.is_some() {
                let parser =
                    RemoteParser::from_hub(&repo_id.unwrap(), RepoType::Model, &file_path, &None);

                let header = parser.parse_header()?;
                println!("{:?}", header);
            } else {
                let parser = LocalParser::new(&file_path);

                let header = parser.parse_header()?;
                println!("{:?}", header);
            }
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}

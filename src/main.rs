mod fetch;
mod metadata;
mod parser;
mod table;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use hf_hub::RepoType;
use parser::{LocalParser, MetadataParser, RemoteParser};
use table::InfoTable;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show the Stability AI Model Specification of the file
    #[clap(name = "modelspec")]
    ModelSpec {
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
        Some(Commands::ModelSpec { file_path, repo_id }) => {
            let header = match repo_id {
                Some(repo_id) => {
                    let parser =
                        RemoteParser::from_hub(&repo_id, RepoType::Model, &file_path, &None);
                    parser.parse_header()?
                }
                None => {
                    let parser = LocalParser::new(&file_path);
                    parser.parse_header()?
                }
            };

            if let Some(modelspec) = header.metadata.model_spec {
                println!("Stability AI Model Metadata Standard Specification");
                println!("{}", modelspec.format_table());
            } else {
                println!("No modelspec found in the file.");
            }
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}

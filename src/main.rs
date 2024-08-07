mod fetch;
mod metadata;
mod parser;
mod table;
mod utils;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use hf_hub::RepoType;
use metadata::{Header, Weight};
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
    /// Show the parameter sizes of the model
    Params {
        /// The path of the safetensors file
        file_path: String,

        /// Repository id on HuggingFace hub
        #[clap(long)]
        repo_id: Option<String>,
    },

    /// Show the layers of the model
    Layers {
        /// The path of the safetensors file
        file_path: String,

        /// Repository id on HuggingFace hub
        #[clap(long)]
        repo_id: Option<String>,
    },

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

fn parse_header(repo_id: Option<String>, file_path: String) -> Result<Header> {
    match repo_id {
        Some(repo_id) => {
            let parser = RemoteParser::from_hub(&repo_id, RepoType::Model, &file_path, &None);
            parser.parse_header()
        }
        None => {
            let parser = LocalParser::new(&file_path);
            parser.parse_header()
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // println!("{:?}", args);

    match args.command {
        Some(Commands::Params { file_path, repo_id }) => {
            let header = parse_header(repo_id, file_path)?;

            let params = header.weights.values().fold(0, |sum: i64, weight| {
                sum + weight.shape.iter().product::<i64>()
            });
            let (params, unit) = utils::pretty_param_size(params);

            println!("Total parameters: {}{} params", params, unit);
        }
        Some(Commands::Layers { file_path, repo_id }) => {
            let header = parse_header(repo_id, file_path)?;

            let format = match header.metadata.format {
                Some(format) => format.to_string(),
                None => "Unknown format".to_string(),
            };

            println!("Tensor format: {}", format);
            println!("{}", header.weights.format_table());
        }
        Some(Commands::ModelSpec { file_path, repo_id }) => {
            let header = parse_header(repo_id, file_path)?;

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

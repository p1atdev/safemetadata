mod fetch;
mod metadata;
mod parser;
mod table;
mod utils;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use hf_hub::{Cache, RepoType};
use metadata::Header;
use parser::{LocalParser, MetadataParser, RemoteParser};
use table::InfoTable;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
struct FileArgs {
    /// The path of the safetensors file
    file_path: String,

    /// Repository id on HuggingFace hub
    #[clap(long, short)]
    repo_id: Option<String>,

    /// HuggingFace API token
    #[clap(long, short)]
    token: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show the parameter sizes of the model
    Params(FileArgs),

    /// Show the layers of the model
    Layers(FileArgs),

    /// Show the Stability AI Model Specification of the file
    #[clap(name = "modelspec")]
    ModelSpec(FileArgs),
}

fn parse_header(args: FileArgs) -> Result<Header> {
    let FileArgs {
        file_path,
        repo_id,
        token,
    } = args;

    match repo_id {
        Some(repo_id) => {
            let token = match token {
                Some(token) => Some(token),       // do nothing
                None => Cache::default().token(), // load token from cache
            };
            let parser = RemoteParser::from_hub(&repo_id, RepoType::Model, &file_path, &token);
            parser.parse_header()
        }
        None => {
            let parser = LocalParser::new(&file_path);
            parser.parse_header()
        }
    }
}

fn main() -> Result<()> {
    let args = CLI::parse();

    match args.command {
        Commands::Params(file_args) => {
            let header = parse_header(file_args)?;

            let params = header.weights.values().fold(0, |sum: i64, weight| {
                sum + weight.shape.iter().product::<i64>()
            });
            let (params, unit) = utils::pretty_param_size(params);

            println!("Total parameters: {}{} params", params, unit);
        }
        Commands::Layers(file_args) => {
            let header = parse_header(file_args)?;

            let format = match header.metadata.format {
                Some(format) => format.to_string(),
                None => "Unknown format".to_string(),
            };

            println!("Tensor format: {}", format);
            println!("{}", header.weights.format_table());
        }
        Commands::ModelSpec(file_args) => {
            let header = parse_header(file_args)?;

            if let Some(modelspec) = header.metadata.model_spec {
                println!("Stability AI Model Metadata Standard Specification");
                println!("{}", modelspec.format_table());
            } else {
                println!("No modelspec found in the file.");
            }
        }
    }

    Ok(())
}

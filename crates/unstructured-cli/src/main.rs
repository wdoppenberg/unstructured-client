mod args;

use clap::{Parser};
use reqwest::Url;
use std::path::PathBuf;
use std::process::ExitCode;
use anyhow::Result;

use unstructured_client::{PartitionParameters, UnstructuredClient};

use crate::args::CliPartitionParameters;

#[derive(Debug, Parser)]
pub struct AppArgs {
    /// Path to the file to be parsed
    #[clap(long)]
    pub file_path: PathBuf,
    /// The base URL for the Unstructured API
    #[clap(long, default_value = "http://localhost:8000")]
    pub base_url: Url,
    #[clap(flatten)]
    partition_parameters: CliPartitionParameters
}

#[tokio::main]
async fn main() -> Result<ExitCode> {
    // Define CLI arguments using clap
    let app_args = AppArgs::parse();

    // Create an instance of UnstructuredClient
    let client = UnstructuredClient::new(app_args.base_url);

    // Define partition parameters
    let params = PartitionParameters::from(app_args.partition_parameters);

    // Make the API request
    match client.partition_file(&app_args.file_path, params).await {
        Ok(response) => println!("{:?}", response),
        Err(error) => eprintln!("Error: {:?}", error),
    }

    Ok(ExitCode::SUCCESS)
}
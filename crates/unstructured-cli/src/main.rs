mod args;
mod error;

use clap::Parser;
use reqwest::Url;
use serde_json::to_string;
use std::path::PathBuf;

use crate::args::CliPartitionParameters;
use crate::error::CliError;
use unstructured_client::partition::PartitionResponse;
use unstructured_client::{PartitionParameters, UnstructuredClient};

#[derive(Debug, Parser)]
pub struct AppArgs {
    /// Path to the file to be parsed
    #[clap(long)]
    pub file_path: PathBuf,
    /// The base URL for the Unstructured API
    #[clap(long, default_value = "http://localhost:8000")]
    pub base_url: Url,
    #[clap(flatten)]
    partition_parameters: CliPartitionParameters,
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    // Parse CLI Arguments
    let app_args = AppArgs::parse();

    // Create an instance of UnstructuredClient
    let client = UnstructuredClient::new(app_args.base_url.as_ref())?;

    // Define partition parameters
    let params = PartitionParameters::from(app_args.partition_parameters);

    // Make the API request
    let partition_response = client.partition_file(&app_args.file_path, params).await?;

    // Print the output
    match partition_response {
        PartitionResponse::Success(element_list) => {
            println!("{}", to_string(&element_list)?);
        }
        value => {
            eprintln!("{}", to_string(&value)?);
        }
    }

    Ok(())
}

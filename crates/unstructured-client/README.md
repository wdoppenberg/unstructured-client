# Unofficial Unstructured Rust client library

Use [Unstructured](https://docs.unstructured.io/welcome)'s API service with this light client library for Rust. 


## Usage example

Either use their platform offering, or spin up an Unstructured API service locally:

```bash
docker run -p 8000:8000 -it downloads.unstructured.io/unstructured-io/unstructured-api:latest
```

```rust
use unstructured_client::{error::Result, PartitionParameters, UnstructuredClient};

#[tokio::main]
async fn main() -> Result<()> {
    // Specify file path
    let file_path =
        std::path::PathBuf::from("crates/unstructured-cli/tests/fixtures/sample-pdf.pdf");

    // Create an instance of UnstructuredClient
    let client = UnstructuredClient::new("http://localhost:8765")?;

    // Define partition parameters
    let params = PartitionParameters::default();

    // Make the API request
    match client.partition_file(&file_path, params).await {
        Ok(element_list) => println!("{:#?}", element_list),
        Err(error) => eprintln!("Error: {:#?}", error),
    }

    Ok(())
}
```

Check out [`partition.rs`](src/partition.rs) for the partition arguments.



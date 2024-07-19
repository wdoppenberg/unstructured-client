# Unofficial Unstructured Rust library and CLI

Use [Unstructured](https://docs.unstructured.io/welcome)'s API service with this light client library for Rust. 


## Library Usage example

Either use their platform offering, or spin up an Unstructured API service locally:

```bash
docker run -p 8000:8000 -it downloads.unstructured.io/unstructured-io/unstructured-api:latest
```

```rust
use std::path::PathBuf;
use unstructured_client::{PartitionParameters, UnstructuredClient};

#[tokio::main]
async fn main() -> Result<()> {
	let file_path = PathBuf::from("/path/to/file.pdf");
	// Create an instance of UnstructuredClient
	let client = UnstructuredClient::new("http://localhost:8000");

	// Define partition parameters
	let params = PartitionParameters::default();

	// Make the API request
	match client.partition_file(&file_path, params).await {
		Ok(response) => println!("{:?}", response),
		Err(error) => eprintln!("Error: {:?}", error),
	}

	Ok(())
}
```

Check out [`partition.rs`](crates/unstructured-client/src/partition.rs) for the partition arguments
use reqwest::multipart::Form;
use reqwest::{multipart, Url};
use std::fs;
use std::path::Path;

use crate::error::{ClientError, Result};
use crate::partition::{PartitionParameters, PartitionResponse};

/// Current crate version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The sub-route for partitioning
const API_ROUTE: &str = "/general/v0/general";

#[derive(Debug, Clone)]
pub struct UnstructuredClient {
    client: reqwest::Client,
    base_url: Url,
    api_key: Option<String>,
}

impl UnstructuredClient {
    /// Creates a new `UnstructuredClient` with a specified base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url`: A string slice that holds the base URL for the client.
    ///
    /// returns: `Result<UnstructuredClient, ClientError>` - On success, returns an instance of `UnstructuredClient`.
    /// On failure, returns a `ClientError` explaining what went wrong.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = UnstructuredClient::new("https://example.com");
    /// match client {
    ///     Ok(client) => println!("Client created successfully."),
    ///     Err(e) => println!("Failed to create client: {:?}", e),
    /// }
    /// ```
    pub fn new(base_url: &str) -> Result<Self> {
        let url = Url::parse(base_url).map_err(|e| ClientError::URLParseFailed(e.to_string()))?;
        Ok(UnstructuredClient {
            client: reqwest::Client::new(),
            base_url: url,
            api_key: None,
        })
    }

    /// Sets the API key for the `UnstructuredClient`.
    ///
    /// This method allows you to provide an API key that will be included in the
    /// headers of requests made by the client.
    ///
    /// # Arguments
    ///
    /// * `api_key`: A string slice that holds the API key.
    ///
    /// # Returns
    ///
    /// `Self` with the API key set.
    pub fn with_api_key(self, api_key: &str) -> Self {
        Self {
            api_key: Some(api_key.to_string()),
            ..self
        }
    }

    /// Partitions the content of a given file using Unstructured's API.
    ///
    /// This asynchronous function reads the content of a specified file, creates a multipart
    /// form along with given parameters, and sends a POST request to the Unstructured API route.
    /// The result is a text representation of the file's content, partitioned by the type of the
    /// text element.
    ///
    /// # Arguments
    ///
    /// * `file_path`: The path to the file that needs to be partitioned.
    /// * `params`: Parameters for partitioning which are defined by the `PartitionParameters` type.
    ///
    /// Returns: `Result<ElementList, ClientError>` - On success, returns a [ElementList];
    /// otherwise returns a `ClientError`.
    #[tracing::instrument]
    pub async fn partition_file(
        &self,
        file_path: &Path,
        params: PartitionParameters,
    ) -> Result<PartitionResponse> {
        let url = self
            .base_url
            .join(API_ROUTE)
            .map_err(|e| ClientError::URLParseFailed(e.to_string()))?;

        tracing::trace!("Building partition request for {file_path:?} to {url}.");

        let file = fs::read(file_path)?;

        let file_name = file_path
            .file_name()
            .ok_or(ClientError::FileIOError("No filename found.".into()))?
            .to_str()
            .ok_or(ClientError::FileIOError("File name not valid UTF-8".into()))?
            .to_string();

        tracing::debug!("Reading file into memory");
        let file_part = multipart::Part::bytes(file).file_name(file_name);

        // Create reqwest multipart Form using the implementation for Into<Form>
        let form: Form = params.into();

        // Add file part
        let form = form.part("files", file_part);

        // Post request and await response
        tracing::debug!("Performing request");
        let request = self
            .client
            .post(url)
            .multipart(form)
            .header("Content-Type", "multipart/form-data")
            .header("User-Agent", format!("Unstructured-Rust-Client/{VERSION}"));

        // Add api key
        let request = {
            match &self.api_key {
                None => request,
                Some(api_key) => request.header("unstructured-api-key", api_key),
            }
        };

        // Process response
        let response = request.send().await?;
        let element_list = response.json().await?;

        Ok(element_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::partition::PartitionResponse::{Failure, Success};
    use mockito::Matcher;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_partition_file() -> Result<()> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let url = server.url();

        // Mock server setup
        let mock = server
            .mock("POST", "/general/v0/general")
            .match_header(
                "content-type",
                Matcher::Regex("multipart/form-data.*".to_string()),
            )
            .with_status(200)
            .with_body(
                r#"
		        [
		            {
		                "type": "NarrativeText",
		                "element_id": "1",
		                "text": "This is a test paragraph.",
		                "metadata": null
		            },
		            {
		                "type": "NarrativeText",
		                "element_id": "1",
		                "text": "This is a test paragraph."
		            },
		            {
		                "type": "Image",
		                "element_id": "2",
		                "text": "base64encodedstring",
		                "metadata": {
		                    "filename": "image.jpg"
		                }
		            }
		        ]
            "#,
            )
            .create();

        // Create a temporary file using tempfile
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "This is a test file.").unwrap();

        // Create the client and parameters
        let client = UnstructuredClient::new(&url).unwrap();
        let params = PartitionParameters::default(); // Adjust with actual defaults

        // Call the function
        let result = client.partition_file(temp_file.path(), params).await?;

        // Ensure the result is OK and matches expected structure
        match result {
            Success(element_list) => {
                assert_eq!(element_list.len(), 3);
            }
            Failure(e) => {
                panic!("Test failed with error: {:?}", e);
            }
        }

        // Verify that the mock was called
        mock.assert();

        Ok(())
    }
}

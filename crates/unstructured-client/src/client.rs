use anyhow::Result;
use reqwest::{multipart, Url};
use std::fs;
use std::path::Path;

use crate::partition::{ElementList, PartitionParameters};

#[derive(Debug, Clone)]
struct RetryConfig {}

#[derive(Debug, Clone)]
pub struct UnstructuredClient {
    client: reqwest::Client,
    base_url: Url,
    // user_agent: String,
    // retry_config: Option<RetryConfig>,
}

impl UnstructuredClient {
    pub fn new(base_url: impl Into<Url>) -> Self {
        UnstructuredClient {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            // user_agent: format!("Unstructured-Rust-Client/{VERSION}").to_string(),
            // retry_config: None,
        }
    }

    pub async fn partition_file(
        &self,
        file_path: &Path,
        params: PartitionParameters,
    ) -> Result<ElementList> {
        // let url = format!("http://{}/general/v0/general", self.base_url);
        let url = self.base_url.join("/general/v0/general")?;

        let file = fs::read(file_path)?;
        // TODO: Get rid of unwraps
        let file_name = String::from(file_path.file_name().unwrap().to_str().unwrap());
        let file_part = multipart::Part::bytes(file).file_name(file_name);

        let form = multipart::Form::new()
            .part("files", file_part)
            .text("coordinates", params.coordinates.to_string())
            .text(
                "encoding",
                params
                    .encoding
                    .clone()
                    .unwrap_or_else(|| "utf-8".to_string()),
            )
            .text(
                "extract_image_block_types",
                serde_json::to_string(&params.extract_image_block_types).unwrap(),
            )
            .text(
                "gz_uncompressed_content_type",
                params
                    .gz_uncompressed_content_type
                    .clone()
                    .unwrap_or_default(),
            )
            .text(
                "hi_res_model_name",
                params.hi_res_model_name.clone().unwrap_or_default(),
            )
            .text(
                "include_page_breaks",
                params.include_page_breaks.to_string(),
            )
            .text(
                "languages",
                serde_json::to_string(&params.languages).unwrap(),
            )
            .text("output_format", params.output_format.clone())
            .text(
                "skip_infer_table_types",
                serde_json::to_string(&params.skip_infer_table_types).unwrap(),
            )
            .text(
                "starting_page_number",
                params.starting_page_number.unwrap_or_default().to_string(),
            )
            .text("strategy", params.strategy.clone())
            .text("unique_element_ids", params.unique_element_ids.to_string())
            .text("xml_keep_tags", params.xml_keep_tags.to_string())
            .text(
                "chunking_strategy",
                params.chunking_strategy.clone().unwrap_or_default(),
            )
            .text(
                "combine_under_n_chars",
                params.combine_under_n_chars.unwrap_or_default().to_string(),
            )
            .text(
                "include_orig_elements",
                params.include_orig_elements.to_string(),
            )
            .text(
                "max_characters",
                params.max_characters.unwrap_or_default().to_string(),
            )
            .text("multipage_sections", params.multipage_sections.to_string())
            .text(
                "new_after_n_chars",
                params.new_after_n_chars.unwrap_or_default().to_string(),
            )
            .text("overlap", params.overlap.to_string())
            .text("overlap_all", params.overlap_all.to_string());

        let response = self
            .client
            .post(url)
            .multipart(form)
            .header("Content-Type", "multipart/form-data")
            // TODO: Add API key
            // .header("unstructured-api-key", api_key)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Request error: {:?}", e);
                e
            })?;

        if response.status().is_success() {
            let partition_response = response.json::<ElementList>().await.map_err(|e| {
                eprintln!("Response parsing error: {:?}", e);
                e
            })?;
            Ok(partition_response)
        } else {
            let error_text = response.text().await.map_err(|e| {
                eprintln!("Error text retrieval error: {:?}", e);
                e
            })?;
            anyhow::bail!("Request didn't succeed: {}", error_text);
        }
    }
}

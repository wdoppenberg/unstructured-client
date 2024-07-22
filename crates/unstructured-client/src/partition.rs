use crate::ElementList;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

/// This chunks the returned elements after partitioning.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChunkingStrategy {
    Basic,
    ByPage,
    BySimilarity,
    ByTitle,
}

/// The strategy to use for partitioning PDF/image.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Strategy {
    Fast,
    HiRes,
    Auto,
    OcrOnly,
}

/// The format of the response.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum OutputFormat {
    #[serde(rename = "application/json")]
    ApplicationJson,

    #[serde(rename = "text/csv")]
    TextCsv,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionParameters {
    /// If `True`, return coordinates for each element extracted via OCR. Default: `False`.
    pub coordinates: bool,

    /// The encoding method used to decode the text input. Default: utf-8
    pub encoding: Option<String>,

    /// The types of elements to extract, for use in extracting image blocks as base64 encoded data stored in metadata fields. Default: [].
    pub extract_image_block_types: Vec<String>,

    /// If file is gzipped, use this content type after unzipping.
    pub gz_uncompressed_content_type: Option<String>,

    /// The name of the inference model used when strategy is hi_res
    pub hi_res_model_name: Option<String>,

    /// If true, the output will include page breaks if the filetype supports it. Default: false
    pub include_page_breaks: bool,

    /// The languages present in the document, for use in partitioning and/or OCR. See the Tesseract documentation for a full list of languages. Default: [].
    pub languages: Option<Vec<String>>,

    /// The format of the response. Supported formats are application/json and text/csv. Default: application/json.
    pub output_format: String,

    /// The document types that you want to skip table extraction with. Default: [].
    pub skip_infer_table_types: Vec<String>,

    /// When PDF is split into pages before sending it into the API, providing this information will allow the page number to be assigned correctly. Introduced in 1.0.27.
    pub starting_page_number: Option<i32>,

    /// The strategy to use for partitioning PDF/image. Options are fast, hi_res, auto. Default: auto
    pub strategy: Strategy,

    /// When `True`, assign UUIDs to element IDs, which guarantees their uniqueness (useful when using them as primary keys in database). Otherwise a SHA-256 of element text is used. Default: `False`
    pub unique_element_ids: bool,

    /// If `True`, will retain the XML tags in the output. Otherwise it will simply extract the text from within the tags. Only applies to XML documents. Default: false
    pub xml_keep_tags: bool,

    /// Use one of the supported strategies to chunk the returned elements after partitioning. When 'chunking_strategy' is not specified, no chunking is performed and any other chunking parameters provided are ignored. Supported strategies: 'basic', 'by_page', 'by_similarity', or 'by_title'
    pub chunking_strategy: Option<ChunkingStrategy>,

    /// If chunking strategy is set, combine elements until a section reaches a length of n chars. Default: 500
    pub combine_under_n_chars: Option<i32>,

    /// When a chunking strategy is specified, each returned chunk will include the elements consolidated to form that chunk as `.metadata.orig_elements`. Default: true.
    pub include_orig_elements: bool,

    /// If chunking strategy is set, cut off new sections after reaching a length of n chars (hard max). Default: 500
    pub max_characters: Option<i32>,

    /// If chunking strategy is set, determines if sections can span multiple sections. Default: true
    pub multipage_sections: bool,

    /// If chunking strategy is set, cut off new sections after reaching a length of n chars (soft max). Default: 1500
    pub new_after_n_chars: Option<i32>,

    /// Specifies the length of a string ('tail') to be drawn from each chunk and prefixed to the next chunk as a context-preserving mechanism. By default, this only applies to split-chunks where an oversized element is divided into multiple chunks by text-splitting. Default 0.
    pub overlap: i32,

    /// When `True`, apply overlap between 'normal' chunks formed from whole elements and not subject to text-splitting. Use this with caution as it entails a certain level of 'pollution' of otherwise clean semantic chunk boundaries. Default false.
    pub overlap_all: bool,

    /// A value between 0.0 and 1.0 describing the minimum similarity two elements must have to be included in the same chunk. Note that similar elements may be separated to meet chunk-size criteria; this value can only guarantees that two elements with similarity below the threshold will appear in separate chunks.
    pub similarity_threshold: Option<f64>,
}

impl Default for PartitionParameters {
    fn default() -> Self {
        PartitionParameters {
            coordinates: false,
            encoding: Some("utf-8".to_string()),
            extract_image_block_types: vec![],
            gz_uncompressed_content_type: None,
            hi_res_model_name: None,
            include_page_breaks: false,
            languages: None,
            output_format: "application/json".to_string(),
            skip_infer_table_types: vec![],
            starting_page_number: None,
            strategy: Strategy::Auto,
            unique_element_ids: false,
            xml_keep_tags: false,
            chunking_strategy: None,
            combine_under_n_chars: None,
            include_orig_elements: true,
            max_characters: None,
            multipage_sections: true,
            new_after_n_chars: None,
            overlap: 0,
            overlap_all: false,
            similarity_threshold: None,
        }
    }
}

impl From<PartitionParameters> for Form {
    fn from(value: PartitionParameters) -> Self {
        let mut form = Form::new();
        form = form.text("coordinates", value.coordinates.to_string());
        if let Some(encoding) = value.encoding.clone() {
            form = form.text("encoding", encoding);
        }
        form = form.text(
            "extract_image_block_types",
            serde_json::to_string(&value.extract_image_block_types).unwrap(),
        );
        if let Some(gz_uncompressed_content_type) = value.gz_uncompressed_content_type.clone() {
            form = form.text("gz_uncompressed_content_type", gz_uncompressed_content_type);
        }
        if let Some(hi_res_model_name) = value.hi_res_model_name.clone() {
            form = form.text("hi_res_model_name", hi_res_model_name);
        }
        form = form.text("include_page_breaks", value.include_page_breaks.to_string());
        if let Some(languages) = value.languages.clone() {
            form = form.text("languages", serde_json::to_string(&languages).unwrap());
        }
        form = form.text("output_format", value.output_format.clone());
        form = form.text(
            "skip_infer_table_types",
            serde_json::to_string(&value.skip_infer_table_types).unwrap(),
        );
        if let Some(starting_page_number) = value.starting_page_number {
            form = form.text("starting_page_number", starting_page_number.to_string());
        }
        form = form.text("strategy", {
            let s = String::from(
                serde_json::to_string(&value.strategy)
                    .expect("Could not convert Strategy enum to string.")
                    .trim_matches('"'),
            );
            s
        });
        form = form.text("unique_element_ids", value.unique_element_ids.to_string());
        form = form.text("xml_keep_tags", value.xml_keep_tags.to_string());
        if let Some(chunking_strategy) = value
            .chunking_strategy
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .expect("Could not convert Chunking Strategy enum to string.")
        {
            form = form.text(
                "chunking_strategy",
                chunking_strategy.trim_matches('"').to_string(),
            );
        }
        if let Some(combine_under_n_chars) = value.combine_under_n_chars {
            form = form.text("combine_under_n_chars", combine_under_n_chars.to_string());
        }
        form = form.text(
            "include_orig_elements",
            value.include_orig_elements.to_string(),
        );
        if let Some(max_characters) = value.max_characters {
            form = form.text("max_characters", max_characters.to_string());
        }
        form = form.text("multipage_sections", value.multipage_sections.to_string());
        if let Some(new_after_n_chars) = value.new_after_n_chars {
            form = form.text("new_after_n_chars", new_after_n_chars.to_string());
        }
        form = form.text("overlap", value.overlap.to_string());
        form = form.text("overlap_all", value.overlap_all.to_string());
        form
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PartitionResponse {
    /// Successful response; returns a list of elements.
    Success(ElementList),

    /// Failed request; returns JSON with error message.
    Failure(serde_json::Value),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_partition_params() {
        let params = PartitionParameters::default();
        println!("{:?}", params)
    }

    #[test]
    fn test_deserialize_chunking_strategy() {
        let json = r#""basic""#;
        let strategy: ChunkingStrategy = serde_json::from_str(json).unwrap();
        assert_eq!(strategy, ChunkingStrategy::Basic);
    }

    #[test]
    fn test_deserialize_strategy() {
        let json = r#""auto""#;
        let strategy: Strategy = serde_json::from_str(json).unwrap();
        assert_eq!(strategy, Strategy::Auto);
    }

    #[test]
    fn test_deserialize_output_format() {
        let json = r#""application/json""#;
        let format: OutputFormat = serde_json::from_str(json).unwrap();
        assert_eq!(format, OutputFormat::ApplicationJson);
    }

    #[test]
    fn test_deserialize_partition_parameters() {
        let json = r#"{
            "coordinates": true,
            "encoding": "utf-8",
            "extract_image_block_types": [],
            "gz_uncompressed_content_type": null,
            "hi_res_model_name": null,
            "include_page_breaks": true,
            "languages": null,
            "output_format": "application/json",
            "skip_infer_table_types": [],
            "starting_page_number": null,
            "strategy": "auto",
            "unique_element_ids": false,
            "xml_keep_tags": false,
            "chunking_strategy": null,
            "combine_under_n_chars": null,
            "include_orig_elements": true,
            "max_characters": null,
            "multipage_sections": true,
            "new_after_n_chars": null,
            "overlap": 0,
            "overlap_all": false,
            "similarity_threshold": null
        }"#;
        let params: PartitionParameters = serde_json::from_str(json).unwrap();
        assert_eq!(params.coordinates, true);
        assert_eq!(params.encoding.unwrap(), "utf-8");
        assert_eq!(params.include_page_breaks, true);
        assert_eq!(params.output_format, "application/json".to_string());
        assert_eq!(params.include_orig_elements, true);
        assert_eq!(params.multipage_sections, true);
        assert_eq!(params.overlap, 0);
        assert_eq!(params.overlap_all, false);
    }
}

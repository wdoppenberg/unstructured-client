use serde::{Deserialize, Serialize};

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
    pub languages: Vec<String>,
    /// The format of the response. Supported formats are application/json and text/csv. Default: application/json.
    pub output_format: String,
    /// The document types that you want to skip table extraction with. Default: [].
    pub skip_infer_table_types: Vec<String>,
    /// When PDF is split into pages before sending it into the API, providing this information will allow the page number to be assigned correctly. Introduced in 1.0.27.
    pub starting_page_number: Option<i32>,
    /// The strategy to use for partitioning PDF/image. Options are fast, hi_res, auto. Default: auto
    pub strategy: String,
    /// When `True`, assign UUIDs to element IDs, which guarantees their uniqueness (useful when using them as primary keys in database). Otherwise a SHA-256 of element text is used. Default: `False`
    pub unique_element_ids: bool,
    /// If `True`, will retain the XML tags in the output. Otherwise it will simply extract the text from within the tags. Only applies to XML documents. Default: false
    pub xml_keep_tags: bool,
    /// Use one of the supported strategies to chunk the returned elements after partitioning. When 'chunking_strategy' is not specified, no chunking is performed and any other chunking parameters provided are ignored. Supported strategies: 'basic', 'by_page', 'by_similarity', or 'by_title'
    pub chunking_strategy: Option<String>,
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
            languages: vec![],
            output_format: "application/json".to_string(),
            skip_infer_table_types: vec![],
            starting_page_number: None,
            strategy: "auto".to_string(),
            unique_element_ids: false,
            xml_keep_tags: false,
            chunking_strategy: None,
            combine_under_n_chars: Some(500),
            include_orig_elements: true,
            max_characters: Some(500),
            multipage_sections: true,
            new_after_n_chars: Some(1500),
            overlap: 0,
            overlap_all: false,
            similarity_threshold: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::partition::PartitionParameters;

    #[test]
    fn test_default_partition_params() {
        let params = PartitionParameters::default();
        println!("{:?}", params)
    }
}

#[derive(Debug, Deserialize)]
pub struct Element {
    pub r#type: String,
    pub element_id: String,
    pub text: String,
    pub metadata: Option<serde_json::Value>,
}

pub type ElementList = Vec<Element>;

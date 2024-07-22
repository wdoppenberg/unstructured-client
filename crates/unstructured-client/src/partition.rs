use reqwest::multipart::Form;
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

impl From<PartitionParameters> for Form {
    fn from(value: PartitionParameters) -> Self {
        Form::new()
            .text("coordinates", value.coordinates.to_string())
            .text(
                "encoding",
                value
                    .encoding
                    .clone()
                    .unwrap_or_else(|| "utf-8".to_string()),
            )
            .text(
                "extract_image_block_types",
                serde_json::to_string(&value.extract_image_block_types).unwrap(),
            )
            .text(
                "gz_uncompressed_content_type",
                value
                    .gz_uncompressed_content_type
                    .clone()
                    .unwrap_or_default(),
            )
            .text(
                "hi_res_model_name",
                value.hi_res_model_name.clone().unwrap_or_default(),
            )
            .text("include_page_breaks", value.include_page_breaks.to_string())
            .text(
                "languages",
                serde_json::to_string(&value.languages).unwrap(),
            )
            .text("output_format", value.output_format.clone())
            .text(
                "skip_infer_table_types",
                serde_json::to_string(&value.skip_infer_table_types).unwrap(),
            )
            .text(
                "starting_page_number",
                value.starting_page_number.unwrap_or_default().to_string(),
            )
            .text("strategy", value.strategy.clone())
            .text("unique_element_ids", value.unique_element_ids.to_string())
            .text("xml_keep_tags", value.xml_keep_tags.to_string())
            .text(
                "chunking_strategy",
                value.chunking_strategy.clone().unwrap_or_default(),
            )
            .text(
                "combine_under_n_chars",
                value.combine_under_n_chars.unwrap_or_default().to_string(),
            )
            .text(
                "include_orig_elements",
                value.include_orig_elements.to_string(),
            )
            .text(
                "max_characters",
                value.max_characters.unwrap_or_default().to_string(),
            )
            .text("multipage_sections", value.multipage_sections.to_string())
            .text(
                "new_after_n_chars",
                value.new_after_n_chars.unwrap_or_default().to_string(),
            )
            .text("overlap", value.overlap.to_string())
            .text("overlap_all", value.overlap_all.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Element {
    pub r#type: String,
    pub element_id: String,
    pub text: String,
    pub metadata: Option<serde_json::Value>,
}

pub type ElementList = Vec<Element>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default_partition_params() {
        let params = PartitionParameters::default();
        println!("{:?}", params)
    }

    #[test]
    fn test_deserialize_simple() {
        let json_str = r#"
        {
          "type": "text",
          "element_id": "1",
          "text": "Hello, world!",
          "metadata": null
        }
        "#;

        let expected = Element {
            r#type: "text".to_string(),
            element_id: "1".to_string(),
            text: "Hello, world!".to_string(),
            metadata: None,
        };

        let element: Element = serde_json::from_str(json_str).unwrap();
        assert_eq!(element, expected);
    }

    #[test]
    fn test_deserialize_with_metadata() {
        let json_str = r#"
        {
          "type": "image",
          "element_id": "2",
          "text": "An image element",
          "metadata": {
            "width": 1024,
            "height": 768,
            "format": "png"
          }
        }
        "#;

        let expected = Element {
            r#type: "image".to_string(),
            element_id: "2".to_string(),
            text: "An image element".to_string(),
            metadata: Some(json!({
                "width": 1024,
                "height": 768,
                "format": "png"
            })),
        };

        let element: Element = serde_json::from_str(json_str).unwrap();
        assert_eq!(element, expected);
    }

    #[test]
    fn test_deserialize_without_metadata() {
        let json_str = r#"
        {
          "type": "video",
          "element_id": "3",
          "text": "A video element"
        }
        "#;

        let expected = Element {
            r#type: "video".to_string(),
            element_id: "3".to_string(),
            text: "A video element".to_string(),
            metadata: None,
        };

        let element: Element = serde_json::from_str(json_str).unwrap();
        assert_eq!(element, expected);
    }

    #[test]
    fn test_deserialize_complex_metadata() {
        let json_str = r#"
        {
          "type": "text",
          "element_id": "4",
          "text": "A complex text element",
          "metadata": {
            "attributes": {
              "bold": true,
              "italic": false
            },
            "styles": [
              "font-size: 14px",
              "color: #333333"
            ]
          }
        }
        "#;

        let expected = Element {
            r#type: "text".to_string(),
            element_id: "4".to_string(),
            text: "A complex text element".to_string(),
            metadata: Some(json!({
                "attributes": {
                    "bold": true,
                    "italic": false
                },
                "styles": [
                    "font-size: 14px",
                    "color: #333333"
                ]
            })),
        };

        let element: Element = serde_json::from_str(json_str).unwrap();
        assert_eq!(element, expected);
    }

    #[test]
    fn test_deserialize_nested_metadata() {
        let json_str = r#"
        {
          "type": "container",
          "element_id": "5",
          "text": "Container element",
          "metadata": {
            "items": [
              {
                "type": "text",
                "text": "Nested text element"
              },
              {
                "type": "image",
                "src": "example.png"
              }
            ]
          }
        }
        "#;

        let expected = Element {
            r#type: "container".to_string(),
            element_id: "5".to_string(),
            text: "Container element".to_string(),
            metadata: Some(json!({
                "items": [
                    {
                        "type": "text",
                        "text": "Nested text element"
                    },
                    {
                        "type": "image",
                        "src": "example.png"
                    }
                ]
            })),
        };

        let element: Element = serde_json::from_str(json_str).unwrap();
        assert_eq!(element, expected);
    }

    #[test]
    fn test_serialize() {
        let element = Element {
            r#type: "text".to_string(),
            element_id: "1".to_string(),
            text: "Hello, world!".to_string(),
            metadata: None,
        };

        let expected_json =
            r#"{"type":"text","element_id":"1","text":"Hello, world!","metadata":null}"#;
        let json_str = serde_json::to_string(&element).unwrap();
        assert_eq!(json_str, expected_json);
    }
}

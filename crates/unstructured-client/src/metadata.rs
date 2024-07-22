use serde::{Deserialize, Serialize};

/// Struct representing common metadata fields for document elements
/// from all file types.
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct CommonMetadata {
    /// Filename.
    pub filename: Option<String>,

    /// File directory.
    pub file_directory: Option<String>,

    /// Last modified Date.
    pub last_modified: Option<String>,

    /// File type.
    pub filetype: Option<String>,

    /// XY Bounding Box Coordinates.
    /// See notes below for further details about the bounding box.
    pub coordinates: Option<String>,

    /// Element Hierarchy.
    /// `parent_id` may be used to infer where an element resides within the overall hierarchy of a document.
    /// For instance, a NarrativeText element may have a Title element as a parent (a “sub-title”),
    /// which in turn may have another Title element as its parent (a “title”).
    pub parent_id: Option<String>,

    /// Element depth relative to other elements of the same category.
    /// Category depth is the depth of an element relative to other elements of the same category.
    /// It’s set by a document partitioner and enables the hierarchy post-processor to compute more accurate hierarchies.
    /// Category depth may be set using native document hierarchies, e.g. reflecting <H1>, <H2>, or <H3> tags within an HTML document
    /// or the indentation level of a bulleted list item in a Word document.
    pub category_depth: Option<u32>,

    /// HTML representation of extracted tables.
    /// Only applicable to table elements.
    pub text_as_html: Option<String>,

    /// Document Languages.
    /// At document level or element level.
    /// The list is ordered by probability of being the primary language of the text.
    pub languages: Option<Vec<String>>,

    /// Emphasized text (bold or italic) in the original document.
    pub emphasized_text_contents: Option<String>,

    /// Tags on text that is emphasized in the original document.
    pub emphasized_text_tags: Option<String>,

    /// True if the element is a continuation of a previous element.
    /// Only relevant for chunking, if an element was divided into two due to max_characters.
    pub is_continuation: Option<bool>,

    /// Detection model class probabilities.
    /// From unstructured-inference, hi-res strategy.
    pub detection_class_prob: Option<Vec<f64>>,
}

/// Metadata for DOCX, PDF, PPT, XLSX document types.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PagedDocument {
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Page number.
    pub page_number: Option<u32>,
}

/// Metadata for XLSX document type.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExcelMetadata {
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Page number.
    pub page_number: Option<u32>,

    /// Sheet name in an Excel document.
    pub page_name: Option<String>,
}

/// Metadata for EML document type.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EmailMetadata {
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Email sender.
    pub sent_from: Option<String>,

    /// Email recipient.
    pub sent_to: Option<String>,

    /// Email subject.
    pub subject: Option<String>,
}

/// Metadata for MSG document type.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MsgMetadata {
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Filename that attachment file is attached to.
    pub attached_to_filename: Option<String>,
}

/// Metadata for Word Document.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WordDocMetadata {
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Page number.
    pub page_number: Option<u32>,

    /// Pages a header or footer applies to: “primary”, “even_only”, and “first_page”.
    pub header_footer_type: Option<String>,
}

/// Metadata for HTML document type.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HtmlMetadata {
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// The URL associated with a link in a document.
    pub link_urls: Option<Vec<String>>,

    /// The text associated with a link in a document.
    pub link_texts: Option<Vec<String>>,
}

/// Metadata for EPUB document type.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EpubMetadata {
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Book section title corresponding to table of contents.
    pub section: Option<String>,
}

/// Enum representing various types of metadata for different document types.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "filetype")]
pub enum ExtendedMetadata {
    // For DOCX, PDF, PPT, XLSX
    #[serde(rename = "application/pdf")]
    PdfPage(PagedDocument),

    #[serde(rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.document")]
    DocxPage(PagedDocument),

    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.presentation")]
    PptPage(PagedDocument),

    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        alias = "sheet",
        alias = "excel"
    )]
    XlsxPage(ExcelMetadata),

    // For EML
    #[serde(rename = "message/rfc822")]
    Eml(EmailMetadata),

    // For MSG
    #[serde(rename = "application/vnd.ms-outlook")]
    Msg(MsgMetadata),

    // For Word Document (which we'll assume to be DOCX for simplicity)
    #[serde(rename = "application/msword")]
    WordDoc(WordDocMetadata),

    // For HTML
    #[serde(rename = "text/html")]
    Html(HtmlMetadata),

    // For EPUB
    #[serde(rename = "application/epub+zip")]
    Epub(EpubMetadata),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Metadata {
    KnownFormat(ExtendedMetadata),
    UnknownFormat(CommonMetadata),
}

impl Metadata {
    pub fn into_common_metadata(self) -> CommonMetadata {
        match self {
            Metadata::KnownFormat(ext_metadata) => match ext_metadata {
                ExtendedMetadata::PdfPage(m) => m.common,
                ExtendedMetadata::DocxPage(m) => m.common,
                ExtendedMetadata::PptPage(m) => m.common,
                ExtendedMetadata::XlsxPage(m) => m.common,
                ExtendedMetadata::Eml(m) => m.common,
                ExtendedMetadata::Msg(m) => m.common,
                ExtendedMetadata::WordDoc(m) => m.common,
                ExtendedMetadata::Html(m) => m.common,
                ExtendedMetadata::Epub(m) => m.common,
            },
            Metadata::UnknownFormat(metadata) => metadata,
        }
    }
}

impl From<Metadata> for CommonMetadata {
    fn from(value: Metadata) -> Self {
        value.into_common_metadata()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    fn test_metadata_for_mime_type(
        mime_type: &str,
        expected_format: ExtendedMetadata,
    ) -> Result<()> {
        let json_str = r#"
		{
	        "filetype": "<REPLACE>"
	    }
		"#;
        let json_str = json_str.replace("<REPLACE>", mime_type);

        // Deserialize the JSON into the DocumentMetadata enum
        let metadata: Metadata = serde_json::from_str(&json_str).unwrap();

        // Verify deserialization
        match metadata {
            Metadata::KnownFormat(mdi) => {
                assert_eq!(mdi, expected_format);
            }
            _ => panic!("Other Metadata"),
        }

        Ok(())
    }

    #[test]
    fn test_all_known_formats() -> Result<()> {
        let known_formats = vec![
            (
                "application/pdf",
                ExtendedMetadata::PdfPage(PagedDocument {
                    common: CommonMetadata::default(),
                    page_number: None,
                }),
            ),
            (
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                ExtendedMetadata::DocxPage(PagedDocument {
                    common: CommonMetadata::default(),
                    page_number: None,
                }),
            ),
            (
                "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                ExtendedMetadata::PptPage(PagedDocument {
                    common: CommonMetadata::default(),
                    page_number: None,
                }),
            ),
            (
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                ExtendedMetadata::XlsxPage(ExcelMetadata {
                    common: CommonMetadata::default(),
                    page_number: None,
                    page_name: None,
                }),
            ),
            (
                "message/rfc822",
                ExtendedMetadata::Eml(EmailMetadata {
                    common: CommonMetadata::default(),
                    sent_from: None,
                    sent_to: None,
                    subject: None,
                }),
            ),
            (
                "application/vnd.ms-outlook",
                ExtendedMetadata::Msg(MsgMetadata {
                    common: CommonMetadata::default(),
                    attached_to_filename: None,
                }),
            ),
            (
                "application/msword",
                ExtendedMetadata::WordDoc(WordDocMetadata {
                    common: CommonMetadata::default(),
                    page_number: None,
                    header_footer_type: None,
                }),
            ),
            (
                "text/html",
                ExtendedMetadata::Html(HtmlMetadata {
                    common: CommonMetadata::default(),
                    link_urls: None,
                    link_texts: None,
                }),
            ),
            (
                "application/epub+zip",
                ExtendedMetadata::Epub(EpubMetadata {
                    common: CommonMetadata::default(),
                    section: None,
                }),
            ),
        ];

        for (mime_type, expected_format) in known_formats {
            test_metadata_for_mime_type(mime_type, expected_format)?;
        }

        Ok(())
    }
    #[test]
    fn test_pdf_element() -> Result<()> {
        // Example JSON string
        let json_str = r#"
    {
        "filetype": "application/pdf",
        "filename": "example.pdf",
        "file_directory": "/documents",
        "last_modified": "2023-10-01",
        "coordinates": "100,100,200,200",
        "parent_id": "1",
        "category_depth": 2,
        "text_as_html": "<p>Example</p>",
        "languages": ["en", "fr"],
        "emphasized_text_contents": "important",
        "emphasized_text_tags": "<b>",
        "is_continuation": false,
        "detection_class_prob": [0.1, 0.9],
        "page_number": 1
    }
    "#;

        // Deserialize the JSON into the DocumentMetadata enum
        let metadata: Metadata = serde_json::from_str(json_str).unwrap();

        // Verify deserialization
        match metadata {
            Metadata::KnownFormat(mdi) => match mdi {
                ExtendedMetadata::PdfPage(_) => {}
                _ => panic!("Format is not PDF"),
            },
            _ => panic!("Other Metadata"),
        }

        Ok(())
    }

    #[test]
    fn test_unknown_element() -> Result<()> {
        // Example JSON string
        let json_str = r#"
    {
        "filetype": "asdfasdfasdf",
        "filename": "example.pdf",
        "file_directory": "/documents",
        "last_modified": "2023-10-01"
    }
    "#;

        // Deserialize the JSON into the DocumentMetadata enum
        let metadata: Metadata = serde_json::from_str(json_str).unwrap();

        // Verify deserialization
        match metadata {
            Metadata::UnknownFormat(_) => {}
            _ => panic!("Wrong format"),
        }

        Ok(())
    }
}

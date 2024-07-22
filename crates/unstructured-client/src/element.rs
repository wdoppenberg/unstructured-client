use crate::metadata::Metadata;
use serde::{Deserialize, Serialize};

/// Enum representing various types of elements in a document.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ElementType {
    /// An element containing formulas in a document.
    Formula,

    /// An element for capturing text associated with figure captions.
    FigureCaption,

    /// NarrativeText is an element consisting of multiple, well-formulated sentences.
    /// This excludes elements such as titles, headers, footers, and captions.
    NarrativeText,

    /// ListItem is a NarrativeText element that is part of a list.
    ListItem,

    /// A text element for capturing titles.
    Title,

    /// A text element for capturing physical addresses.
    Address,

    /// A text element for capturing email addresses.
    EmailAddress,

    /// A text element for capturing image metadata.
    Image,

    /// An element for capturing page breaks.
    PageBreak,

    /// An element for capturing tables.
    Table,

    /// An element for capturing document headers.
    Header,

    /// An element for capturing document footers.
    Footer,

    /// An element for capturing code snippets.
    CodeSnippet,

    /// An element for capturing page numbers.
    PageNumber,

    /// Base element for capturing free text from within the document.
    UncategorizedText,

    /// A chunk formed from text (non-Table) elements. It is only produced by chunking.
    CompositeElement,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Element {
    pub r#type: ElementType,
    pub element_id: String,
    pub text: String,
    pub metadata: Option<Metadata>,
}

pub type ElementList = Vec<Element>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_simple() {
        let json_str = r#"
        {
          "type": "NarrativeText",
          "element_id": "1",
          "text": "Hello, world!",
          "metadata": null
        }
        "#;

        let expected = Element {
            r#type: ElementType::NarrativeText,
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
          "type": "Image",
          "element_id": "2",
          "text": "An image element"
        }
        "#;

        let expected = Element {
            r#type: ElementType::Image,
            element_id: "2".to_string(),
            text: "An image element".to_string(),
            metadata: None,
        };

        let element: Element = serde_json::from_str(json_str).unwrap();
        assert_eq!(element, expected);
    }

    #[test]
    fn test_deserialize_without_metadata() {
        let json_str = r#"
        {
          "type": "ListItem",
          "element_id": "3",
          "text": "A list element."
        }
        "#;

        let expected = Element {
            r#type: ElementType::ListItem,
            element_id: "3".to_string(),
            text: "A list element.".to_string(),
            metadata: None,
        };

        let element: Element = serde_json::from_str(json_str).unwrap();
        assert_eq!(element, expected);
    }

    #[test]
    fn test_serialize() {
        let element = Element {
            r#type: ElementType::NarrativeText,
            element_id: "1".to_string(),
            text: "Hello, world!".to_string(),
            metadata: None,
        };

        let expected_json =
            r#"{"type":"NarrativeText","element_id":"1","text":"Hello, world!","metadata":null}"#;
        let json_str = serde_json::to_string(&element).unwrap();
        assert_eq!(json_str, expected_json);
    }
}

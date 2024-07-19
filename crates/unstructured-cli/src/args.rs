use clap::Parser;
use unstructured_client::PartitionParameters;

#[derive(Debug, Parser)]
pub struct CliPartitionParameters {
    /// If `True`, return coordinates for each element extracted via OCR. Default: `False`.
    #[clap(long, default_value = "false")]
    coordinates: bool,

    /// The encoding method used to decode the text input. Default: utf-8
    #[clap(long)]
    encoding: Option<String>,

    /// The types of elements to extract, for use in extracting image blocks as base64 encoded data stored in metadata fields. Default: [].
    #[clap(long, default_value = "")]
    extract_image_block_types: Vec<String>,

    /// If file is gzipped, use this content type after unzipping.
    #[clap(long)]
    gz_uncompressed_content_type: Option<String>,

    /// The name of the inference model used when strategy is hi_res
    #[clap(long)]
    hi_res_model_name: Option<String>,

    /// If true, the output will include page breaks if the filetype supports it. Default: false
    #[clap(long, default_value = "false")]
    include_page_breaks: bool,

    /// The languages present in the document, for use in partitioning and/or OCR. See the Tesseract documentation for a full list of languages. Default: [].
    #[clap(long, default_value = "")]
    languages: Vec<String>,

    /// The format of the response. Supported formats are application/json and text/csv. Default: application/json.
    #[clap(long, default_value = "application/json")]
    output_format: String,

    /// The document types that you want to skip table extraction with. Default: [].
    #[clap(long, default_value = "")]
    skip_infer_table_types: Vec<String>,

    /// When PDF is split into pages before sending it into the API, providing this information will allow the page number to be assigned correctly. Introduced in 1.0.27.
    #[clap(long)]
    starting_page_number: Option<i32>,

    /// The strategy to use for partitioning PDF/image. Options are fast, hi_res, auto. Default: auto.
    #[clap(long, default_value = "auto")]
    strategy: String,

    /// When `True`, assign UUIDs to element IDs, which guarantees their uniqueness (useful when using them as primary keys in database). Otherwise a SHA-256 of element text is used. Default: `False`.
    #[clap(long, default_value = "false")]
    unique_element_ids: bool,

    /// If `True`, will retain the XML tags in the output. Otherwise it will simply extract the text from within the tags. Only applies to XML documents. Default: false
    #[clap(long, default_value = "false")]
    xml_keep_tags: bool,

    /// Use one of the supported strategies to chunk the returned elements after partitioning. When 'chunking_strategy' is not specified, no chunking is performed and any other chunking parameters provided are ignored. Supported strategies: 'basic', 'by_page', 'by_similarity', or 'by_title'
    #[clap(long)]
    chunking_strategy: Option<String>,

    /// If chunking strategy is set, combine elements until a section reaches a length of n chars. Default: 500
    #[clap(long)]
    combine_under_n_chars: Option<i32>,

    /// When a chunking strategy is specified, each returned chunk will include the elements consolidated to form that chunk as `.metadata.orig_elements`. Default: true.
    #[clap(long, default_value = "true")]
    include_orig_elements: bool,

    /// If chunking strategy is set, cut off new sections after reaching a length of n chars (hard max). Default: 500
    #[clap(long)]
    max_characters: Option<i32>,

    /// If chunking strategy is set, determines if sections can span multiple sections. Default: true
    #[clap(long, default_value = "true")]
    multipage_sections: bool,

    /// If chunking strategy is set, cut off new sections after reaching a length of n chars (soft max). Default: 1500
    #[clap(long)]
    new_after_n_chars: Option<i32>,

    /// Specifies the length of a string ('tail') to be drawn from each chunk and prefixed to the next chunk as a context-preserving mechanism. By default, this only applies to split-chunks where an oversized element is divided into multiple chunks by text-splitting. Default 0.
    #[clap(long, default_value = "0")]
    overlap: i32,

    /// When `True`, apply overlap between 'normal' chunks formed from whole elements and not subject to text-splitting. Use this with caution as it entails a certain level of 'pollution' of otherwise clean semantic chunk boundaries. Default false.
    #[clap(long, default_value = "false")]
    overlap_all: bool,

    /// A value between 0.0 and 1.0 describing the minimum similarity two elements must have to be included in the same chunk. Note that similar elements may be separated to meet chunk-size criteria; this value can only guarantee that two elements with similarity below the threshold will appear in separate chunks.
    #[clap(long)]
    similarity_threshold: Option<f64>,
}

impl From<CliPartitionParameters> for PartitionParameters {
    fn from(cli_params: CliPartitionParameters) -> Self {
        PartitionParameters {
            coordinates: cli_params.coordinates,
            encoding: cli_params.encoding,
            extract_image_block_types: cli_params.extract_image_block_types,
            gz_uncompressed_content_type: cli_params.gz_uncompressed_content_type,
            hi_res_model_name: cli_params.hi_res_model_name,
            include_page_breaks: cli_params.include_page_breaks,
            languages: cli_params.languages,
            output_format: cli_params.output_format,
            skip_infer_table_types: cli_params.skip_infer_table_types,
            starting_page_number: cli_params.starting_page_number,
            strategy: cli_params.strategy,
            unique_element_ids: cli_params.unique_element_ids,
            xml_keep_tags: cli_params.xml_keep_tags,
            chunking_strategy: cli_params.chunking_strategy,
            combine_under_n_chars: cli_params.combine_under_n_chars,
            include_orig_elements: cli_params.include_orig_elements,
            max_characters: cli_params.max_characters,
            multipage_sections: cli_params.multipage_sections,
            new_after_n_chars: cli_params.new_after_n_chars,
            overlap: cli_params.overlap,
            overlap_all: cli_params.overlap_all,
            similarity_threshold: cli_params.similarity_threshold,
        }
    }
}

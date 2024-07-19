# Unofficial Unstructured Rust client library

Use [Unstructured](https://docs.unstructured.io/welcome)'s API service with this light client library for Rust. 


## Usage example

Either use their platform offering, or spin up an Unstructured API service locally:

```bash
docker run -p 8000:8000 -it downloads.unstructured.io/unstructured-io/unstructured-api:latest
```

```rust
use std::path::PathBuf;
use unstructured_client::{PartitionParameters, UnstructuredClient};

#[tokio::main]
async fn main() -> Result<()> {
    // Define path to file
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

Check out [`partition.rs`](src/partition.rs) for the partition arguments.


## CLI Usage

```text

Usage: unstructured-cli [OPTIONS] --file-path <FILE_PATH>

Options:
      --file-path <FILE_PATH>
          Path to the file to be parsed
      --base-url <BASE_URL>
          The base URL for the Unstructured API [default: http://localhost:8000]
      --coordinates
          If `True`, return coordinates for each element extracted via OCR. Default: `False`
      --encoding <ENCODING>
          The encoding method used to decode the text input. Default: utf-8
      --extract-image-block-types <EXTRACT_IMAGE_BLOCK_TYPES>
          The types of elements to extract, for use in extracting image blocks as base64 encoded data stored in metadata fields. Default: [] [default: ]
      --gz-uncompressed-content-type <GZ_UNCOMPRESSED_CONTENT_TYPE>
          If file is gzipped, use this content type after unzipping
      --hi-res-model-name <HI_RES_MODEL_NAME>
          The name of the inference model used when strategy is hi_res
      --include-page-breaks
          If true, the output will include page breaks if the filetype supports it. Default: false
      --languages <LANGUAGES>
          The languages present in the document, for use in partitioning and/or OCR. See the Tesseract documentation for a full list of languages. Default: [] [default: ]
      --output-format <OUTPUT_FORMAT>
          The format of the response. Supported formats are application/json and text/csv. Default: application/json [default: application/json]
      --skip-infer-table-types <SKIP_INFER_TABLE_TYPES>
          The document types that you want to skip table extraction with. Default: [] [default: ]
      --starting-page-number <STARTING_PAGE_NUMBER>
          When PDF is split into pages before sending it into the API, providing this information will allow the page number to be assigned correctly. Introduced in 1.0.27
      --strategy <STRATEGY>
          The strategy to use for partitioning PDF/image. Options are fast, hi_res, auto. Default: auto [default: auto]
      --unique-element-ids
          When `True`, assign UUIDs to element IDs, which guarantees their uniqueness (useful when using them as primary keys in database). Otherwise a SHA-256 of element text is used. Default: `False`
      --xml-keep-tags
          If `True`, will retain the XML tags in the output. Otherwise it will simply extract the text from within the tags. Only applies to XML documents. Default: false
      --chunking-strategy <CHUNKING_STRATEGY>
          Use one of the supported strategies to chunk the returned elements after partitioning. When 'chunking_strategy' is not specified, no chunking is performed and any other chunking parameters provided are ignored. Supported strategies: 'basic', 'by_page', 'by_similarity', or 'by_title'
      --combine-under-n-chars <COMBINE_UNDER_N_CHARS>
          If chunking strategy is set, combine elements until a section reaches a length of n chars. Default: 500
      --include-orig-elements
          When a chunking strategy is specified, each returned chunk will include the elements consolidated to form that chunk as `.metadata.orig_elements`. Default: true
      --max-characters <MAX_CHARACTERS>
          If chunking strategy is set, cut off new sections after reaching a length of n chars (hard max). Default: 500
      --multipage-sections
          If chunking strategy is set, determines if sections can span multiple sections. Default: true
      --new-after-n-chars <NEW_AFTER_N_CHARS>
          If chunking strategy is set, cut off new sections after reaching a length of n chars (soft max). Default: 1500
      --overlap <OVERLAP>
          Specifies the length of a string ('tail') to be drawn from each chunk and prefixed to the next chunk as a context-preserving mechanism. By default, this only applies to split-chunks where an oversized element is divided into multiple chunks by text-splitting. Default 0 [default: 0]
      --overlap-all
          When `True`, apply overlap between 'normal' chunks formed from whole elements and not subject to text-splitting. Use this with caution as it entails a certain level of 'pollution' of otherwise clean semantic chunk boundaries. Default false
      --similarity-threshold <SIMILARITY_THRESHOLD>
          A value between 0.0 and 1.0 describing the minimum similarity two elements must have to be included in the same chunk. Note that similar elements may be separated to meet chunk-size criteria; this value can only guarantee that two elements with similarity below the threshold will appear in separate chunks
  -h, --help
          Print help
```
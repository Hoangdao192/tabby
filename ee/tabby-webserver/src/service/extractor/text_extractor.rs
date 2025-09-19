use docx_rust::DocxFile;
use std::path::Path;

pub trait TextExtractor {
    fn extract(file_path: &Path) -> Result<String, String>;

    fn detect_extractor(file_path: &Path) -> Option<impl TextExtractor> {
        if let Some(extension) = file_path.extension() {
            match extension.to_str() {
                Some(value) => match value.to_lowercase().as_str() {
                    "doc" => Some(WordExtractor {}),
                    "docx" => Some(WordExtractor {}),
                    _ => None
                },
                None => None
            }
        } else {
            None
        }
    }
}

struct WordExtractor;
impl TextExtractor for WordExtractor {
    fn extract(file_path: &Path) -> Result<String, String> {
        let extension = file_path.extension().unwrap().to_str().unwrap();
        if extension.to_lowercase() == "doc" {

        } else if extension.to_lowercase() == "docx" {
            let docx = DocxFile::from_file(file_path).unwrap();
            let document = docx.parse().unwrap();
            return Ok(document.document.body.text());
        }
        Err("Unsupported file extension".to_string())
    }
}
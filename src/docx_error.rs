use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DocxError {
    TemplateNotFound,
}

impl fmt::Display for DocxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DocxError::TemplateNotFound => f.write_str("template.docx not found"),
        }
    }
}

impl Error for DocxError {
    fn description(&self) -> &str {
        match *self {
            DocxError::TemplateNotFound => "template.docx not found",
        }
    }
}

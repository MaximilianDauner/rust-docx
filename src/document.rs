use docx_error::DocxError;
use std::env::current_dir;
use std::fs::{metadata, rename, File};
use std::io::prelude::{Read, Seek};
use std::path::{Path, PathBuf};
use zip::read::{ZipArchive, ZipFile};
use zip::result::ZipResult;

#[derive(Debug)]
pub struct Document {
    pub file_path: String,
}

fn template_docx_path() -> Result<PathBuf, DocxError> {
    match current_dir() {
        Ok(current_path) => Ok([current_path, Path::new("template").join("template.docx")]
            .iter()
            .collect()),
        Err(_) => Err(DocxError::TemplateNotFound),
    }
}

impl Document {
    pub fn new() -> Result<Document, DocxError> {
        let template_docx = template_docx_path()?;
        let file = File::open(template_docx_path()?).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        for i in 0..archive.len() {
            let mut f = archive.by_index(i).unwrap();
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).unwrap();
            println!("----------------------------");
            println!("{}", f.name());
            println!("----------------------------");
            println!("{}", buffer);
        }
        let mut zip = current_dir().unwrap();
        zip.push("template");
        zip.push("template.zip");
        println!("{:?}", zip);
        /*
        let mut template_docx =
        let mut docx_path = current_dir().push("template").unwrap();
        let mut zip_path = current_dir().unwrap();
        docx_path.push("template.docx");
        zip_path.push("template.zip");
        rename(docx_path.to_str().unwrap(), zip_path.to_str().unwrap());
        */
        //let mut file = File::create("template.zip").unwrap();
       // Document { file_path: docx_path.to_str().unwrap().to_string() }
        /*
        rename(cur_dir.to_str().unwrap(), zip.to_str().unwrap());

        let mut file = File::open(&zip).expect("Couldn't open file");
        let files = browse_zip_archive(&mut file, |f| {

            Ok(format!("{}: {} -> {}", f.name(), f.size(), f.compressed_size()))
        });
        println!("{:?}", files);
        rename(zip.to_str().unwrap(), cur_dir.to_str().unwrap());
        Document { file_path: "t".to_string()}
    }

    pub fn from(path: String) -> Document {
        let metadata = metadata(&path).unwrap();
        if metadata.is_file() == false {
            println!("File nicht gefunden");
        }
        */
        Ok(Document {
            file_path: "t".to_string(),
        })
    }
}

use std::fs::{File, metadata, rename};
use std::env::{current_dir};
use std::io::prelude::{Seek, Read};
use std::io::{Result};
use zip::result::ZipResult;
use zip::read::{ZipArchive, ZipFile};

#[derive(Debug)]
pub struct Document {
    pub file_path: String,
}


pub fn browse_zip_archive<T, F, U>(buf: &mut T, browse_func: F) -> ZipResult<Vec<U>>
    where T: Read + Seek,
          F: Fn(&ZipFile) -> ZipResult<U>
{
    let mut archive = ZipArchive::new(buf)?;
    (0..archive.len())
        .map(|i| archive.by_index(i).and_then(|file| browse_func(&file)))
        .collect()
}

fn convert_file(source_file: &str, destination_file: &str) -> Result<()> {
    rename(source_file, destination_file)?;
    Ok(())
}

impl Document {
    pub fn new() -> Document {
        let mut cur_dir = current_dir().unwrap();
        cur_dir.push("template");
        cur_dir.push("template.docx");
        println!("{:?}", cur_dir);

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
        Document { file_path: path }
    }
}
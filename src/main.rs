extern crate rustdocx;

use rustdocx::document;

fn main() {
    let doc = document::Document::new().unwrap();
    //let doc = document::Document::from("/home/robolab/Schreibtisch/rust-docx/template.docx".into());
    //println!("{:?}", doc.file_path);
}
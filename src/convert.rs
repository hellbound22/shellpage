use std::{fs::File, io::Read};

use markdown::to_html;

use crate::ConfigFile;

pub fn md_to_html(config: &ConfigFile, file_name: &str) -> String {
    let mut file_contents = String::new();
    let mdfile_path = format!("{}{}.md", config.md_storage, file_name);
    
    let mut file = File::open(mdfile_path).unwrap();
    file.read_to_string(&mut file_contents).unwrap();

    let html = to_html(&file_contents);
    html
}

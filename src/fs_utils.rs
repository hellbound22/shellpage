use std::{fs::File, io::Write};

pub fn write_overwrite(path: &str, content: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}


pub fn write_new(path: &str, content: &str) {
    let mut file = File::create_new(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

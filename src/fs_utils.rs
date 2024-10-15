use crate::errors::ShellpageError;


use std::{fs::File, io::{Write, Read}};

pub fn write_overwrite(path: &str, content: &str) -> Result<(), ShellpageError> {
    let mut file = if let Ok(f) = File::create(path) {
        f
    } else {
        return Err(ShellpageError::UnableToWrite(path.to_owned()))
    };
    if let Err(_e) = file.write_all(content.as_bytes()) {
        return Err(ShellpageError::UnableToWrite(path.to_owned()))
    }

    Ok(())
}


pub fn write_new(path: &str, content: &str) -> Result<(), ShellpageError> {
    let mut file = if let Ok(f) = File::create_new(path) {
        f
    } else {
        return Err(ShellpageError::UnableToWrite(path.to_owned()))
    };
    if let Err(_e) = file.write_all(content.as_bytes()) {
        return Err(ShellpageError::UnableToWrite(path.to_owned()))
    }

    Ok(())
}

pub fn read(path: &str) -> Result<String, ShellpageError> {
    let mut file_contents = String::new();
    
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut file_contents).unwrap();

    Ok(file_contents)
}

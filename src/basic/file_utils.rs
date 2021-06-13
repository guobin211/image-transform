use std::fs::File;
use std::io::Result;
use std::io::{ErrorKind, Read};

#[allow(dead_code)]
pub(crate) fn get_text_from_file(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;
    let mut s: String = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
pub(crate) fn read_text_from_file(path: &str) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s: String = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
pub(crate) fn open_file(file_path: &str) {
    let f = File::open(file_path);
    match f {
        Ok(file) => {
            println!("open file success {:?}", file);
        }
        Err(err) => {
            println!("open file error {:?}", err);
        }
    }
}

/// open file or create file
#[allow(dead_code)]
pub(crate) fn open_or_create(filename: &str) -> bool {
    return match File::open(filename) {
        Ok(file) => {
            println!("open file {}, {:?}", filename, file);
            true
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("{} is not found", filename);
                let new_file = File::create(filename);
                match new_file {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            other_err => {
                println!("open file error : {:?}", other_err);
                false
            }
        },
    };
}

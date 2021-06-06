use std::fs::File;
use std::io;
use std::io::Read;

#[allow(dead_code)]
pub(crate) fn read_text_from_file(path: &str) -> Result<String, io::Error> {
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

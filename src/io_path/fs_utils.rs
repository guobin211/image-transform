use std::fs;
use std::path;

#[allow(dead_code)]
pub fn run_fs_utils() {
    let target = "/Users/guobin/idea/image-transform/local/data";
    if create_dir_all(target) {
        println!("create dir success {}", target);
        let file = String::from(target) + &String::from(".txt");
        write_file_sync(file, "input some data");
    } else {
        println!("create dir failed {}", target);
    }
}

/// 写入文件
#[allow(dead_code)]
pub fn write_file_sync(file: String, data: &str) -> bool {
    match fs::write(file, data) {
        Ok(_) => true,
        _ => false,
    }
}

/// 创建目录
#[allow(dead_code)]
pub fn create_dir_all(dirname: &str) -> bool {
    match fs::create_dir_all(dirname) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[allow(dead_code)]
pub fn is_directory(file_path: &str) -> bool {
    let dir = path::Path::new(file_path);
    match fs::metadata(dir) {
        Ok(metadata) => metadata.is_dir(),
        _ => false,
    }
}

#[allow(dead_code)]
pub fn is_file(file_path: &str) -> bool {
    let dir = path::Path::new(file_path);
    match fs::metadata(dir) {
        Ok(metadata) => metadata.is_file(),
        _ => false,
    }
}

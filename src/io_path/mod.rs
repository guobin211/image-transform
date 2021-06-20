mod fetch_utils;
mod fs_utils;
mod png_utils;
mod thread_lock;

#[allow(unused_imports)]
use crate::io_path::fs_utils::run_fs_utils;
#[allow(unused_imports)]
use crate::io_path::png_utils::run_png_min;
#[allow(unused_imports)]
use crate::io_path::thread_lock::generate_workout;
use fetch_utils::fetch_json as other_fetch_json;
use std::fs;
use std::path;
use std::result::Result::Ok;

#[allow(dead_code)]
pub fn run_file_io() {
    println!("run_file_io");
    generate_workout(10, 1);
    run_fs_utils();
    // run_png_min("/Users/guobin/idea/image-transform/local/banner_bg.png");
    // run_png_min("/Users/guobin/idea/image-transform/local/android_2x.png");
}

/// 遍历文件夹目录
#[allow(dead_code)]
pub fn read_dir_sync(file_path: &str) -> Vec<fs::DirEntry> {
    let mut res: Vec<fs::DirEntry> = vec![];
    let dir = path::Path::new(file_path);
    println!("path is {:?}", dir);
    if let Ok(files) = fs::read_dir(dir) {
        for file in files {
            if let Ok(file_info) = file {
                if let Ok(metadata) = file_info.metadata() {
                    // println!("{:?}: {:?}", file_info.path(), file_info.file_name());
                    println!("permissions: {:?}", metadata.permissions());
                    res.push(file_info)
                } else {
                    println!("Couldn't get metadata for {:?}", file_info.path());
                }
            }
        }
    }

    res
}

#[allow(dead_code)]
pub async fn fetch_json() -> Result<(), Box<dyn std::error::Error>> {
    return other_fetch_json();
}

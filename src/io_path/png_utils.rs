use oxipng::{optimize, OutFile};
use oxipng::{InFile, Options};
use std::path;

/// png图片压缩, 改变源文件
#[allow(dead_code)]
pub fn run_png_min(filename: &str) {
    let dir = path::Path::new(filename);
    if let Some(name) = dir.file_name() {
        let input = InFile::Path(dir.to_path_buf());
        let out_path: &str = &filename.to_string().replace(".png", "_min.png");
        let out = OutFile::Path(Some(path::Path::new(out_path).to_path_buf()));
        let option = Options::max_compression();
        match optimize(&input, &out, &option) {
            Ok(_) => {
                println!("success input {:?} ==> output {}", name, out_path);
            }
            Err(err) => {
                println!("optimize error {:?}", err)
            }
        };
    }
}

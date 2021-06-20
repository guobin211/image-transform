mod basic;
mod generic_type;
mod io_path;
mod link_list;

#[allow(unused_imports)]
use crate::basic::run_basic_mod;
#[allow(unused_imports)]
use crate::generic_type::run_generic_type;
#[allow(unused_imports)]
use crate::io_path::run_file_io;
#[allow(unused_imports)]
use crate::link_list::run_link_list;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

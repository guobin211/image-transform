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

#[allow(dead_code)]
async fn get() {
    let resp = reqwest::get("https://httpbin.org/ip");
    match resp.await {
        Ok(resp) => {
            println!("{:#?}", resp);
            let text = resp.json::<HashMap<String, String>>();
            match text.await {
                Ok(json) => {
                    println!("{:#?}", json);
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

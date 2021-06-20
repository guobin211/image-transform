use std::collections::HashMap;

#[allow(dead_code)]
#[tokio::main]
pub async fn fetch_json() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[allow(dead_code)]
pub async fn get() {
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

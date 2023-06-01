use std::cmp::min;
use std::fs::File;
use std::io::Write;

use reqwest::Client;
use indicatif::{ProgressBar};
use futures_util::StreamExt;
use std::collections::HashMap;

pub async fn download_file(client: &Client, url: &str, query_parameters: Vec<(String, String)>, path: &str) -> Result<(), String> {

    let mut query_params: HashMap<String, String> = HashMap::new();

    for param in query_parameters.into_iter() {
        query_params.insert(param.0, param.1);
    }

    // Reqwest setup
    let res = client
        .get(url)
        .query(&query_params)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    
    // Indicatif setup
    let pb = ProgressBar::new(total_size);

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.unwrap();
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    // pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(());
}

#[tokio::main]
async fn main() {

    let mut param: Vec<(String, String)> = Vec::new();

    param.push((String::from("api-key"), String::from("8acbcf1e-732c-4574-a3bf-27e6a85b86f1")));
    param.push((String::from("token"), String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhZ2VudCI6Ik1vemlsbGEvNS4wIChNYWNpbnRvc2g7IEludGVsIE1hYyBPUyBYIDEwLjE1OyBydjoxMDkuMCkgR2Vja28vMjAxMDAxMDEgRmlyZWZveC8xMTEuMCIsInJlbW90ZUFkZHJlc3MiOiIxMDIuODguMzUuMTU4IiwiZG9tYWluIjoieXRzLmphbXNiYXNlLmNvbSIsImV4cCI6MTY4NjA3Mjg5MSwic2Vzc2lvbklEIjoiYTFvSkNxdTFVSnQ2akstUC1MV1RRT2xCYkhsdTY5eXgiLCJyYXRlIjoiMTBNIiwicm9sZSI6Im5vYm9keSJ9.Cg-cXx9tXYJvFX98gkLIP4P9XCpvRzL_JwKme4AvM8M")));
    param.push((String::from("user-id"), String::from("429e8bbf403bbc730c77e5161ab1eafb")));
    param.push((String::from("download-id"), String::from("7bd0bda52022be53eca394413e09d79b")));

    download_file(
        &Client::new(),
        &String::from("https://abra--67910d27.api.frosty-night.buzz/d4cc099ee6c1b77de7938ff0e7f389d4813d21bb/The%20Little%20Mermaid%20(1989)%20%5BREPACK%5D%20%5B720p%5D%20%5BBluRay%5D%20%5BYTS.MX%5D%2FThe.Little.Mermaid.1989.REPACK.720p.BluRay.x264.AAC-%5BYTS.MX%5D.mp4"),
        param,
        "little_mermaid.mp4"
    ).await.unwrap()
}

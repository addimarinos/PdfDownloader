use crate::structs::UrlRecord;
use reqwest::{Client, Response, header::{ACCEPT, ACCEPT_ENCODING, HeaderValue, USER_AGENT}};
use tokio::{fs::File, io::{AsyncBufReadExt, AsyncWriteExt}};
use std::{path::{Path, PathBuf}, time::Duration};
use futures_util::{StreamExt, join, stream};

enum DownloadResult {
    Success,
    AltSuccess,
    NoResponse,
    NoFileBytes, 

}

pub async fn download_all(all_records: Vec<UrlRecord>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .build()?;

    let output_dir = Path::new("downloads/").to_path_buf();
    tokio::fs::remove_dir_all(&output_dir).await?;
    tokio::fs::create_dir_all(&output_dir).await?;

    // try downloading one file
    let mut record = stream::iter(all_records.into_iter().map(|rec| {
        let client = client.clone();
        let output_dir = output_dir.clone();

        async move {
            download_file(rec, &output_dir, &client).await
        }
    })).buffer_unordered(10).collect::<Vec<_>>().await;



    //let mut records = stream::iter(all_records.into_iter())

    Ok(())
}

pub async fn download_file(
    mut record: UrlRecord, 
    output_dir: &Path, 
    client: &reqwest::Client
) -> Result<UrlRecord, Box<dyn std::error::Error + Send + Sync>> {
    
    let id = record.id;
    
    let mut urls: Vec<&str> = vec![record.url.as_str()];
    if let Some(alt_url) = record.alturl.as_deref() {
            println!("adding alt url for id {} : {}", id, alt_url);
            urls.push(alt_url);
    }

    let file_path = output_dir.join(format!("{}.pdf", id));
    
    // lets build the header
    let mut headers = reqwest::header::HeaderMap::new();
    let src_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:157.0) Gecko/20100101 Firefox/157.0";
    let src_accept = "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8";
    let src_accept_encoding = "gzip, deflate, br, zstd";
    headers.insert(USER_AGENT, HeaderValue::from_static(src_user_agent));
    headers.insert(ACCEPT, HeaderValue::from_static(src_accept));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static(src_accept_encoding));




    for url in urls {
        let response = match client.get(url)
            .headers(headers.clone())
            .send()
            .await?
            .error_for_status() {
                Ok(res) => {
                    println!("No problem fetching pdf with id {}", id);
                    let content = res.bytes().await?;
                    let mut file = tokio::fs::File::create(&file_path).await?;
                    file.write_all(&content).await?;
                    record.status = true;
                    //break;
                }
                Err(err) => {
                    println!("Error fetching pdf with id {}: {}", id, err);
                    println!("request for id {} : {:?}", id, err.status());
                    record.status = false;
                } 
            };
            
        }
    
    Ok(record)
}
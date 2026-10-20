use crate::structs::UrlRecord;
use log::{info, warn};
use reqwest::{Client, Response, header::{ACCEPT, HeaderValue, USER_AGENT}};
use tokio::io::AsyncWriteExt;
use std::{io::ErrorKind::{InvalidData, InvalidFilename}, path::{Path, PathBuf}, time::Duration};
use futures_util::{StreamExt, stream::self};



pub async fn download_all(all_records: Vec<UrlRecord>) -> Result<Vec<UrlRecord>, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .build()?;

    let output_dir = Path::new("downloads/").to_path_buf();
    if !tokio::fs::try_exists(&output_dir).await? {
        tokio::fs::create_dir_all(&output_dir).await?;
        //tokio::fs::remove_dir_all(&output_dir).await?;
    }
    
    let start = std::time::Instant::now();

    let updated_records = stream::iter(all_records.into_iter().map(|rec| {
        let client = client.clone();
        let output_dir = output_dir.clone();

        async move {
            download_file(rec, &output_dir, &client).await
        }
    })).buffer_unordered(20).collect::<Vec<_>>().await;

    
    println!("Time it took: {} secs", start.elapsed().as_secs());
    

    let  count_downl_records = updated_records
        .iter()
        .filter(|record| record.status)
        .count();
    println!("Downloaded files: {} / {}", count_downl_records, updated_records.len());
    println!("Failed to download the following records:");
    let  failed_downl_records = updated_records
        .iter()
        .filter(|record| !record.status);
    for rec in failed_downl_records {
        println!("id: {} | status {}", rec.id, rec.status);
    }

    Ok(updated_records)
}

pub async fn download_file(
    mut record: UrlRecord, 
    output_dir: &Path, 
    client: &reqwest::Client
) -> UrlRecord { //Result<UrlRecord, Box<dyn std::error::Error + Send + Sync>> {
    if record.status {
        println!("file with id {} already downloaded", {record.id});
        return record;
    }

    let id = record.id;
    
    let mut urls: Vec<&str> = vec![record.url.as_str()];
    if let Some(alt_url) = record.alturl.as_deref() {
            //println!("adding alt url for id {} : {}", id, alt_url);
            urls.push(alt_url);
    }

    let file_path = output_dir.join(format!("{}.pdf", id));
    
    // lets build the header
    let mut headers = reqwest::header::HeaderMap::new();
    let src_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:157.0) Gecko/20100101 Firefox/157.0";
    let src_accept = "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8";
    headers.insert(USER_AGENT, HeaderValue::from_static(src_user_agent));
    headers.insert(ACCEPT, HeaderValue::from_static(src_accept));




    for url in urls {
        let response = client.get(url)
            .headers(headers.clone())
            .send()
            .await; 
        
        match response {
            Ok(res) => {
                match res.error_for_status() {
                    
                    Ok(res) => {
                        //println!("No problem fetching pdf with id {}", id);
                        /* let content = res.bytes().await.unwrap();
                        let mut file = tokio::fs::File::create(&file_path).await.unwrap();
                        file.write_all(&content).await; */
                        match write_file(res, &file_path).await {
                            Ok(_res) => {
                                record.status = true;
                                break;
                            }
                            Err(err) => {
                                warn!("Failed to write file with id {} and Error: {}", id, err);
                            }
                        }
                    }
                    Err(err) => {
                        warn!("Error downloading file with id {} : {}", id, err);
                    }
                }
            }
            Err(err) => {
                warn!("Network error - sending request failed for id {} : {}", id, err);
            }
        }
    }
    
    record
}

async fn write_file(response: Response, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    match response.bytes().await {
        /* Ok(content) if content.is_empty() => {
            Err("content is empty");
        } */
        Ok(content)
            if content[..content.len().min(1024)]
                .windows(5)
                .any(|w| w == b"%PDF-") => {
            match tokio::fs::File::create(file_path).await {
                Ok(mut file) => {
                    match file.write_all(&content).await {
                        Ok(_res) => {
                            info!("Successfully wrote content to file in path : {:?}", file_path);
                            file.flush().await?;
                        }
                        Err(err) => {
                            file.flush().await?;
                            warn!("Failed to write content to file in path : {:?} | Error: {}", file_path, err);
                            return Err(std::io::Error::new( InvalidData, "Failed to write content to file").into());

                        }
                    }
                }
                Err(err) => {
                    warn!("Failed to create file with path: {:?} | Error: {}", file_path, err);
                    return Err(std::io::Error::new( InvalidFilename, "Failed to create file in given path").into());

                }
            }
        }
        Ok(_content) => {
            //println!(" might not be a pdf ?");
            warn!("Pdf signature not found in first 1024 bytes of content");
            //println!("The first 5 bytes does not match pdf signature: {:?}", &content[..content.len().min(5)]);
            return Err(std::io::Error::new(InvalidData, "PDF signature not found, possibly not a pdf").into());
        }
        Err(err) => {
            warn!("Failed reading content from file with Error: {}", err);
            return Err(std::io::Error::new(InvalidData, "Failed reading content from file").into())
        }
    }
    Ok(())
}
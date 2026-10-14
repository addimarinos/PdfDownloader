use crate::structs::UrlRecord;
use async_downloader::{Downloader, Job, Progress, BatchProgress, Settings, Threads};
use reqwest::{Client, Response};
use tokio::{fs::File, io::{AsyncBufReadExt, AsyncWriteExt}};
use std::{path::{Path, PathBuf}, time::Duration};
use futures_util::{StreamExt, join, stream};

enum DownloadResult {
    Success,
    NoResponse,
    NoFileBytes, 

}


pub async fn download_url(urls: Vec<UrlRecord>) -> bool {

    let output_path = Path::new("downloads");
    let mut batch_list: Vec<Job> = vec![];
    for record in urls.iter(){
        batch_list.push(Job::new(record.url.to_string(), output_path));
    }
    
    let downloader = Downloader::new(Settings {
        threads: Threads::Auto,
        max_concurrent: 4
    })
    .expect("runtime");

    let batch = downloader.enqueue_batch(batch_list);

    let p = batch.progress();
    println!("{}/{} files", p.done, p.files);
    
    /* let BatchProgress { files, done, failed, cancelled, bytes_received, bytes_total, speed } = batch.progress() {
        println!("Files: {files} - Done: {done:?} - Failed: {failed} - Cancelled: {cancelled}");
        println!("Bytes received: {bytes_received} / Total: {bytes_total:?} - Speed: {speed}");
    } */

    return false;
}

pub async fn download_all(all_records: Vec<UrlRecord>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .build()?;

    let output_dir = Path::new("downloads/").to_path_buf();
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
    record: UrlRecord, 
    output_dir: &Path, 
    client: &reqwest::Client
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let url = "https://www.novonordisk.com/content/dam/nncorp/global/en/investors/irmaterial/annual_report/2025/novo-nordisk-annual-report-2024.pdfst";
    
    let id = record.id;
    let main_url = record.url;
    let alt_url = record.alturl;
    println!("Trying to Download: {} ", main_url);

    let file_path = output_dir.join(format!("{}.pdf", record.id));

    let response = client.get(&main_url)
        .send()
        .await?
        .error_for_status()?;

    let content = response.bytes().await?;

    let mut file = tokio::fs::File::create(&file_path).await?;
    file.write_all(&content).await?;
    Ok(())
}
use std::path::Path;
use tokio::io::AsyncWrite;
use reqwest;

mod structs;
use structs::UrlRecord;
mod excelreadwrite;
use excelreadwrite::parse_file;
mod downloader;

use crate::downloader::{download_all, download_file};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new("src")
        .join("input")
        .join("GRI_2017_2025.xlsx");
    let test_path = Path::new("src")
        .join("input")
        .join("testfile.xlsx");
    

    let mut all_records = parse_file(&test_path)
        .expect("Excel parsing failed");

    println!("Found {:} rows in excel file.", all_records.len());    
    
    println!("Starting downloads...");
    //let _result = download_url(_res).await;
    let _response = download_all(all_records).await?;
    
    println!("Done dowloading files.");
    println!("Lets look at the records...");
    /* for record in all_records {
        println!("Status of pdf with id {}: ", record.id);

    } */

    Ok(())
}

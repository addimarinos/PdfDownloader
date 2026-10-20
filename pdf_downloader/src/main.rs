use std::path::Path;

mod structs;
mod excelreadwrite;
use clap::Parser;
use excelreadwrite::{parse_file, write_to_excel};
mod downloader;
use crate::downloader::download_all;
mod cli;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* let input_path = Path::new("src")
        .join("input")
        .join("GRI_2017_2025.xlsx");
    let test_path = Path::new("src")
        .join("input")
        .join("testfile.xlsx"); */

    
    let cli = Cli::parse();

    let path = cli.path;
    let reset = cli.reset;
    println!("reset input: {}", reset);

    let cli_input_file = Path::new(&path);


    let all_records = parse_file(&cli_input_file, reset)
        .expect("Excel parsing failed");

    println!("Found {:} rows in excel file.", all_records.len());    
    
    println!("Starting downloads...");
    let downloads_records = download_all(all_records).await?;
    
    println!("Done dowloading files.");
    println!("Lets write to file");
    let output_path = Path::new("output/");
    if !tokio::fs::try_exists(output_path).await? {
        tokio::fs::create_dir_all(&output_path).await?;
    }
    let _other_res = write_to_excel(downloads_records, &output_path)?;

    println!("Downloads complete!");
    Ok(())
}

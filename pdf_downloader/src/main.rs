use std::path::Path;

mod structs;
use structs::UrlRecord;
mod excelreadwrite;
use excelreadwrite::parse_file;
mod downloader;
use downloader::download_url;
fn main() {
    let path = Path::new("src")
        .join("input")
        .join("GRI_2017_2025.xlsx");
    let testpath = Path::new("src")
        .join("input")
        .join("testfile.xlsx");
    let mut _res = parse_file(&testpath)
        .expect("Excel to vector failed");    
    
    let _result = download_url(_res);
    
    println!("Done!")
}

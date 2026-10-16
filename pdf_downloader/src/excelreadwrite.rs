use excelstream::streaming_reader::StreamingReader;
use excelstream::{CellValue, ExcelReader};
use std::error::Error;
use std::path::Path;    
use crate::structs::UrlRecord;



pub fn parse_file(path: &Path) -> Result<Vec<UrlRecord>, Box<dyn Error>> {

    //let mut rows: [Vec<CellValue>];
    let mut _urls: Vec<UrlRecord> = vec![];
    let mut reader = StreamingReader::open(path)
        .expect("Failed to open excel file");

    for row_result in reader.stream_rows("Sheet1")?.skip(1) {
        let row = row_result?;
        let id = row[0].as_i64().unwrap();
        let url = row[1].as_string();
        let mut alt = None;
        if let Some(cell) = row.get(2) {
            let value = cell.to_string();
            let value = value.trim();
            if !value.is_empty() {
                alt = Some(value.to_string());
            }
        }

        let info = UrlRecord {
            id: id,
            url: url,
            alturl: alt,
            status: false //row[3].as_bool().is_some_and(|x| x == true)
        };

        _urls.push(info);
    }
    println!("Length of urls vector: {:?}", _urls.len());

    Ok(_urls)
}


pub async fn write_results_to_file(all_records: Vec<UrlRecord>, path: &Path) -> Result<(), Box<dyn Error>> {
    
    
    
    Ok(())
}
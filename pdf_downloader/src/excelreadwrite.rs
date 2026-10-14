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
        println!("Row: {:?}", row);
        let id = row[0].as_i64().unwrap();
        let url = row[1].as_string();
        let mut alt = "".to_string();
        if row.len() > 2 {
            alt = row[2].to_string();
        }

        let info = UrlRecord {
            id: id,
            url: url,
            alturl: Some(alt),
            status: false //row[3].as_bool().is_some_and(|x| x == true)
        };

        _urls.push(info);
    }
    println!("Length of urls vector: {:?}", _urls.len());

    Ok(_urls)
}

use excelstream::streaming_reader::StreamingReader;
use std::error::Error;
use std::path::Path;    

use rust_xlsxwriter::{FormatAlign, Format, Workbook, XlsxError};

use crate::structs::UrlRecord;



pub fn parse_file(path: &Path, reset: bool) -> Result<Vec<UrlRecord>, Box<dyn Error>> {

    //let mut rows: [Vec<CellValue>];
    let mut _urls: Vec<UrlRecord> = vec![];
    let mut reader = StreamingReader::open(path)
        .expect("Failed to open excel file");

    let worksheets = reader.sheet_names();

    for row_result in reader.stream_rows(worksheets[0].as_str())?.skip(1) {
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
        
        let mut status = false;
        if let Some(cell) = row.get(3) && !reset {
            let downl = cell.as_bool();
            if !downl.is_none() {
                status = downl.unwrap();
            }
        }

        let info = UrlRecord {
            id: id,
            url: url,
            alturl: alt,
            status: status 
        };

        _urls.push(info);
    }
    println!("Length of urls vector: {:?}", _urls.len());

    Ok(_urls)
}

pub fn write_to_excel(mut all_records: Vec<UrlRecord>, path: &Path) -> Result<(), Box<dyn Error>> {
    let file_path = path.join("blegh.xlsx");
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 5)?;
    worksheet.set_column_width(1, 60)?;
    worksheet.set_column_width(2, 60)?;
    worksheet.set_column_width(3, 12)?;
    let wrap_format = Format::new().set_text_wrap();
    let center_format = Format::new().set_align(FormatAlign::Center);

    let header = ["id", "url", "alt url", "downloaded?"];
    all_records.sort_by_key(|r|r.id.clone());
    
    worksheet.write_row_with_format(0,0 ,header, &center_format)?;
    let mut row_idx = 1;
    for rec in all_records {
        worksheet.write(row_idx, 0, rec.id)?;
        worksheet.write_string_with_format(row_idx, 1, rec.url, &wrap_format)?;
        worksheet.write_string_with_format(row_idx, 2, rec.alturl.unwrap_or_default(), &wrap_format)?;
        worksheet.write_boolean(row_idx, 3, rec.status)?;
        row_idx += 1;
    }
    

    workbook.save(file_path)?;
    
    Ok(())
}
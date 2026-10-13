use crate::structs::UrlRecord;
use async_downloader::{Downloader, Job, Progress, Settings, Threads};
use std::path::Path;

pub fn download_url(urls: Vec<UrlRecord>) -> bool {

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
    
    while !batch.is_finished() {
    }

    return false;
}
use clap::Parser;

#[derive(Parser)]
#[command(name = "pdfDownloader", version = "1.0")]
#[command(about = "Give me a list of urls in an Excel file and I'll download them all!")]
pub struct Cli {
    #[arg(help="Path to .xlsx (Excel) file")]
    pub path: String,
    #[arg(short, long, help="Enable reset flag to ignore already downloaded files")]
    pub reset: bool,
}
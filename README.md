# PdfDownloader

PdfDownloader reads document links from an Excel workbook, downloads PDFs, and
writes a workbook with the download status for each record.

## Requirements

- Rust and Cargo
- An `.xlsx` workbook in the format described below

## Input workbook

The program reads the first worksheet and skips its header row.

| Column | Value | Required |
| --- | --- | --- |
| A | Integer ID; used as the PDF filename | Yes |
| B | Primary URL | Yes |
| C | Alternate URL | No |
| D | Downloaded status (`TRUE`/`FALSE`) | No |

## Run

From the repository root:

```bash
cargo run --manifest-path pdf_downloader/Cargo.toml -- path/to/links.xlsx
```

Use `--reset` (or `-r`) to ignore downloaded statuses in the input workbook:

```bash
cargo run --manifest-path pdf_downloader/Cargo.toml -- path/to/links.xlsx --reset
```

## Output

Paths are relative to the directory where the command is run:

- PDFs: `downloads/<id>.pdf`
- Status report: `output/result.xlsx`

The report contains the ID, URLs, and final download status, sorted by ID.

## Notes

- Up to 20 records are downloaded concurrently.
- The alternate URL is tried if the primary URL fails.
- A response must contain `%PDF-` within its first 1,024 bytes to be saved.
- A row marked as downloaded is skipped; the program does not check whether
  its PDF file still exists.
- The PDF signature check is basic and does not fully validate the file.
- Input parsing expects the workbook columns and cell types to be valid.

## Build and test

Build a release executable:

```bash
cargo build --release --manifest-path pdf_downloader/Cargo.toml
```

Run the tests:

```bash
cargo test --manifest-path pdf_downloader/Cargo.toml
```

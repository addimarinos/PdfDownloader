# PdfDownloader

PdfDownloader is a command-line tool that reads document links from an Excel
workbook, downloads PDF files, and writes a workbook with the download status
for each record.

## Features

- Reads records from the first worksheet in an `.xlsx` file.
- Supports a primary URL and an optional alternate URL for each PDF.
- Downloads up to 20 records concurrently.
- Checks that the response contains a PDF signature (`%PDF-`) near the
  beginning of the content before saving it.
- Skips records already marked as downloaded in the input workbook.
- Writes results to an Excel report.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo.
- An Excel `.xlsx` workbook structured as described below.

## Input workbook

The program reads the first worksheet in the workbook (the worksheet name does
not need to be `Sheet1`). It skips the first row as the header and expects the
following columns:

| Column | Value | Required |
| --- | --- | --- |
| A | Integer ID; used as the downloaded filename | Yes |
| B | Primary URL | Yes |
| C | Alternate URL | No |
| D | Downloaded status (`TRUE`/`FALSE`) | No |

The values in the required columns should be valid and present. The parser
expects an integer ID and a URL string.

## Run

From the repository root:

```bash
cargo run --manifest-path pdf_downloader/Cargo.toml -- path/to/links.xlsx
```

To ignore the downloaded statuses in the workbook and attempt the downloads
again, pass `--reset` (or `-r`):

```bash
cargo run --manifest-path pdf_downloader/Cargo.toml -- path/to/links.xlsx --reset
```

The command-line help is available with:

```bash
cargo run --manifest-path pdf_downloader/Cargo.toml -- --help
```

## Output

Paths are relative to the directory where the command is run:

- Downloaded PDFs are saved in `downloads/<id>.pdf`.
- The status workbook is written to `output/blegh.xlsx`.

The report contains the ID, primary URL, alternate URL, and final downloaded
status. Records are sorted by ID in the report.

## Behavior and limitations

- `--reset` ignores statuses recorded in the input workbook. It does not remove
  existing files; a successful download for an ID writes to that ID's `.pdf`
  path.
- A record marked as already downloaded is skipped. The program does not
  verify that the corresponding PDF file still exists.
- The downloader checks for the `%PDF-` signature within the first 1,024 bytes;
  this is a basic content check, not full PDF validation.
- Network errors, unsuccessful HTTP responses, invalid PDF content, or file
  errors can cause a record to remain unsuccessful. Check the generated report
  for final statuses.
- Input parsing currently assumes the required sheet, columns, and cell types
  are correct.

## Development

The application is written in Rust (Edition 2024). It uses Tokio for async
execution, Reqwest for HTTP requests, Clap for CLI parsing, `excelstream` for
reading workbooks, and `rust_xlsxwriter` for report generation.

Run the test suite from the repository root with:

```bash
cargo test --manifest-path pdf_downloader/Cargo.toml
```

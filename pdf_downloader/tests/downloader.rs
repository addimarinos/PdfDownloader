use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use pdf_downloader::{
    downloader::download_file,
    structs::UrlRecord,
};

#[tokio::test]
pub async fn test_download_file() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/file.pdf"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type","application/pdf")
                .set_body_bytes(b"%PDF-test".to_vec()),
        )
        .mount(&server)
        .await;

    let mock_url = format!("{}/file.pdf", server.uri());

    let record = UrlRecord {
        id: 2,
        url: mock_url,
        alturl: None,
        status: false
    };
    let client = reqwest::Client::new();
    let output_dir = tempfile::tempdir().unwrap();
    
    let updated = download_file(record, output_dir.path(), &client)
        .await
        .expect("download_file should succeed");
    
    assert!(updated.status);

    let saved_bytes = tokio::fs::read(output_dir.path().join("2.pdf"))
        .await
        .expect("PDF should have been created");

    assert_eq!(saved_bytes, b"%PDF-test");
    server.verify().await;

    }
use reqwest::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use pdf_downloader::{
    downloader::download_file,
    structs::UrlRecord,
};

async fn my_test_helper_make_mock(mock_server: &MockServer, http_response: StatusCode) {
    Mock::given(method("GET"))
        .and(path("/file.pdf"))
        .respond_with(
            ResponseTemplate::new(http_response)
                .insert_header("content-type","application/pdf")
                .set_body_bytes(b"%PDF-test".to_vec()),
        )
        .mount(&mock_server)
        .await;
}

#[tokio::test]
pub async fn test_download_file_success_on_first_url() {
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

pub async fn test_download_file_success_on_alt_url() {
    let server = MockServer::start().await;

    my_test_helper_make_mock(&server, StatusCode::from_u16(200).unwrap()).await;

    let mock_url_1 = format!("{}", server.uri());
    let mock_url_2 = format!("{}/file.pdf", server.uri());

    let record = UrlRecord {
        id: 2,
        url: mock_url_1,
        alturl: Some(mock_url_2),
        status: false
    };

    let client = reqwest::Client::new();
    let output_dir = tempfile::tempdir().unwrap();

    let updated = download_file(record, output_dir.path(), &client).await;
    assert!(updated.status);

    let saved_bytes = tokio::fs::read(output_dir.path().join("2.pdf"))
        .await
        .expect("PDF from url 2 should have been created");


}
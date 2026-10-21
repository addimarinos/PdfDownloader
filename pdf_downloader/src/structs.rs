
pub struct UrlRecord {
    pub id: i64,
    pub url: String,
    pub alturl: Option<String>,
    pub status: bool
}

enum DownloadResult {
    Success,
    AltSuccess,
    NoResponse,
    NoFileBytes, 

}

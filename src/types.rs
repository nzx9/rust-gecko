#[derive(Debug)]
pub struct Response<T> {
    pub is_success: bool,
    pub status: reqwest::StatusCode,
    pub json: Option<T>,
    pub error: Option<reqwest::Error>,
}

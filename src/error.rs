#[derive(Debug)]
pub enum KyaniteError {
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    ReqwestUrlError(reqwest::UrlError),
    SerdeJSONError(serde_json::Error),
}

impl From<std::io::Error> for KyaniteError {
    fn from(err: std::io::Error) -> Self {
        KyaniteError::IOError(err)
    }
}

impl From<reqwest::Error> for KyaniteError {
    fn from(err: reqwest::Error) -> Self {
        KyaniteError::ReqwestError(err)
    }
}

impl From<reqwest::UrlError> for KyaniteError {
    fn from(err: reqwest::UrlError) -> Self {
        KyaniteError::ReqwestUrlError(err)
    }
}

impl From<serde_json::Error> for KyaniteError {
    fn from(err: serde_json::Error) -> Self {
        KyaniteError::SerdeJSONError(err)
    }
}

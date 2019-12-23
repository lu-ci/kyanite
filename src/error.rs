#[derive(Debug)]
pub enum KyaniteError {
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    ReqwestUrlError(reqwest::UrlError),
    SerdeJSONError(serde_json::Error),
    SetLoggerError(log::SetLoggerError),
    SerdeXMLError(serde_xml_rs::Error),
    DecompressError(flate2::DecompressError),
    SimpleString(String),
    FromUTF8Error(std::string::FromUtf8Error),
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

impl From<log::SetLoggerError> for KyaniteError {
    fn from(err: log::SetLoggerError) -> Self {
        KyaniteError::SetLoggerError(err)
    }
}

impl From<serde_xml_rs::Error> for KyaniteError {
    fn from(err: serde_xml_rs::Error) -> Self {
        KyaniteError::SerdeXMLError(err)
    }
}

impl From<flate2::DecompressError> for KyaniteError {
    fn from(err: flate2::DecompressError) -> Self {
        KyaniteError::DecompressError(err)
    }
}

impl From<String> for KyaniteError {
    fn from(err: String) -> Self {
        KyaniteError::SimpleString(err)
    }
}

impl From<std::string::FromUtf8Error> for KyaniteError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        KyaniteError::FromUTF8Error(err)
    }
}

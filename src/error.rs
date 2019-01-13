use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ReqwestError(reqwest::Error),
    UrlError(reqwest::UrlError),
    YAMLError(serde_yaml::Error),
    XMLError(serde_xml_rs::Error),
    JSONError(serde_json::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(err: reqwest::UrlError) -> Self {
        Error::UrlError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::YAMLError(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Error::XMLError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JSONError(err)
    }
}

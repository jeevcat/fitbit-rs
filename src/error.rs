use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to deserialize JSON")]
    Serde(#[from] serde_json::Error),
    #[error("HTTP error")]
    Reqwest(#[from] reqwest::Error),
    #[error("failed to read/write file")]
    Io(#[from] std::io::Error),
    #[error("error parsing URL")]
    Url(#[from] url::ParseError),
    #[error("unknown data store error")]
    Unknown,
}

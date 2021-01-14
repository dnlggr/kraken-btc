use thiserror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The API request failed with status code {status} ({status_text}).")]
    RequestFailed { status: u16, status_text: String },

    #[error("Kraken returned the following error: {kraken_error}.")]
    KrakenError { kraken_error: String },

    #[error("Failed to deserialize the API response.")]
    DeserializationError {
        #[from]
        source: serde_json::error::Error,
    },

    #[error(transparent)]
    IoError {
        #[from]
        source: std::io::Error,
    },
}

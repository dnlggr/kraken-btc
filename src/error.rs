use thiserror;

use super::networking::Error as NetworkingError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("API Error")]
    NetworkingError {
        #[from]
        source: NetworkingError,
    },

    #[error("Placing the order failed due to {details}.")]
    InvalidOrder { details: String },
}

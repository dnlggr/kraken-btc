use std::convert::From;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use super::error::{Error, Result};

#[derive(Deserialize, Debug)]
struct KrakenResponse {
    error: Vec<String>,
    result: Option<Value>,
}

pub struct Response {
    pub body: Value,
}

impl From<Value> for Response {
    fn from(body: Value) -> Self {
        Response { body }
    }
}

impl Response {
    pub fn deserialize<T: DeserializeOwned>(self) -> Result<T> {
        let response: KrakenResponse = serde_json::from_value(self.body)?;

        // Todo:
        // The `error` field can contains warnings and/or errors.
        // We don't differentiate between warnings and errors and fail on both.
        // This might lead to good responses (that have warnings) being treated as errors.
        if !response.error.is_empty() {
            return Err(Error::KrakenError {
                kraken_error: response.error.join(", "),
            });
        }

        let json = response
            .result
            .expect("field `result` missing in response despite no errors");

        match serde_json::from_value::<T>(json) {
            Ok(res) => Ok(res),
            Err(err) => Err(Error::from(err)),
        }
    }
}

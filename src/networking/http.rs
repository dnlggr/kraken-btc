use serde_json::Value;
use ureq;

use super::error::{Error, Result};

pub trait Http {
    fn post(&self, url: &str, headers: &[(&str, &str)], data: &[(&str, &str)]) -> Result<Value>;
}

pub fn new() -> Box<dyn Http> {
    Box::new(SyncHttp {})
}

struct SyncHttp;

impl Http for SyncHttp {
    fn post(&self, url: &str, headers: &[(&str, &str)], data: &[(&str, &str)]) -> Result<Value> {
        let mut request = ureq::post(url);

        for header in headers {
            request.set(header.0, header.1);
        }

        let response = request.send_form(data);

        if !response.ok() {
            return Err(Error::RequestFailed {
                status: response.status(),
                status_text: response.status_text().to_string(),
            });
        }

        match response.into_json() {
            Ok(value) => Ok(value),
            Err(err) => Err(Error::from(err)),
        }
    }
}

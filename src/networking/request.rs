use std::time::{SystemTime, UNIX_EPOCH};

use ring::{digest, hmac};

use super::error::Result;
use super::http::{self, Http};
use super::response::Response;
use crate::KeyPair;

pub struct Request<'a> {
    url: &'a str,
    key_pair: Option<&'a KeyPair<'a>>,
    data: Option<Vec<(&'a str, &'a str)>>,
    http: Box<dyn Http>,
}

impl<'a> Request<'a> {
    pub fn public(url: &'a str, data: Vec<(&'a str, &'a str)>) -> Request<'a> {
        Request {
            url: url,
            key_pair: None,
            data: Some(data),
            http: http::new(),
        }
    }

    pub fn private(
        url: &'a str,
        key_pair: &'a KeyPair<'a>,
        data: Option<Vec<(&'a str, &'a str)>>,
    ) -> Request<'a> {
        Request {
            url: url,
            data: data,
            key_pair: Some(key_pair),
            http: http::new(),
        }
    }

    pub fn send(&mut self) -> Result<Response> {
        let user_agent = ("User-Agent", "Kraken REST API");

        match self.key_pair {
            Some(key_pair) => {
                // Todo: Injecting nonce would improve testability.
                let nonce = self.nonce();
                let body = self.body(&nonce, self.data.as_ref());
                let signature = self.sign(&nonce, &body, key_pair);

                let headers = vec![
                    ("Api-Key", key_pair.public_key),
                    ("Api-Sign", &signature),
                    user_agent,
                ];

                Ok(self.http.post(self.url, &headers, &body)?.into())
            }
            None => {
                let headers = vec![user_agent];

                let json = match &self.data {
                    Some(data) => self.http.post(self.url, &headers, data.as_ref())?,
                    None => self.http.post(self.url, &headers, &vec![])?,
                };

                Ok(json.into())
            }
        }
    }

    fn nonce(&self) -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis()
            .to_string()
    }

    fn body(
        &self,
        nonce: &'a str,
        data: Option<&'a Vec<(&'a str, &'a str)>>,
    ) -> Vec<(&'a str, &'a str)> {
        match data {
            Some(data) => {
                // Todo: Modifying the original data instead of copying it would be more efficient.
                let mut body = data.to_vec();
                body.push(("nonce", nonce));
                body
            }
            None => vec![("nonce", nonce)],
        }
    }

    fn sign(&self, nonce: &str, body: &Vec<(&str, &str)>, key_pair: &KeyPair) -> String {
        let body_enc: String = body
            .iter()
            .map(|param| format!("{}={}", param.0, param.1))
            .collect::<Vec<String>>()
            .join("&");

        let mut digest_ctx = digest::Context::new(&digest::SHA256);
        digest_ctx.update(nonce.as_bytes());
        digest_ctx.update(body_enc.as_bytes());
        let digest = digest_ctx.finish();

        let path = self.url.trim_start_matches(crate::HOST);
        let key = hmac::Key::new(
            hmac::HMAC_SHA512,
            &base64::decode(key_pair.private_key).expect("api private key not base64 encoded"),
        );

        let mut tag_ctx = hmac::Context::with_key(&key);
        tag_ctx.update(path.as_bytes());
        tag_ctx.update(digest.as_ref());
        let tag = tag_ctx.sign();

        base64::encode(tag.as_ref())
    }
}

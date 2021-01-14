use std::collections::HashMap;

use crate::networking::{Request, Result};

#[derive(Debug, serde::Deserialize)]
pub struct AssetPair {
    pub altname: String,
    pub base: String,
    pub quote: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Ticker {
    #[serde(rename = "a")]
    pub ask: Vec<String>,
    #[serde(rename = "b")]
    pub bid: Vec<String>,
}

pub struct Api {
    path: String,
}

impl Api {
    pub fn new(path: String) -> Api {
        Api { path }
    }

    pub fn asset_pairs(&self, pairs: &str) -> Result<HashMap<String, AssetPair>> {
        let data = [("pair", pairs)].to_vec();

        Request::public(&format!("{}/AssetPairs", self.path), data)
            .send()?
            .deserialize()
    }

    pub fn ticker(&self, pairs: &str) -> Result<HashMap<String, Ticker>> {
        let data = [("pair", pairs)].to_vec();

        Request::public(&format!("{}/Ticker", self.path), data)
            .send()?
            .deserialize()
    }
}

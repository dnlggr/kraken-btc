use std::collections::HashMap;

use crate::networking::{Request, Result};

use crate::KeyPair;

#[derive(Debug, serde::Deserialize)]
pub struct TradeInfo {
    pub ordertxid: String,
    pub pair: String,
    pub time: f64,
    pub price: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderInfo {
    pub status: String,
    pub trades: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderDescription {
    pub order: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Order {
    pub descr: OrderDescription,
    pub txid: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderCancellation {
    pub count: i32,
}

pub struct Api<'a> {
    path: String,
    key_pair: &'a KeyPair<'a>,
}

impl<'a> Api<'a> {
    pub fn new(path: String, key_pair: &'a KeyPair<'a>) -> Api<'a> {
        Api { path, key_pair }
    }
}

impl<'a> Api<'a> {
    #[allow(dead_code)]
    pub fn balance(&self) -> Result<HashMap<String, String>> {
        Request::private(&format!("{}/Balance", self.path), &self.key_pair, None)
            .send()?
            .deserialize()
    }

    #[allow(dead_code)]
    pub fn query_orders(&self, txids: &str) -> Result<HashMap<String, OrderInfo>> {
        let data = [("txid", txids), ("trades", "true")].to_vec();

        Request::private(
            &format!("{}/QueryOrders", self.path),
            self.key_pair,
            Some(data),
        )
        .send()?
        .deserialize()
    }

    #[allow(dead_code)]
    pub fn query_trades(&self, txids: &str) -> Result<HashMap<String, TradeInfo>> {
        let data = [("txid", txids)].to_vec();

        Request::private(
            &format!("{}/QueryTrades", self.path),
            self.key_pair,
            Some(data),
        )
        .send()?
        .deserialize()
    }

    pub fn buy(&self, pair: &str, volume: &str, price: &str, dry_run: bool) -> Result<Order> {
        let mut data = [
            ("pair", pair),
            ("volume", volume),
            ("price", price),
            ("type", "buy"),
            ("ordertype", "limit"),
        ]
        .to_vec();

        if dry_run {
            data.push(("validate", "true"))
        }

        Request::private(
            &format!("{}/AddOrder", self.path),
            self.key_pair,
            Some(data),
        )
        .send()?
        .deserialize()
    }

    #[allow(dead_code)]
    pub fn cancel(&self, txid: &str) -> Result<OrderCancellation> {
        let data = [("txid", txid)].to_vec();

        Request::private(
            &format!("{}/CancelOrder", self.path),
            self.key_pair,
            Some(data),
        )
        .send()?
        .deserialize()
    }
}

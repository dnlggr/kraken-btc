mod api;
mod error;
mod networking;

use error::Error;

const BITCOIN: &str = "XBT";
const HOST: &str = "https://api.kraken.com";
const PRIVATE_PATH: &str = "0/private";
const PUBLIC_PATH: &str = "0/public";

pub type Result<T> = std::result::Result<T, Error>;

pub struct KeyPair<'a> {
    pub public_key: &'a str,
    pub private_key: &'a str,
}

impl<'a> KeyPair<'a> {
    pub fn new(public_key: &'a str, private_key: &'a str) -> KeyPair<'a> {
        KeyPair {
            public_key,
            private_key,
        }
    }
}

pub mod buy {
    use super::api::PrivateApi;
    use super::api::PublicApi;
    use super::error::Error;
    use super::KeyPair;
    use super::Result;

    pub struct Order<'a> {
        fiat: Option<&'a str>,
        amount: Option<i32>,
        key_pair: Option<&'a KeyPair<'a>>,
        dry_run: bool,
    }

    #[derive(Debug)]
    pub struct Trade {
        pub descr: String,
        pub txids: Option<String>,
    }

    impl<'a> Order<'a> {
        pub fn new() -> Order<'a> {
            Order {
                fiat: None,
                amount: None,
                key_pair: None,
                dry_run: false,
            }
        }

        pub fn fiat_currency(&'a mut self, fiat: &'a str) -> &'a mut Order {
            self.fiat = Some(fiat);
            self
        }

        pub fn fiat_amount(&'a mut self, amount: i32) -> &'a mut Order {
            self.amount = Some(amount);
            self
        }

        pub fn api_credentials(&'a mut self, key_pair: &'a KeyPair<'a>) -> &'a mut Order {
            self.key_pair = Some(key_pair);
            self
        }

        pub fn dry_run(&'a mut self, dry_run: bool) -> &'a mut Order {
            self.dry_run = dry_run;
            self
        }

        pub fn place(&self) -> Result<Trade> {
            let fiat = match self.fiat {
                Some(fiat) => fiat,
                None => return self.invalid_order("missing fiat currency"),
            };

            let fiat_amount = match self.amount {
                Some(amount) => amount,
                None => return self.invalid_order("missing fiat amount"),
            };

            let key_pair = match self.key_pair {
                Some(key_pair) => key_pair,
                None => return self.invalid_order("missing api credentials"),
            };

            let public_api = PublicApi::new(format!("{}/{}", super::HOST, super::PUBLIC_PATH));
            let private_api =
                PrivateApi::new(format!("{}/{}", super::HOST, super::PRIVATE_PATH), key_pair);

            // make sure asset pair exists
            let pairs = public_api.asset_pairs(&format!("{}{}", super::BITCOIN, fiat))?;
            let pair = pairs.values().next().expect("no pair despite ok api call");
            let pair = format!("{}{}", pair.base, pair.quote);

            // get current btc bid price
            let ticker = public_api.ticker(&pair)?;
            let ticker = ticker.get(&pair).expect("no ticker despite ok api call");
            let btc_bid_price = ticker.bid[0].parse::<f32>().expect("bid price not an f32");

            // we need to convert the fiat amount to a btc amount to submit the order
            let precision = 100000000_f32;
            let btc_volume = (fiat_amount as f32) / btc_bid_price;
            let btc_volume = (btc_volume * precision).round() / precision;

            let order = private_api.buy(
                &pair,
                &format!("{}", btc_volume),
                &format!("{}", btc_bid_price),
                self.dry_run,
            )?;

            let trade = Trade {
                descr: order.descr.order,
                txids: if let Some(txid) = order.txid {
                    Some(txid.join(", "))
                } else {
                    None
                },
            };

            Ok(trade)
        }

        fn invalid_order(&self, details: &str) -> Result<Trade> {
            Err(Error::InvalidOrder {
                details: details.to_string(),
            })
        }
    }
}

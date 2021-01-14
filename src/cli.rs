use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "pulpo",
    about = "An opinionated CLI for buying Bitcoin on Kraken.com"
)]
pub struct Cli {
    #[structopt(
        short = "k",
        long = "api-key",
        env = "KRAKEN_API_KEY",
        help = "The API key. Also known as API Public Key."
    )]
    pub api_key: String,

    #[structopt(
        short = "s",
        long = "api-secret",
        env = "KRAKEN_API_SECRET",
        help = "The API secret. Also known as API Private Key."
    )]
    pub api_secret: String,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    Buy(Buy),
}

#[derive(StructOpt)]
#[structopt(about = "Buy Bitcoin by selling a specified amount of fiat currency")]
pub struct Buy {
    #[structopt(
        name = "currency code",
        short = "c",
        long = "fiat-currency",
        help = "The ISO 4217 3-letter code of the fiat currency to sell. E.g. 'EUR' or 'USD'."
    )]
    pub curr: String,

    #[structopt(
        name = "fiat amount",
        short = "a",
        long = "fiat-amount",
        help = "The amount of fiat currency to sell."
    )]
    pub amt: i32,

    #[structopt(
        short = "d",
        long = "dry-run",
        help = "Does a full dry run but doesn't place an order."
    )]
    pub dry: bool,
}

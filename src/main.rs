mod cli;

use anyhow::{Error, Result};
use console::{pad_str, style, Alignment, Emoji};
use structopt::StructOpt;

use cli::{Buy, Cli, Command};
use kraken_btc::buy::Order;
use kraken_btc::KeyPair;

fn main() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.cmd {
        Command::Buy(cmd) => buy(&cli, &cmd),
    }
}
fn buy(cli: &Cli, cmd: &Buy) -> Result<()> {
    println!("🐙 Spending {} {} on Bitcoin.\n", cmd.amt, &cmd.curr);

    let key_pair = KeyPair::new(&cli.api_key, &cli.api_secret);

    let trade = Order::new()
        .api_credentials(&key_pair)
        .fiat_currency(&cmd.curr)
        .fiat_amount(cmd.amt)
        .dry_run(cmd.dry)
        .place()?;

    print_status("📦", "ORDER", &trade.descr);

    if let Some(txids) = &trade.txids {
        print_status("🎫", "TXID", txids);
        print_status("🎉", "STATUS", "Order placed.");
    } else if cmd.dry {
        print_status("👩‍🔬", "STATUS", "Order not placed - this was a dry run.");
    } else {
        return Err(Error::msg("Order not placed - something went wrong."));
    }

    Ok(())
}

fn print_status(emoji: &str, title: &str, status: &str) {
    println!(
        "{} {} {}",
        Emoji(emoji, "-"),
        style(pad_str(&format!("{}:", title), 8, Alignment::Left, None)).bold(),
        style(status)
    );
}

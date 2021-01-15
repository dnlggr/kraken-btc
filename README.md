<h1 align="center" style="font-weight: bold !important">kraken-btc 🐙</h1>

<p align="center">
  An <em>opinionated</em> command line interface for buying Bitcoin on <a href="https://kraken.com">Kraken.com</a>. Written in Rust.
</p>

<h3 align="center">
  <a href="#-installation-">Installation</a>
  <span> · </span>
  <a href="#-usage">Usage</a>
  <span> · </span>
  <a href="#-api-credentials">API Credentials</a>
  <span> · </span>
  <a href="#-is-this-safe">Is This Safe?</a>
</h3>

---

This is a simple command line interface for buying Bitcoin on [Kraken.com](https://kraken.com). The idea is to enable setups that make it easy to get your hands on some Bitcoin in an automated way. For example, you could use this in an Auto-[DCA](https://en.wikipedia.org/wiki/Dollar_cost_averaging) script.

I've always wanted to build something in [Rust 🦀](https://rust-lang.org) so this was a good opportunity for me to learn how to do that.

## 💽 Installation

Binaries for macOS are available through Homebrew and GitHub releases. For other platforms, it is recommended to build from source.

### Homebrew (macOS)

```
$ brew install dnlggr/tap/kraken-btc
```

<details>
  <summary><i>Expand for uninstall instructions</i></summary>
  <p><code>$ brew untap dnlggr/tap</code> (optional)</p>
  <p><code>$ brew uninstall kraken-btc</code></p>
</details>

### Download Binary Release (macOS)

You can find binaries for all releases on this repo's [releases page](https://github.com/dnlggr/kraken-btc/releases).

### From Source

To build from source, you must have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).

```
$ cargo install kraken-btc 
```

This will install a binary in your Rust installation root's `bin` directory. Typically, this is `$HOME/.cargo/bin`. 

## 💸 Usage

This is a short example on how to use `kraken-btc` to spend 100 Euro on Bitcoin.

```
$ kraken-btc --api-key <...> --api-secret <...> buy --fiat-currency EUR --fiat-amount 100
```

You'll get a confirmation of your order that looks something like the one below.

```
🐙 Spending 100 EUR on Bitcoin.

📦 ORDER:   buy 0.00531632 XBTEUR @ limit 18810.0
🎫 TXID:    ABCDEF-GHIJK-LMNOPQ
🎉 STATUS:  Order was placed.
```

Make sure to double check your [orders page](https://www.kraken.com/u/trade#tab=orders) on Kraken from time to time make sure everything works as expected.

**Tip:** Pass the optional `--dry-run` flag to the `buy` subcommand to test your order without actually placing it.

For more detailed usage information and documentation run `kraken-btc help`.

## 🔑 API Credentials

Create the necessary Kraken API credentials here: [API settings page](https://www.kraken.com/u/security/api/new).

To be able to place orders, the generated API credentials must have the **_Modify Orders_** permission. Other permissions are not needed.

On some pages, the _API Key_ might be referred to as _API Public Key_. Likewise, the _API Secret_ is also known as _API Private Key_. It is important to keep these credentials safe and never share them with anyone. It's best to not store them in plain text on your computer but to use a password manager. More information on how to create API credentials can be found in Kraken's [documentation](https://support.kraken.com/hc/en-us/articles/360000919966-How-to-generate-an-API-key-pair-).

## 🚓 Is This Safe?

The API credentials that you use with `kraken-btc` need the **_Modify Orders_** permission. No other permission is needed. The **_Modify Orders_** permission allows anyone who might learn about the credentials to place new orders on your behalf. However, according to [Kraken's documentation](https://support.kraken.com/hc/en-us/articles/360000919966-How-to-generate-an-API-key-pair-), it does not allow them to withdraw any funds from your account.

Of cource, your API credentials stay on your machine and `kraken-btc` does not store, transmit, or use them in any other way than to place orders on Kraken.

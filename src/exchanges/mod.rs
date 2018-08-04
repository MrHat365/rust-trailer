pub mod binance;
pub mod bittrex;
pub mod bitfinex;
pub mod kucoin;

use ::models::*;
use ::error::*;

pub trait ExchangeAPI {
    fn display(&self)               -> String;
    fn funds(&self)                 -> Result<Funds, TrailerError>;
    fn balances(&self)              -> Result<Vec<Asset>, TrailerError>;
    fn price(&self, symbol: &str)   -> Result<f64, TrailerError>;
    fn prices(&self)                -> Result<Prices, TrailerError>;
    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn open_orders(&self)           -> Result<Vec<Order>, TrailerError>;
    fn past_orders(&self)           -> Result<Vec<Order>, TrailerError>;
    fn past_trades_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError>;
    fn chart_data(&self, symbol: &str, interval: &str) -> Result<Vec<Candlestick>, TrailerError>;
    fn btc_price(&self)             -> Result<f64, TrailerError>;

    fn price_for_symbol(&self, symbol: &str) -> Result<f64, TrailerError> {
        Ok(self.funds()?.alts.iter().find(|c|c.symbol == symbol)
            .ok_or(TrailerError::generic(&format!("symbol not in funds: {}", symbol)))?.amount)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum Exchange {
    Unknown,
    Bittrex,
    Binance,
    Kucoin,
}

use std::str::FromStr;
impl FromStr for Exchange {
    type Err = ();

    fn from_str(s: &str) -> Result<Exchange, ()> {
        match s {
            "unknown"       => Ok(Exchange::Unknown),
            "-"             => Ok(Exchange::Unknown),
            "bittrex"       => Ok(Exchange::Bittrex),
            "binance"       => Ok(Exchange::Binance),
            "kucoin"        => Ok(Exchange::Binance),
            _               => Err(()),
        }
    }
}

use std::string::ToString;
impl ToString for Exchange {
    fn to_string(&self) -> String {
        match self {
            &Exchange::Bittrex => "bittrex".into(),
            &Exchange::Binance => "binance".into(),
            &Exchange::Kucoin => "kucoin".into(),
            _ => "-".into(),
        }
    }
}

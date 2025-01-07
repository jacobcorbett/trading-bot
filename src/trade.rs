use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::process::exit;
use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;

use crate::api;
use crate::portfolio_code::Portfolio;

#[derive(Debug)]
pub struct trade_position {
    pub uuid: Uuid,
    pub ticker: String,
    pub size: f32,
    pub open_price: f32,
    pub close_price: f32,
    pub inital_value: f32,
}
pub fn open_trade(mut portfolio: Portfolio, ticker: &str, amount_of_shares: f32) -> Portfolio {
    let uuid = Uuid::new_v4();
    let price_per_share = match api::finnhub_get_current_stock_price(ticker) {
        Ok(price) => price,
        Err(e) => {
            eprintln!(
                "failed to find price of stock while opening manual trade: {}",
                e
            );
            return portfolio;
        }
    };

    let total_value = price_per_share * amount_of_shares;

    if total_value > portfolio.cash_balance {
        println!("not enough money");
        return portfolio;
    }

    let temp = trade_position {
        uuid: uuid,
        ticker: ticker.to_string(),
        size: amount_of_shares,
        open_price: price_per_share,
        close_price: -1.0,
        inital_value: total_value,
    };

    portfolio.open_trades.push(temp);
    portfolio.cash_balance -= total_value;

    portfolio
}

pub fn close_trade(mut portfolio: Portfolio, trade_uuid: Uuid) -> Portfolio {
    let mut index_to_remove = 1000000;

    for i in 0..portfolio.open_trades.len() {
        if portfolio.open_trades[i].uuid == trade_uuid {
            index_to_remove = i;
        }
    }

    if index_to_remove == 1000000 {
        println!("No matching open trades");
        return portfolio;
    } else {
        portfolio.open_trades.remove(index_to_remove);
    }
    portfolio
}

use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::fs;
use std::process::exit;
use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;

use crate::api;
use crate::trade::trade_position;

#[derive(Debug)]
pub struct Portfolio {
    pub cash_balance: f32,
    pub assets: HashMap<String, f32>,
    pub open_trades: Vec<trade_position>,
}

pub fn update_cash_balance(mut portfolio: Portfolio, update_value: f32) -> Portfolio {
    let current_value = portfolio.cash_balance;
    let new_value = current_value + update_value;
    portfolio.cash_balance = new_value;
    return portfolio;
}

pub fn status_of_all_trades(portfolio: Portfolio) -> Portfolio {
    if portfolio.open_trades.len() == 0 {
        println!("Cash Balance: ${}", portfolio.cash_balance);
        println!("No Open Trades");
        return portfolio;
    }
    println!("Cash Balance: ${}\n", portfolio.cash_balance);
    for trade in &portfolio.open_trades {
        println!("UUID: {}", trade.uuid);
        println!("Ticker: {}", trade.ticker);
        println!("Amount of Shares: {}", trade.size);
        println!("Trade open price: ${}", trade.open_price);
        let current_stock_price = api::finnhub_get_current_stock_price(&trade.ticker).unwrap();
        println!("Current Price: ${:?}", current_stock_price);
        let profit_or_loss = current_stock_price - trade.open_price;
        println!("Profit/Loss: ${:?}", profit_or_loss);
        let percetange_differance =
            ((current_stock_price - trade.open_price) / trade.open_price) * 100.0;

        println!("Percentage Gain/Loss: {:?}%", percetange_differance);

        let total_value = current_stock_price * trade.size;
        println!("Total Value: ${:?}", total_value);
        println!(" ");
    }
    return portfolio;
}

pub fn save_state(portfolio: Portfolio) {
    /*
        SAVE FORMAT
        version: 1.0
        cash_balance: 100.0
        number_of_open_trades: 2
        open_trade: 1 {
        uuid: 123123-123123-123123-123123
        ticker: AAPL
        size: 1.0
        open_price: 235.45
        close_price: -1.0
        inital_value: 235.45
        }
        open_trade: 2 {
        uuid: 123123-123123-123123-123123
        ticker: AAPL
        size: 1.0
        open_price: 235.45
        close_price: -1.0
        inital_value: 235.45
        }




    */

    println!("SAVE STATE");
    let data = "version:1.0\ncash_balance:".to_owned()
        + &portfolio.cash_balance.to_string()
        + "\nnumber_of_open_trades:"
        + &portfolio.open_trades.len().to_string();

    // TODO loop through all open trades and concat to data

    fs::write("./save_states/test.txt", data).expect("Unable to write file");
}

// pub fn load_state() -> Portfolio {
// }

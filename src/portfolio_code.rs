use crate::api;
use crate::trade::trade_position;
use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;

#[derive(Debug)]
pub struct Portfolio {
    pub cash_balance: f32,
    pub assets: HashMap<String, f32>,
    pub open_trades: Vec<trade_position>,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
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

pub fn save_state(portfolio: Portfolio, file_name: &str) -> Portfolio {
    // Constructs the basic header
    let mut data = "version:1.0\ncash_balance:".to_owned()
        + &portfolio.cash_balance.to_string()
        + "\nnumber_of_open_trades:"
        + &portfolio.open_trades.len().to_string();

    let mut index = 0;
    for open_trade in &portfolio.open_trades {
        dbg!(open_trade);
        let temp_str = "\nopen_trade:".to_owned()
            + &(index).to_string()
            + "\nuuid:"
            + &open_trade.uuid.to_string()
            + "\nticker:"
            + &open_trade.ticker.to_string()
            + "\nsize:"
            + &open_trade.size.to_string()
            + "\nopen_price:"
            + &open_trade.open_price.to_string()
            + "\nclose_price:"
            + &open_trade.close_price.to_string()
            + "\ninital_value:"
            + &open_trade.inital_value.to_string();

        data = data + &temp_str;
        index += 1;
    }

    // TODO loop through all open trades and concat to data

    let path = "./save_states/".to_owned() + file_name + ".txt";

    fs::write(path, data).expect("Unable to write file");

    println!("Successfully saved State as {}.txt in /saves", file_name);

    portfolio
}

pub fn load_state_v1(mut portfolio: Portfolio, file_name: &str) -> Result<Portfolio, String> {
    let save_file_path = "./save_states/".to_owned() + file_name + ".txt";

    let save_file_lines = lines_from_file(save_file_path);

    if save_file_lines[0] == "version:1.0" {
        let new_cash_balance = save_file_lines[1].split(":");
        let temp_collection: Vec<&str> = new_cash_balance.collect();

        // attempt to parse the loaded cash balance from the file
        // If fail return
        match temp_collection[1].parse::<f32>() {
            Ok(loaded_cash_balance) => portfolio.cash_balance = loaded_cash_balance,
            Err(e) => {
                eprintln!("Failed to convert to f32: {}", e);
                return Err("Failed to convert to f32".to_string());
            }
        }

        let number_of_open_trades_line = save_file_lines[2].split(":");
        let temp_collection: Vec<&str> = number_of_open_trades_line.collect();
        let mut number_of_open_trades = 0;

        // reads in number of trades
        match temp_collection[1].parse::<i32>() {
            Ok(loaded_number_of_open_trades) => {
                number_of_open_trades = loaded_number_of_open_trades
            }
            Err(e) => {
                eprintln!("Failed convert number of trades to i32: {}", e);
                return Err("Failed convert number of trades to i32".to_string());
            }
        };

        for i in 0..number_of_open_trades {
            println!("{}", i);

            let mut temp_trade_position = trade_position {
                uuid: Uuid::new_v4(),
                ticker: "TEMP".to_string(),
                size: 0.0,
                open_price: 0.0,
                close_price: 0.0,
                inital_value: 0.0,
            };

            let starting_index = 4;

            // Working on UUID
            let working_line_index = starting_index + ((i as usize) * 7);
            let uuid_line = save_file_lines[working_line_index].split(":");
            let temp_collection: Vec<&str> = uuid_line.collect();

            match temp_collection[1].parse::<Uuid>() {
                Ok(loaded_uuid) => temp_trade_position.uuid = loaded_uuid,
                Err(e) => {
                    eprintln!("Failed to parse UUID: {}", e);
                    return Err("Failed to parse UUID".to_string());
                }
            };

            // Working on ticker

            let working_line_index = starting_index + 1 + ((i as usize) * 7);
            let ticker_line = save_file_lines[working_line_index].split(":");
            let temp_collection: Vec<&str> = ticker_line.collect();

            match temp_collection[1].parse::<String>() {
                Ok(loaded_ticker) => temp_trade_position.ticker = loaded_ticker,
                Err(e) => {
                    eprintln!("Failed to parse ticker into String: {}", e);
                    return Err("Failed to parse ticker into String".to_string());
                }
            };

            // Working on postion size
            let working_line_index = starting_index + 2 + ((i as usize) * 7);
            let size_line = save_file_lines[working_line_index].split(":");
            let temp_collection: Vec<&str> = size_line.collect();

            match temp_collection[1].parse::<f32>() {
                Ok(loaded_size) => temp_trade_position.size = loaded_size,
                Err(e) => {
                    eprintln!("Failed to convert to f32: {}", e);
                    return Err("Failed to convert to f32".to_string());
                }
            };

            // Working on open price
            let working_line_index = starting_index + 3 + ((i as usize) * 7);
            let open_price_line = save_file_lines[working_line_index].split(":");
            let temp_collection: Vec<&str> = open_price_line.collect();

            match temp_collection[1].parse::<f32>() {
                Ok(loaded_open_price) => temp_trade_position.open_price = loaded_open_price,
                Err(e) => {
                    eprintln!("Failed to convert to f32: {}", e);
                    return Err("Failed to convert to f32".to_string());
                }
            };

            // Working on close price
            let working_line_index = starting_index + 4 + ((i as usize) * 7);
            let close_price_line = save_file_lines[working_line_index].split(":");
            let temp_collection: Vec<&str> = close_price_line.collect();

            match temp_collection[1].parse::<f32>() {
                Ok(loaded_close_price) => temp_trade_position.close_price = loaded_close_price,
                Err(e) => {
                    eprintln!("Failed to convert to f32: {}", e);
                    return Err("Failed to convert to f32".to_string());
                }
            };

            // Working on inital_value price
            let working_line_index = starting_index + 5 + ((i as usize) * 7);
            let inital_value_line = save_file_lines[working_line_index].split(":");
            let temp_collection: Vec<&str> = inital_value_line.collect();

            match temp_collection[1].parse::<f32>() {
                Ok(loaded_inital_value) => temp_trade_position.inital_value = loaded_inital_value,
                Err(e) => {
                    eprintln!("Failed to convert to f32: {}", e);
                    return Err("Failed to convert to f32".to_string());
                }
            };

            portfolio.open_trades.push(temp_trade_position);
        }

        Ok(portfolio)
    } else {
        Err("not version 1.0".to_string())
    }
}

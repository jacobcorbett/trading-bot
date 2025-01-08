use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::process::exit;
use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;

use crate::algorithms;
use crate::api;
use crate::portfolio_code;
use crate::portfolio_code::Portfolio;
use crate::trade;

pub fn status_of_all_trades_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!(" ");
    portfolio = portfolio_code::status_of_all_trades(portfolio);
    println!(" ");
    portfolio
}

pub fn algorithm_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!("\nCommands:\np: Percentage Change Trigger Algorithm\na: Moving Average Crossover Algorithm");

    let mut line = String::new();
    println!("\nEnter command :");
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let command = line.trim();

    if command == "p" {
        portfolio = algorithms::percentage_change_trigger_algo(portfolio);
    } else if command == "a" {
        portfolio = algorithms::moving_average_crossover_algo(portfolio);
    }

    portfolio
}

pub fn closeing_trade_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!("closing trade");
    //
    println!("Enter uuid of trade :");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let trade_uuid = line.trim().parse::<Uuid>();

    // validate uuid
    match &trade_uuid {
        Ok(uuid) => println!("Valid UUID: {}", uuid),
        Err(e) => {
            println!("Invalid UUID: {}", e);
            return portfolio;
        }
    }

    portfolio = trade::close_trade(portfolio, trade_uuid.expect("REASON"));

    portfolio
}

pub fn add_cash_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!("Enter $ amount to change cash balance by: ");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let change_value = line.trim().parse::<f32>();

    portfolio = portfolio_code::update_cash_balance(portfolio, change_value.expect("REASON"));
    portfolio
}
pub fn open_trade_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!(" ");
    println!("OPENING TRADE");
    //
    println!("Enter Ticker :");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let ticker = line.trim();

    //validate ticker

    match api::check_vaild_ticker(ticker) {
        Ok(true) => {
            //println!("ticker is valid")
        }
        Ok(false) => {
            println!("invalid ticker");
            return portfolio;
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            return portfolio;
        }
    }

    //
    println!("Enter Number of shares :");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let number_of_shares = line.trim().parse::<f32>();
    //
    portfolio = trade::open_trade(portfolio, ticker, number_of_shares.expect("REASON"));
    portfolio
    //
}

pub fn save_state_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!("STARTING SAVE STATE");

    portfolio = portfolio_code::save_state(portfolio);

    portfolio
}

pub fn load_state_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!("LOADING STATE");

    let blank_portfolio = Portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
        open_trades: Vec::new(),
    };

    match portfolio_code::load_state_v1(portfolio) {
        Ok(loaded_portfolio) => loaded_portfolio,
        Err(e) => {
            eprintln!("Failed to load State: {}", e);

            blank_portfolio
        }
    }
}

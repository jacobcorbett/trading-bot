use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::process::exit;
use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;

mod algorithms;
mod api;
mod menu_functions;
mod portfolio_code;
mod trade;
use crate::portfolio_code::Portfolio;

fn main() {
    let mut main_portfolio = Portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
        open_trades: Vec::new(),
    };

    // let x = api::get_last_100_days_price_data("AAPL");
    // dbg!(x);
    // exit(0);
    //
    //
    //

    // TODO FINISH ABOVE FUNCTION TO TAKE IN TICKERS ^^

    loop {
        let market_status = match api::is_market_open() {
            Ok(true) => "open".to_string(),
            Ok(false) => "closed".to_string(),
            Err(e) => {
                println!("An error occurred: {}", e);
                "An error occurred in fetching status of market".to_string()
            }
        };

        println!("\nMarket Status: {}", market_status);
        println!("Commands:\ns: status of all trades\no: open new single trade\nc: close single trade\na: algorithm mode\nm: add cash\nS: save state\nR: load state\nq: quit\n");
        let mut line = String::new();
        println!("Enter command :");

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let command = line.trim();

        if command == "s" {
            main_portfolio = menu_functions::status_of_all_trades_menu_function(main_portfolio);
        } else if command == "o" {
            main_portfolio = menu_functions::open_trade_menu_function(main_portfolio);
        } else if command == "c" {
            main_portfolio = menu_functions::closeing_trade_menu_function(main_portfolio);
        } else if command == "a" {
            main_portfolio = menu_functions::algorithm_menu_function(main_portfolio);
        } else if command == "m" {
            main_portfolio = menu_functions::add_cash_menu_function(main_portfolio);
        } else if command == "R" {
            main_portfolio = menu_functions::load_state_menu_function(main_portfolio);
        } else if command == "S" {
            main_portfolio = menu_functions::save_state_menu_function(main_portfolio);
        } else if command == "q" {
            println!("Exiting..");
            break;
        }
    }
}

// todo, validate stock inputs
// clear screen
//

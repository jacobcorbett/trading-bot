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
    // main_portfolio.cash_balance = 200.0;
    // portfolio_code::save_state(main_portfolio);
    // exit(9);

    // TODO FINISH ABOVE FUNCTION TO TAKE IN TICKERS ^^

    loop {
        let open_or_closed = api::is_market_open();
        let mut market_status = "closed";

        match open_or_closed {
            Ok(is_open) => {
                if is_open == true {
                    market_status = "open";
                } else {
                    market_status = "closed";
                }
            }
            Err(e) => {
                println!("An error occurred: {}", e);
            }
        }

        println!("\nMarket Status: {}", market_status);
        println!("Commands:\ns: status of all trades\no: open new single trade\nc: close single trade\na: algorithm mode\nm: add cash\nq: quit\n");
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
        } else if command == "r" {
            // load in from text file
        } else if command == "s" {
            // save sstate to text file
        } else if command == "q" {
            println!("Exiting..");
            break;
        }
    }
}

// todo, validate stock inputs
// clear screen
//

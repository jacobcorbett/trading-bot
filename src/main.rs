use dotenv::dotenv;
use flexi_logger::{FileSpec, Logger, WriteMode};
use log;

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
    // Initialize the logger
    Logger::try_with_str("trace") // Set log level to "info"
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs") // Set directory to "logs"
                .suffix("log"), // Optional: add .log suffix to files
        )
        .format(flexi_logger::detailed_format)
        .write_mode(WriteMode::BufferAndFlush) // Buffer logs and flush periodically
        .start()
        .unwrap();

    // log::error!("Critical error occurred: {}", "database connection failed");
    // log::warn!("Warning: approaching memory limit at {}%", 90);
    // log::info!("Server started successfully on port {}", 8080);
    // log::debug!("Processing request with id: {}", "12345");
    // log::trace!("Entering function with args: {:?}", vec![1, 2, 3]);

    log::info!("Program started");

    let mut main_portfolio = portfolio_code::blank_portfolio();
    log::info!("Created main_portfolio: {:?}", main_portfolio);

    // let x = api::get_20_years_old_historial_data("AAPL");
    // dbg!(x);
    // exit(0);
    //
    //
    //

    // TODO FINISH ABOVE FUNCTION TO TAKE IN TICKERS ^^
    log::info!("Starting main loop");
    loop {
        let market_status = match api::is_market_open() {
            Ok(true) => {
                log::info!("Succsefully got market status and its: {}", true);
                "open".to_string()
            }
            Ok(false) => {
                log::info!("Succsefully got market status and its: {}", false);
                "closed".to_string()
            }
            Err(e) => {
                log::error!(
                    "An error occurred when trying to fetch market status: {}",
                    e
                );
                println!("An error occurred: {}", e);
                "An error occurred in fetching status of market".to_string()
            }
        };

        println!("\nMarket Status: {}", market_status);
        println!("Commands:\ns: status of all trades\no: open new single trade\nc: close single trade\nd: download stock data\na: algorithm mode\nm: add cash\nS: save state\nR: load state\nq: quit\n");
        let mut line = String::new();
        println!("Enter command :");

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let command = line.trim();
        log::info!("Reading user input: \'{}\'", command);

        if command == "s" {
            main_portfolio = menu_functions::status_of_all_trades_menu_function(main_portfolio);
        } else if command == "o" {
            main_portfolio = menu_functions::open_trade_menu_function(main_portfolio);
        } else if command == "c" {
            main_portfolio = menu_functions::closeing_trade_menu_function(main_portfolio);
        } else if command == "a" {
            main_portfolio = menu_functions::algorithm_menu_function(main_portfolio);
        } else if command == "d" {
            menu_functions::download_stock_data_menu_function();
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

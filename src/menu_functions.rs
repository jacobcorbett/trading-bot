use crate::algorithms;
use crate::api;
use crate::portfolio_code;
use crate::portfolio_code::Portfolio;
use crate::trade;
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

pub fn status_of_all_trades_menu_function(mut portfolio: Portfolio) -> Portfolio {
    log::info!("User entered status of app trades menu function");
    println!(" ");
    portfolio = portfolio_code::status_of_all_trades(portfolio);
    println!(" ");
    portfolio
}

pub fn algorithm_menu_function(mut portfolio: Portfolio) -> Portfolio {
    log::info!("User entered algorithm menu function");
    println!("\nCommands:\np: Percentage Change Trigger Algorithm\na: Moving Average Crossover Algorithm");

    let mut line = String::new();
    println!("\nEnter command :");
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let command = line.trim();
    log::info!("User entered: {}", command);

    if command == "p" {
        portfolio = algorithms::percentage_change_trigger_algo(portfolio);
    } else if command == "a" {
        let mut line = String::new();
        println!("\nEnter save file name:");
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let command = line.trim();
        log::info!("User entered: {}", command);

        portfolio = algorithms::moving_average_crossover_algo(portfolio, command);
    }

    portfolio
}

pub fn closeing_trade_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!("closing trade");
    log::info!("User entered closing trade menu function");

    if portfolio.open_trades.len() == 0 {
        log::info!("User has not trades to open, going back to main menu");
        return portfolio;
    }

    //
    println!("Enter uuid of trade :");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let trade_uuid = match line.trim().parse::<Uuid>() {
        Ok(number) => {
            log::info!("Successfully converted \'{}\' into Uuid", line);
            number
        }
        Err(e) => {
            println!("Invalid UUID");
            log::error!(
                "Failed to parse uuid of trade to close into Uuid, user entered \'{}\': {}",
                line,
                e
            );
            return portfolio;
        }
    };

    portfolio = trade::close_trade(portfolio, trade_uuid);

    portfolio
}

pub fn add_cash_menu_function(mut portfolio: Portfolio) -> Portfolio {
    log::info!("User entered add cash menu function");
    println!("Enter $ amount to change cash balance by: ");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let change_value = match line.trim().parse::<f32>() {
        Ok(number) => {
            log::info!("Successfully converted \'{}\' into f32", line);
            number
        }
        Err(e) => {
            log::error!(
                "Failed to parse money amount into f32, user entered \'{}\': {}",
                line,
                e
            );
            return portfolio;
        }
    };

    log::info!(
        "Updating cash balance: old value: {} ,diffrence: {}, new value: {}",
        portfolio.cash_balance,
        change_value,
        portfolio.cash_balance + change_value
    );

    portfolio = portfolio_code::update_cash_balance(portfolio, change_value);
    portfolio
}
pub fn open_trade_menu_function(mut portfolio: Portfolio) -> Portfolio {
    println!(" ");
    println!("OPENING TRADE");
    log::info!("User attempting to open a trade");
    //
    println!("Enter Ticker :");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let ticker = line.trim();
    log::info!("User entered for ticker: \'{}\'", ticker);

    //validate ticker
    log::info!("Checking if ticker is vaild...");
    match api::check_vaild_ticker(ticker) {
        Ok(true) => {
            //println!("ticker is valid")
            log::info!("Ticker \'{}\' is vaild", ticker);
        }
        Ok(false) => {
            println!("invalid ticker");
            log::info!("Ticker \'{}\' is not vaild", ticker);
            return portfolio;
        }
        Err(e) => {
            log::error!(
                "An error occurred when trying to see if ticker: \'{}\' is vaild: {}",
                ticker,
                e
            );
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

    let number_of_shares = match line.trim().parse::<f32>() {
        Ok(number) => {
            log::info!("Successfully converted \'{}\' into f32", line);
            number
        }
        Err(e) => {
            log::error!(
                "Failed to parse number of shares into f32, user entered \'{}\': {}",
                line,
                e
            );
            return portfolio;
        }
    };

    //
    portfolio = trade::open_trade(portfolio, ticker, number_of_shares);
    portfolio
    //
}

pub fn save_state_menu_function(mut portfolio: Portfolio) -> Portfolio {
    log::info!("User started save state menu function");
    println!("STARTING SAVE STATE");

    println!("Enter File name of Save:");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let file_name = line.trim();
    log::info!("User entered file name: {}", file_name);

    portfolio = portfolio_code::save_state(portfolio, file_name);

    portfolio
}

pub fn load_state_menu_function(mut portfolio: Portfolio) -> Portfolio {
    log::info!("User entered loading state menu function");
    println!("LOADING STATE");

    println!("Enter File name of Save:");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let file_name = line.trim();
    log::info!("User entered file to load state from: {}", file_name);

    let blank_portfolio = portfolio_code::blank_portfolio();

    match portfolio_code::load_state_v1(portfolio, file_name) {
        Ok(loaded_portfolio) => {
            log::info!("Successfully loaded portfolio: {:?}", loaded_portfolio);
            loaded_portfolio
        }
        Err(e) => {
            log::error!("Failed to load portfolio: {}", e);
            eprintln!("Failed to load State: {}", e);
            blank_portfolio
        }
    }
}

pub fn download_stock_data_menu_function() {
    println!(" ");
    println!("DOWNLOAD DATA OF STOCK");
    log::info!("User attempting to download data of a stock");

    println!("Enter Ticker :");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let ticker = line.trim();
    log::info!("User entered for ticker: \'{}\'", ticker);

    log::info!("Checking if ticker is vaild...");

    match api::check_vaild_ticker(ticker) {
        Ok(true) => {
            //println!("ticker is valid")
            log::info!("Ticker \'{}\' is vaild", ticker);
        }
        Ok(false) => {
            println!("invalid ticker");
            log::info!("Ticker \'{}\' is not vaild", ticker);
            return;
        }
        Err(e) => {
            log::error!(
                "An error occurred when trying to see if ticker: \'{}\' is vaild: {}",
                ticker,
                e
            );
            println!("An error occurred: {}", e);
            return;
        }
    }

    println!("Downloading stock data...");
    log::info!("Downloading stock data, ticker: {}", ticker);

    let stock_data = match api::get_20_years_old_historial_data(ticker) {
        Ok(stock_data) => {
            println!("Successfully got stock data");
            log::info!("Successfully got stock data, ticker: {}", ticker);
            stock_data
        }
        Err(e) => {
            log::error!("An error occurred: {}", e);
            println!("An error occurred: {}", e);
            Vec::new()
        }
    };

    //save data to file
    log::info!("Attemping to write data to file, ticker: {}", ticker);
    let path = "./stock_data/".to_owned() + ticker + ".txt";
    let data_to_write = stock_data.join("\n");
    fs::write(path, data_to_write).expect("Unable to write file");
    log::info!("Successfully written data to file, ticker: {}", ticker);
}

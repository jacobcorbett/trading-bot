use crate::api;
use crate::trade::trade_position;
use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use std::time::Duration;
use std::{collections::HashMap, env};
use std::{thread, time};
use tempfile::tempdir;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Portfolio {
    pub cash_balance: f32,
    pub assets: HashMap<String, f32>,
    pub open_trades: Vec<trade_position>,
}

pub fn blank_portfolio() -> Portfolio {
    return Portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
        open_trades: Vec::new(),
    };
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // https://www.thorsten-hans.com/weekly-rust-trivia-get-all-files-in-a-directory/
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
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
        log::info!("User had no open trades");
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
    log::info!(
        "Succesfully printed all users opened trades: {} number of trades",
        portfolio.open_trades.len()
    );
    return portfolio;
}

pub fn save_state(portfolio: Portfolio, file_name: &str) -> Portfolio {
    // Constructs the basic header
    log::info!("Starting Save State function");

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

    let path = "./save_states/".to_owned() + file_name + ".txt";

    fs::write(path, data).expect("Unable to write file");

    log::info!("Successfully saved State as {}.txt in /saves", file_name);
    println!("Successfully saved State as {}.txt in /saves", file_name);

    portfolio
}

pub fn load_state_v1(mut portfolio: Portfolio, file_name: &str) -> Result<Portfolio, String> {
    let save_file_path = "./save_states/".to_owned() + file_name + ".txt";

    // validate that the file exists

    let save_files_names = match get_files_in_directory("./save_states/") {
        Ok(save_files_names) => save_files_names,
        Err(e) => {
            eprintln!("failed");
            log::error!("Failed to get files in /saves dir: {}", e);
            return Err("Failed to get files in dir".to_string());
        }
    };

    let mut file_exisits = false;

    for file in save_files_names {
        if file_name.to_owned() + ".txt" == file {
            file_exisits = true;
        }
    }

    if file_exisits == false {
        log::error!("File does not exist in /saves dir");
        return Err("File does not exist".to_string());
    }

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

#[test]
fn creating_blank_portfolio() {
    let result = blank_portfolio();

    assert_eq!(
        result,
        Portfolio {
            cash_balance: 0.0,
            assets: HashMap::new(),
            open_trades: Vec::new(),
        }
    );
}

#[test]
fn reading_lines_from_file() {
    // TODO create own file and then delete it
    let file_dir = "./testing_files/test_read_lines.txt";

    let file_data_vector = lines_from_file(file_dir);
    assert_eq!("I love pizza", file_data_vector[0]);
    assert_eq!("cats and dogs", file_data_vector[1]);
}

#[test]
fn getting_all_files_in_directory() {
    let dir = "./testing_files/";

    let expected_files = vec!["test_read_lines.txt".to_string()];

    match get_files_in_directory(dir) {
        Ok(temp) => {
            for expected in &expected_files {
                assert!(
                    temp.contains(expected),
                    "The directory does not contain the expected file: '{}'.",
                    expected
                );
            }
        }
        Err(err) => panic!("Failed to get files in directory: {}", err),
    }
}

#[test]
fn test_updating_cash_balance() {
    let mut portfolio = blank_portfolio();
    assert_eq!(
        portfolio.cash_balance, 0.0,
        "Initial cash balance should be 0.0"
    );

    portfolio = update_cash_balance(portfolio, 100.0);
    assert_eq!(
        portfolio.cash_balance, 100.0,
        "Cash balance should update to 100.0"
    );

    portfolio = update_cash_balance(portfolio, -200.0);
    assert_eq!(
        portfolio.cash_balance, -100.0,
        "Cash balance should update to -100.0"
    );

    portfolio = update_cash_balance(portfolio, 50.0);
    assert_eq!(
        portfolio.cash_balance, -50.0,
        "Cash balance should update to -50.0"
    );

    portfolio = update_cash_balance(portfolio, 150.0);
    assert_eq!(
        portfolio.cash_balance, 100.0,
        "Cash balance should update to 100.0"
    );

    portfolio = update_cash_balance(portfolio, -100.0);
    assert_eq!(
        portfolio.cash_balance, 0.0,
        "Cash balance should update back to 0.0"
    );

    portfolio = update_cash_balance(portfolio, 500.0);
    assert_eq!(
        portfolio.cash_balance, 500.0,
        "Cash balance should update to 500.0"
    );

    portfolio = update_cash_balance(portfolio, -750.0);
    assert_eq!(
        portfolio.cash_balance, -250.0,
        "Cash balance should update to -250.0"
    );
}

#[test]
fn test_save_state() {
    // Set the directory path where we want to save the file
    let save_dir = "./save_states/";

    // Create the portfolio and trade_position
    let trade = trade_position {
        uuid: Uuid::new_v4(),
        ticker: "AAPL".to_string(),
        size: 10.0,
        open_price: 150.0,
        close_price: 155.0,
        inital_value: 1500.0,
    };

    let mut portfolio = blank_portfolio();
    portfolio.open_trades.push(trade);
    portfolio.cash_balance = 1000.0;

    // Mock the file saving by calling the function
    let portfolio = save_state(portfolio, "test_save");

    // Now, check if the file exists in the temp directory
    let file_path = save_dir.to_owned() + "test_save.txt";
    let file_content = fs::read_to_string(file_path.clone()).expect("Failed to read file");

    // Assert the file content is correct
    assert!(file_content.contains("version:1.0"));
    assert!(file_content.contains("cash_balance:1000"));
    assert!(file_content.contains("number_of_open_trades:1"));
    assert!(file_content.contains("open_trade:0"));
    assert!(file_content.contains("uuid:"));
    assert!(file_content.contains("ticker:AAPL"));
    assert!(file_content.contains("size:10"));
    assert!(file_content.contains("open_price:150"));
    assert!(file_content.contains("close_price:155"));
    assert!(file_content.contains("inital_value:1500"));

    fs::remove_file(file_path.clone()).expect("Failed to delete file");
}

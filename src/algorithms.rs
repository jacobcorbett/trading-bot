use chrono::Local;
use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::fs;
use std::process::exit;
use std::thread;
use std::time;
use std::time::Duration;
use std::{collections::HashMap, env};
use uuid::Uuid;

use crate::api;
use crate::portfolio_code;
use crate::portfolio_code::Portfolio;
use crate::trade;

#[derive(Debug)]
struct moving_average {
    date: String,
    average: f32,
}

#[derive(Debug)]
struct moving_average_stock {
    ticker: String,
    ten_day_moving_averages: Vec<moving_average>,
    fifty_day_moving_averages: Vec<moving_average>,
}

pub fn percentage_change_trigger_algo(mut portfolio: Portfolio) -> Portfolio {
    log::info!("User entered Percentage Change trigger algorithm");
    /*
        This algorithm, checks a list of stocks, if stock has gone up 2% since inital price
        Then buy 1 share, then if the trade goes up 10% close the trade.
    */

    let tickers_to_watch: Vec<&str> = vec![
        "PLTR", // Palantir Technologies
        "TSLA", // Tesla Inc.
        "NVDA", // NVIDIA Corporation
        "AAPL", // Apple Inc.
        "AMZN", // Amazon.com Inc.
        "GOOG", // Alphabet Inc.
        "MSFT", // Microsoft Corporation
        "META", // Meta Platforms Inc.
        "NFLX", // Netflix Inc.
        "AMD",  // Advanced Micro Devices Inc.
        "INTC", // Intel Corporation
        "ORCL", // Oracle Corporation
        "CRM",  // Salesforce Inc.
        "ADBE", // Adobe Inc.
        "UBER", // Uber Technologies Inc.
        "TWTR", // Twitter Inc. (now X Corp.)
        "SQ",   // Block Inc.
        "PYPL", // PayPal Holdings Inc.
        "SHOP", // Shopify Inc.
        "ZM",   // Zoom Video Communications
        "BABA", // Alibaba Group
        "V",    // Visa Inc.
        "MA",   // Mastercard Inc.
        "DIS",  // The Walt Disney Company
        "NIO",  // NIO Inc.
        "RBLX", // Roblox Corporation
        "LCID", // Lucid Group
        "F",    // Ford Motor Company
        "GM",   // General Motors Company
        "FGL",  // Example placeholder
    ];

    // let tickers_to_watch: Vec<&str> = vec![
    //     "PLTR", // Palantir Technologies
    //     "TSLA", // Tesla Inc.
    //     "NVDA", // NVIDIA Corporation
    //     "FGL", "LCID",
    // ];
    portfolio.cash_balance = 1000.0;

    println!("!ALGO MODE (Percentage Change Trigger)!");
    println!("Starting with ${}", portfolio.cash_balance);
    println!("tickers watching: {:?}", tickers_to_watch);

    let mut inital_price_for_ticker: HashMap<&str, f32> = HashMap::new();

    for ticker in tickers_to_watch {
        let price_per_share = match api::finnhub_get_current_stock_price(ticker) {
            Ok(price) => price,
            Err(e) => {
                eprintln!("Error fetching inital stock price for algo: {}", e);
                return portfolio;
            }
        };

        inital_price_for_ticker.insert(ticker, price_per_share);
    }

    loop {
        for mut stock in &inital_price_for_ticker {
            let current_stock_price = match api::finnhub_get_current_stock_price(stock.0) {
                Ok(price) => price,
                Err(e) => {
                    eprintln!("Error fetching stock price: {}", e);
                    thread::sleep(time::Duration::from_secs(60));
                    continue;
                }
            };

            //compare current price to inital price to see % difference
            // formula, (current_price - inital_price) /100 (allows for negatives)

            let percetange_differance = ((current_stock_price - stock.1) / stock.1) * 100.0;
            println!(
                "stock: {}\ninital price: ${}\ncurrent price: ${}\ndifference: {}%",
                stock.0, stock.1, current_stock_price, percetange_differance
            );

            if percetange_differance > 2.0 {
                // first check if trade already open

                let mut already_open: bool = false;
                for open_trade in &portfolio.open_trades {
                    if stock.0.to_string() == open_trade.ticker {
                        already_open = true;
                    }
                }

                // if trade not open with ticker x, open trade
                if already_open == false {
                    let number_of_shares: f32 = (portfolio.cash_balance * 0.1) / stock.1;
                    portfolio = trade::open_trade(portfolio, stock.0, number_of_shares);
                }
            }
            if percetange_differance < -2.0 {
                // first check if trade already open
                let mut already_open: bool = false;
                for open_trade in &portfolio.open_trades {
                    if stock.0.to_string() == open_trade.ticker {
                        already_open = true;
                    }
                }
                // if trade not open with ticker x, open trade
                if already_open == false {
                    let number_of_shares: f32 = (portfolio.cash_balance * -0.1) / stock.1;
                    portfolio = trade::open_trade(portfolio, stock.0, number_of_shares);
                }
            }

            let mut trades_to_close: Vec<Uuid> = Vec::new();

            // loops over every open trade
            // if the current symbol we grabbing price from is in a open trade
            // check the status of the trade eg up 10%
            // if up 10% we want to close the trade
            for open_trade in &portfolio.open_trades {
                if stock.0.to_string() == open_trade.ticker {
                    let trade_percentage_gain_or_loss =
                        (current_stock_price - open_trade.inital_value) / 100.0;

                    if trade_percentage_gain_or_loss > 10.0 {
                        trades_to_close.push(open_trade.uuid)
                    }
                }
            }

            // closes trades found ealier
            for uuid in trades_to_close {
                portfolio = trade::close_trade(portfolio, uuid)
            }
        }

        println!(" ");
        portfolio = portfolio_code::status_of_all_trades(portfolio);
        println!(" ");

        thread::sleep(time::Duration::from_secs(10));
    }
}

pub fn moving_average_crossover_algo(mut portfolio: Portfolio) -> Portfolio {
    log::info!("User entered Moving average crossover algorithm");
    let tickers_to_watch: Vec<&str> = vec!["AMD", "AAPL", "NVDA", "TSLA", "O"];

    portfolio.cash_balance = 1000.0;
    println!("!ALGO MODE (Moving Average Crossover)!");
    println!("Starting with ${}", portfolio.cash_balance);
    println!("tickers watching: {:?}", tickers_to_watch);

    // TODO GRAB HISTORICAL DATA FROM alphavantage
    // maybe calucalte weekly average and compare to daily
    // or mothly to weekly
    //
    // Moving average, = (Sum(closing prices over the period))/(number days )
    //

    let mut historical_stock_data: Vec<moving_average_stock> = Vec::new();
    for stock in &tickers_to_watch {
        historical_stock_data.push(moving_average_stock {
            ticker: stock.to_string(),
            ten_day_moving_averages: Vec::new(),
            fifty_day_moving_averages: Vec::new(),
        });
    }
    let today = Local::now().date_naive();
    let formatted_date = today.format("%Y-%m-%d").to_string();

    dbg!(formatted_date);

    loop {
        for ticker in &tickers_to_watch {
            let stocks_with_historical_data =
                match portfolio_code::get_files_in_directory("./stock_data/") {
                    Ok(save_files_names) => save_files_names,
                    Err(e) => {
                        eprintln!("failed");
                        log::error!("Failed to get files in /stock_data dir: {}", e);
                        break;
                    }
                };

            let mut file_exisits = false;

            for file in stocks_with_historical_data {
                if ticker.to_owned().to_owned() + ".txt" == file {
                    file_exisits = true;
                }
            }

            if file_exisits == false {
                log::info!("File does not exist in /stocks_data dir");
                println!(
                    "ticker: {}, does not have any data, attempting to download now...",
                    ticker
                );
                match api::get_20_years_old_historial_data(ticker) {
                    Ok(stock_data) => {
                        println!("success downloading data");
                        log::info!("Attemping to write data to file, ticker: {}", ticker);
                        let path = "./stock_data/".to_owned() + ticker + ".txt";
                        let data_to_write = stock_data.join("\n");
                        fs::write(path, data_to_write).expect("Unable to write file");
                        log::info!("Successfully written data to file, ticker: {}", ticker);
                    }
                    Err(e) => {}
                }
            }

            let all_data_lines =
                portfolio_code::lines_from_file("./stock_data/".to_owned() + ticker + ".txt");
            let fifty_days_history: Vec<String> =
                (&all_data_lines[all_data_lines.len() - 50..]).to_vec();
            let ten_days_history: Vec<String> =
                (&all_data_lines[all_data_lines.len() - 10..]).to_vec();

            let mut sum: f32 = 0.0;
            for day in fifty_days_history {
                let temp: Vec<&str> = day.split(':').collect();
                sum += temp[1].parse::<f32>().expect("FAILED TO PARSE f32")
            }
            let moving_average_ten = sum / 50.0;

            sum = 0.0;
            for day in ten_days_history {
                let temp: Vec<&str> = day.split(':').collect();
                sum += temp[1].parse::<f32>().expect("FAILED TO PARSE f32")
            }
            let moving_average_fifty = sum / 10.0;

            println!("{} 10 day moving average: {}", ticker, moving_average_ten);
            println!("{} 50 day moving average: {}", ticker, moving_average_fifty);
            let today = Local::now().date_naive();
            let formatted_date = today.format("%Y-%m-%d").to_string();

            for mut stock in &mut historical_stock_data {
                if stock.ticker == *ticker {
                    let temp_ten_day_moving_average = moving_average {
                        average: moving_average_ten,
                        date: formatted_date.clone(),
                    };
                    let temp_fifty_day_moving_average = moving_average {
                        average: moving_average_fifty,
                        date: formatted_date.clone(),
                    };
                    stock
                        .ten_day_moving_averages
                        .push(temp_ten_day_moving_average);
                    stock
                        .fifty_day_moving_averages
                        .push(temp_fifty_day_moving_average);
                }
            }
        }
        dbg!(&historical_stock_data);

        // wait here for x amount of time, if market open then run script
        println!("waiting here for 24 hours");
        thread::sleep(Duration::from_secs(60 * 60 * 24)); // Wait for 24 hours
                                                          //

        // update all the stocks info
        for ticker in &tickers_to_watch {
            match api::get_20_years_old_historial_data(ticker) {
                Ok(stock_data) => {
                    println!("success downloading data");
                    log::info!("Attemping to write data to file, ticker: {}", ticker);
                    let path = "./stock_data/".to_owned() + ticker + ".txt";
                    let data_to_write = stock_data.join("\n");
                    fs::write(path, data_to_write).expect("Unable to write file");
                    log::info!("Successfully written data to file, ticker: {}", ticker);
                }
                Err(e) => {
                    eprintln!("error: {}", e)
                }
            }
        }
    }

    portfolio
}

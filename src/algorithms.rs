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
use crate::portfolio_code;
use crate::portfolio_code::Portfolio;
use crate::trade;

pub fn percentage_change_trigger_algo(mut portfolio: Portfolio) -> Portfolio {
    /*
        This algorithm, checks a list of stocks, if stock has gone up 2% since inital price
        Then buy 1 share, then if the trade goes up 10% close the trade.
    */
    let tickers_to_watch: Vec<&str> = vec![
        "PLTR", // Palantir Technologies
        "TSLA", // Tesla Inc.
        "NVDA", // NVIDIA Corporation
        "FGL", "LCID",
    ];
    portfolio.cash_balance = 1000.0;

    println!("!ALGO MODE (Percentage Change Trigger)!");
    println!("Starting with ${}", portfolio.cash_balance);
    println!("tickers watching: {:?}", tickers_to_watch);

    let mut inital_price_for_ticker: HashMap<&str, f32> = HashMap::new();

    for ticker in tickers_to_watch {
        let price_per_share = api::finnhub_get_current_stock_price(ticker).unwrap();
        inital_price_for_ticker.insert(ticker, price_per_share);
    }

    loop {
        for mut stock in &inital_price_for_ticker {
            let current_stock_price = api::finnhub_get_current_stock_price(stock.0).unwrap();

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

        let ten_secs = time::Duration::from_millis(10000);
        thread::sleep(ten_secs);
    }
}

pub fn moving_average_crossover_algo(mut portfolio: Portfolio) -> Portfolio {
    let tickers_to_watch: Vec<&str> = vec![
        "PLTR", // Palantir Technologies
               // "TSLA", // Tesla Inc.
               // "NVDA", // NVIDIA Corporation
               // "GME",  // GameStop Corp.
               // "LCID", // Lucid Group Inc.
    ];

    portfolio.cash_balance = 1000.0;
    println!("!ALGO MODE (Moving Average Crossover)!");
    println!("Starting with ${}", portfolio.cash_balance);
    println!("tickers watching: {:?}", tickers_to_watch);

    let mut ticker_info: HashMap<&str, Vec<f32>> = HashMap::new();

    // TODO GRAB HISTORICAL DATA FROM alphavantage
    // maybe calucalte weekly average and compare to daily
    // or mothly to weekly

    portfolio
}

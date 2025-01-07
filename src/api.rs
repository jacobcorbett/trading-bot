use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::hash_set::Difference;
use std::f64::consts::PI;
use std::process::exit;
use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;

#[tokio::main]
pub async fn finnhub_get_current_stock_price(ticker: &str) -> Result<f32, Error> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("FINHUB_API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };
    // let url = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&extended_hours=true&symbol=TSLA&interval=1min&apikey=".to_owned() + &api_key;

    let url = "https://finnhub.io/api/v1/quote?symbol=".to_owned() + ticker + "&token=" + &api_key;

    // let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo";

    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    //println!("{:#?}", response);
    let price = response["c"].to_string();
    // let float_price: f32 = price.parse().unwrap();
    let temp: Vec<char> = price.chars().collect();

    // temp.remove(0); // removes first "
    // temp.pop(); // removes last "

    let temp_string: String = temp.iter().collect();
    let temp_f32: f32 = temp_string.parse().expect("Failed to parse f32");

    //dbg!(temp_f32);

    Ok(temp_f32)
}

#[tokio::main]
pub async fn is_market_open() -> Result<bool, Error> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("FINHUB_API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };

    let url =
        "https://finnhub.io/api/v1/stock/market-status?exchange=US&token=".to_owned() + &api_key;

    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    // dbg!(&response["isOpen"].as_bool());

    let mut open_or_closed: bool = false;
    if let Some(value) = response["isOpen"].as_bool() {
        open_or_closed = value as bool;
    } else {
        // handle error
    }

    Ok(open_or_closed)
}

#[tokio::main]
pub async fn get_last_100_days_price_data(ticker: &str) -> Result<Vec<f32>, Error> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("ALPHA_API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };
    // let url = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&extended_hours=true&symbol=TSLA&interval=1min&apikey=".to_owned() + &api_key;
    let url = "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&apikey=demo";

    // let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=".to_owned()
    //     + ticker
    //     + "&apikey="
    //     + &api_key;

    // let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo";

    let mut close_data_points: Vec<f32> = Vec::new();

    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    //println!("{:#?}", response["Time Series (Daily)"]);

    if let Some(time_data) = response["Time Series (Daily)"].as_object() {
        for (date, data) in time_data {
            dbg!(date);
            dbg!(data);
            if let Some(close_price) = data.get("4. close") {
                dbg!(close_price);
                let temp = (close_price.as_str().expect("REASON")).parse::<f32>();
                close_data_points.push(temp.expect("REASON"));
            }
        }
    }

    Ok(close_data_points)
}

#[tokio::main]
pub async fn check_vaild_ticker(ticker: &str) -> Result<bool, Error> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("FINHUB_API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };

    let url = "https://finnhub.io/api/v1/search?q=".to_owned()
        + ticker
        + "&exchange=US&token="
        + &api_key;

    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;

    let mut length = 0;
    if let Some(value) = response["count"].as_i64() {
        length = value as i32;
    } else {
        // handle error
    }

    let mut valid = false;

    for i in 0..length {
        let current_ticker = &response["result"][i as usize]["displaySymbol"];

        if current_ticker == ticker {
            valid = true;
            break;
        }
    }

    Ok(valid)
}

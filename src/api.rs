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
pub async fn finnhub_get_current_stock_price(ticker: &str) -> Result<f32, String> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("FINHUB_API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };

    let url = "https://finnhub.io/api/v1/quote?symbol=".to_owned() + ticker + "&token=" + &api_key;

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    if let Some(price) = response.get("c").and_then(|v| v.as_f64()) {
        Ok(price as f32)
    } else {
        Err("Missing or invalid 'c' field in response".to_string())
    }
}

#[tokio::main]
pub async fn is_market_open() -> Result<bool, String> {
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

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    if let Some(value) = response["isOpen"].as_bool() {
        let open_or_closed = value as bool;
        Ok(open_or_closed)
    } else {
        return Err("Failed to find 'isOpen' in reponse".to_string());
    }
}

#[tokio::main]
pub async fn get_20_years_old_historial_data(ticker: &str) -> Result<Vec<String>, String> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("ALPHA_API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };
    // let url = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&extended_hours=true&symbol=TSLA&interval=1min&apikey=".to_owned() + &api_key;

    // let url =
    //     "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&outputsize=full&apikey=demo";

    let url = "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=".to_owned()
        + ticker
        + "&outputsize=full&apikey="
        + &api_key;

    let response = reqwest::get(&*url)
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    //println!("{:#?}", response["Time Series (Daily)"]);

    let mut stock_data_points: Vec<String> = Vec::new();

    if let Some(time_data) = response["Time Series (Daily)"].as_object() {
        for (date, data) in time_data {
            let mut temp_line = String::new();

            temp_line += date;
            temp_line += ":";

            if let Some(close_price) = data.get("4. close") {
                //dbg!(close_price);

                let close_price_string = close_price.to_string();
                let foo = close_price_string.trim().trim_matches('"');
                //println!("{}", foo);
                temp_line += foo;
            } else {
                return Err("Failed to find 4. close in data ".to_string());
            }

            //println!("{}", temp_line);
            stock_data_points.push(temp_line);
        }
    } else {
        return Err("Failed to find Time Series (Daily) in response ".to_string());
    }

    Ok(stock_data_points)
}

#[tokio::main]
pub async fn check_vaild_ticker(ticker: &str) -> Result<bool, String> {
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

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    if let Some(value) = response["count"].as_i64() {
        let length = value as i32;
        for i in 0..length {
            let current_ticker = &response["result"][i as usize]["displaySymbol"];
            if current_ticker == ticker {
                return Ok(true);
            }
        }
    } else {
        return Err("Failed to find 'count' in response".to_string());
    }

    Ok(false)
}

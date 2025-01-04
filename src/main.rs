use dotenv::dotenv;
use reqwest::Error;
use std::collections::hash_set::Difference;
use std::process::exit;
use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;

#[derive(Debug)]
struct portfolio {
    cash_balance: f32,
    assets: HashMap<String, f32>,
    open_trades: Vec<trade_position>,
}

#[derive(Debug)]
struct trade_position {
    uuid: Uuid,
    ticker: String,
    size: f32,
    open_price: f32,
    close_price: f32,
    inital_value: f32,
}

// """"
//
#[tokio::main]
async fn finnhub_get_current_stock_price(ticker: &str) -> Result<f32, Error> {
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
    //println!("{:#?}", response["c"]);
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
async fn check_vaild_ticker(ticker: &str) -> Result<bool, Error> {
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

fn update_cash_balance(mut portfolio: portfolio, update_value: f32) -> portfolio {
    let current_value = portfolio.cash_balance;
    let new_value = current_value + update_value;
    portfolio.cash_balance = new_value;
    return portfolio;
}

fn open_trade(mut portfolio: portfolio, ticker: &str, amount_of_shares: f32) -> portfolio {
    let uuid = Uuid::new_v4();
    let price_per_share = finnhub_get_current_stock_price(ticker).unwrap();

    let total_value = price_per_share * amount_of_shares;

    if total_value > portfolio.cash_balance {
        println!("not enough money");
        return portfolio;
    }

    let temp = trade_position {
        uuid: uuid,
        ticker: ticker.to_string(),
        size: amount_of_shares,
        open_price: price_per_share,
        close_price: -1.0,
        inital_value: total_value,
    };

    portfolio.open_trades.push(temp);
    portfolio.cash_balance -= total_value;

    portfolio
}

fn close_trade(mut portfolio: portfolio, trade_uuid: Uuid) -> portfolio {
    let mut index_to_remove = 1000000;

    for i in 0..portfolio.open_trades.len() {
        if portfolio.open_trades[i].uuid == trade_uuid {
            index_to_remove = i;
        }
    }

    if index_to_remove == 1000000 {
        println!("No matching open trades");
        return portfolio;
    } else {
        portfolio.open_trades.remove(index_to_remove);
    }
    portfolio
}

fn status_of_all_trades(portfolio: portfolio) -> portfolio {
    if portfolio.open_trades.len() == 0 {
        println!("Cash Balance: ${}", portfolio.cash_balance);
        println!("No Open Trades");
        return portfolio;
    }
    println!("Cash Balance: ${}\n", portfolio.cash_balance);
    for trade in &portfolio.open_trades {
        println!("UUID: {}", trade.uuid);
        println!("Ticker: {}", trade.ticker);
        println!("Amount of Shares: {}", trade.size);
        println!("Trade open price: ${}", trade.open_price);
        let current_stock_price = finnhub_get_current_stock_price(&trade.ticker).unwrap();
        println!("Current Price: ${:?}", current_stock_price);
        let profit_or_loss = (current_stock_price - trade.open_price);
        println!("Profit/Loss: ${:?}", profit_or_loss);
        let total_value = current_stock_price * trade.size;
        println!("Total Value: ${:?}", total_value);
        println!(" ");
    }
    return portfolio;
}

fn algo(portfolio: portfolio) -> portfolio {
    println!("!ALGO MODE!");
    println!("Starting with ${}", portfolio.cash_balance);

    println!("NO ALGORITHM IMPLITMENTED YET");

    // loop {
    //     println!("checking");
    //     for trade in &portfolio.open_trades {
    //         let current_stock_price = finnhub_get_current_stock_price(&trade.ticker).unwrap();
    //         let profit_or_loss = (current_stock_price - trade.open_price);

    //         if profit_or_loss > 1.0 {
    //             println!("MADE $1 on {:?}", trade)
    //         }
    //     }
    //     let ten_secs = time::Duration::from_millis(10000);
    //     thread::sleep(ten_secs);
    // }

    return portfolio;
}

fn open_trade_menu_function(mut portfolio: portfolio) -> portfolio {
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
    let vaild_or_not = check_vaild_ticker(ticker);

    match vaild_or_not {
        Ok(is_valid) => {
            if is_valid {
                println!("ticker is valid")
            } else {
                println!("invalid ticker");
                return portfolio;
            }
        }
        Err(e) => {
            println!("An error occurred: {}", e);
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
    portfolio = open_trade(portfolio, ticker, number_of_shares.expect("REASON"));
    portfolio
    //
}

fn status_of_all_trades_menu_function(mut portfolio: portfolio) -> portfolio {
    println!(" ");
    portfolio = status_of_all_trades(portfolio);
    println!(" ");
    portfolio
}

fn algorithm_menu_function(mut portfolio: portfolio) -> portfolio {
    portfolio = algo(portfolio);
    portfolio
}

fn closeing_trade_menu_function(mut portfolio: portfolio) -> portfolio {
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

    portfolio = close_trade(portfolio, trade_uuid.expect("REASON"));

    portfolio
}

fn add_cash_menu_function(mut portfolio: portfolio) -> portfolio {
    println!("Enter $ amount to change cash balance by: ");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let change_value = line.trim().parse::<f32>();

    portfolio = update_cash_balance(portfolio, change_value.expect("REASON"));
    portfolio
}

fn main() {
    let mut main_portfolio = portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
        open_trades: Vec::new(),
    };

    loop {
        let mut line = String::new();
        println!("Commands:\ns: status of all trades\no: open new single trade\nc: close single trade\na: algorithm mode\nm: add cash\nq: quit\n");
        println!("Enter command :");

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let command = line.trim();

        if command == "s" {
            main_portfolio = status_of_all_trades_menu_function(main_portfolio);
        } else if command == "o" {
            main_portfolio = open_trade_menu_function(main_portfolio);
        } else if command == "c" {
            main_portfolio = closeing_trade_menu_function(main_portfolio);
        } else if command == "a" {
            main_portfolio = algorithm_menu_function(main_portfolio);
        } else if command == "m" {
            main_portfolio = add_cash_menu_function(main_portfolio);
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

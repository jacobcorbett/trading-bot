use dotenv::dotenv;
use reqwest::Error;
use std::collections::hash_set::Difference;
use std::{collections::HashMap, env};
use std::{thread, time};

#[derive(Debug)]
struct portfolio {
    cash_balance: f32,
    assets: HashMap<String, f32>,
    open_trades: Vec<trade_position>,
}

#[derive(Debug)]
struct trade_position {
    ticker: String,
    size: f32,
    open_price: f32,
    close_price: f32,
    inital_value: f32,
    // time_open: ?,
    // time_closed: ?,
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

#[tokio::main] // DEAD
async fn get_current_stock_price(ticker: &str) -> Result<f32, Error> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("ALPHA_API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };
    // let url = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&extended_hours=true&symbol=TSLA&interval=1min&apikey=".to_owned() + &api_key;

    let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=".to_owned()
        + ticker
        + "&apikey="
        + &api_key;

    // let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo";

    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    println!("{:#?}", response["Global Quote"]["05. price"]);
    let price = response["Global Quote"]["05. price"].to_string();
    // let float_price: f32 = price.parse().unwrap();
    let mut temp: Vec<char> = price.chars().collect();

    temp.remove(0); // removes first "
    temp.pop(); // removes last "

    let temp_string: String = temp.iter().collect();
    let temp_f32: f32 = temp_string.parse().expect("Failed to parse f32");

    dbg!(temp_f32);

    Ok(temp_f32)
}

fn calculate_portfolio_worth(portfolio: portfolio) -> f32 {
    let mut total: f32 = 0.0;
    for stock in portfolio.assets {
        let ticker = stock.0;
        let amount_shares = stock.1;
        let price_per_share = finnhub_get_current_stock_price(ticker.as_str());

        match price_per_share {
            Ok(value) => {
                let share_worth = amount_shares * value;
                println!(
                    "ticker: {}, shares: {}, price per share: {:?}, total: {:?}",
                    ticker,
                    amount_shares,
                    price_per_share.unwrap(),
                    share_worth
                );
                total += share_worth;
            }
            Err(err) => {
                println!("An error occurred: {:?}", err);
            }
        }
    }

    return total;
}

fn add_stock_to_portfolio(
    mut portfolio: portfolio,
    symbol: String,
    amount_of_shares: f32,
) -> portfolio {
    //println!("Adding, {}: {}, to portfolio", symbol, amount_of_shares);
    portfolio.assets.insert(symbol, amount_of_shares);
    return portfolio;
}

fn remove_stock_from_portfolio(mut portfolio: portfolio, symbol: String) -> portfolio {
    portfolio.assets.remove(&symbol);
    portfolio
}

fn update_cash_balance(mut portfolio: portfolio, update_value: f32) -> portfolio {
    let current_value = portfolio.cash_balance;
    let new_value = current_value + update_value;
    portfolio.cash_balance = new_value;
    return portfolio;
}

fn update_stock_postion(mut portfolio: portfolio, symbol: String, update_value: f32) -> portfolio {
    // let current_value = portfolio.cash_balance;
    // let new_value = current_value + update_value;

    match portfolio.assets.get_mut(&symbol) {
        Some(value) => {
            let new_value = *value + update_value;
            *value = new_value
        }
        None => {
            println!("NONE")
        }
    }

    portfolio
}

fn open_trade(ticker: &str, amount_of_shares: f32) -> trade_position {
    let price_per_share = finnhub_get_current_stock_price(ticker).unwrap();

    let total_value = price_per_share * amount_of_shares;

    let temp = trade_position {
        ticker: ticker.to_string(),
        size: amount_of_shares,
        open_price: price_per_share,
        close_price: -1.0,
        inital_value: total_value,
    };
    temp
}

fn status_of_all_trades(portfolio: portfolio) -> portfolio {
    for trade in &portfolio.open_trades {
        println!("Trade: {}", trade.ticker);
        println!("Amount of Shares: {}", trade.size);
        println!("Trade open price: ${}", trade.open_price);
        let current_stock_price = finnhub_get_current_stock_price(&trade.ticker).unwrap();
        println!("Current Price: ${:?}", current_stock_price);
        let profit_or_loss = (current_stock_price - trade.open_price);
        println!("Profit/Loss : ${:?}", profit_or_loss);
        println!(" ");
    }
    return portfolio;
}

fn main() {
    let mut main_portfolio = portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
        open_trades: Vec::new(),
    };

    // let cost_to_buy_x_shares: f32 = price * shares;

    // println!("STOCK: {}, PRICE: ${:?}", ticker, price);
    // println!(
    //     "STOCK: {}, PRICE: ${:?}, Cost to buy {:?} shares = ${:?}",
    //     ticker, price, shares, cost_to_buy_x_shares
    // );
    main_portfolio = update_cash_balance(main_portfolio, 100.0);
    main_portfolio = update_cash_balance(main_portfolio, -120.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "AAPL".to_string(), 2.0);
    main_portfolio = remove_stock_from_portfolio(main_portfolio, "AAPL".to_string());
    main_portfolio = add_stock_to_portfolio(main_portfolio, "MSFT".to_string(), 1.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "GOOGL".to_string(), 2.0);
    main_portfolio = update_stock_postion(main_portfolio, "GOOGL".to_string(), -2.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "TSLA".to_string(), 3.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "AMZN".to_string(), 4.3);
    main_portfolio = update_stock_postion(main_portfolio, "AMZN".to_string(), -1.0);

    dbg!(&main_portfolio);
    // dbg!(calculate_portfolio_worth(main_portfolio));

    // let temp = trade_position {
    //     ticker: "AAPL".to_string(),
    //     open_price: -100000000.0,
    //     close_price: -100000000.0,
    // };

    // dbg!(temp);
    //

    let x = open_trade("GME", 100.0);
    main_portfolio.open_trades.push(x);
    let x = open_trade("AAPL", 10.0);
    main_portfolio.open_trades.push(x);
    let x = open_trade("TSLA", 100.0);
    main_portfolio.open_trades.push(x);

    //dbg!(&main_portfolio);

    loop {
        let mut line = String::new();
        println!("Enter command :");
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let command = line.trim();

        if command == "c" {
            println!(" ");
            main_portfolio = status_of_all_trades(main_portfolio);
            println!(" ");
        } else if command == "q" {
            println!("Exiting..");
            break;
        }
    }

    // loop {
    //     let second = time::Duration::from_millis(10000);
    //     thread::sleep(second);

    //     println!(" ");
    //     println!(
    //         "Trade open price: ${}",
    //         main_portfolio.open_trades[0].open_price
    //     );
    //     let c_price = finnhub_get_current_stock_price(ticker).unwrap();
    //     println!("Current Price: ${:?}", c_price);

    //     let profit_loss = (c_price - main_portfolio.open_trades[0].open_price);
    //     println!("Profit/Loss : ${:?}", profit_loss);
    // }

    // loop {
    //     let second = time::Duration::from_millis(10000);
    //     thread::sleep(second);

    //     let ticker = "TSLA";
    //     let x = finnhub_get_current_stock_price(ticker).unwrap();

    //     println!("{}: ${:?}", ticker, x);
    // }

    // let ticker = "AAPL";
    // let price: f32 = 243.85;
    // let shares: f32 = 1.0;

    // if main_portfolio.cash_balance > price * shares {
    //     println!("you can buy")
    // } else {
    //     println!("you cant affort it ")
    // }
}

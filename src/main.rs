use dotenv::dotenv;
use reqwest::Error;
use std::env;

#[tokio::main]
async fn get_stock_price() -> Result<f32, Error> {
    dotenv().ok(); // Reads the .env file
    let api_key = match env::var("API_KEY") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };

    // let url = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&extended_hours=true&symbol=TSLA&interval=1min&apikey=".to_owned() + &api_key;
    // let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=TSLA&apikey="
    //     .to_owned()
    //     + &api_key;

    let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo";

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

fn main() {
    println!("Hello, world!");

    dotenv().ok(); // Reads the .env file
    let api_key = env::var("API_KEY");

    // match api_key {
    //     Ok(val) => println!("API_KEY: {:?}", val),
    //     Err(e) => println!("Error API_KEY: {}", e),
    // }
    //
    let x = get_stock_price().unwrap();
    println!("STOCK: {}, PRICE: ${:?}", "IBM", x)
}

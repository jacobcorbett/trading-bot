use dotenv::dotenv;
use reqwest::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok(); // Reads the .env file
    let api_key = env::var("API_KEY");
    // GET request
    //
    let url = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=1min&apikey=demo";
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    println!("{:#?}", response["Time Series (1min)"]);

    Ok(())
}

// #[tokio::main]
// async fn main() {
//     println!("Hello, world!");

//     dotenv().ok(); // Reads the .env file
//     let api_key = env::var("API_KEY");

//     // match api_key {
//     //     Ok(val) => println!("API_KEY: {:?}", val),
//     //     Err(e) => println!("Error API_KEY: {}", e),
//     // }
//     //
//     test().await;
// }

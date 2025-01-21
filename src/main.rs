use dotenv::dotenv;
use flexi_logger::{FileSpec, Logger, WriteMode};
use log;

use reqwest::{Error, Response};
use serde_json::Value;
use sqlx::{pool::maybe, postgres::PgCopyIn, query, PgPool, Row};
use std::collections::hash_set::Difference;
use std::error::Error as OtherError;
use std::f64::consts::PI;
use std::process::exit;
use std::{collections::HashMap, env};
use std::{thread, time};
use uuid::Uuid;
mod algorithms;
mod api;
mod menu_functions;
mod portfolio_code;
mod trade;
use crate::portfolio_code::Portfolio;

struct dataBaseTest {
    pub name: String,
}

fn get_market_status() -> String {
    match api::is_market_open() {
        Ok(true) => {
            log::info!("Succsefully got market status and its: {}", true);
            "open".to_string()
        }
        Ok(false) => {
            log::info!("Succsefully got market status and its: {}", false);
            "closed".to_string()
        }
        Err(e) => {
            log::error!(
                "An error occurred when trying to fetch market status: {}",
                e
            );
            println!("An error occurred: {}", e);
            "An error occurred in fetching status of market".to_string()
        }
    }
}

fn enable_logging() {
    // Initialize the logger
    Logger::try_with_str("trace") // Set log level to "info"
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("../logs") // Set directory to "logs"
                .suffix("log"), // Optional: add .log suffix to files
        )
        .format(flexi_logger::detailed_format)
        .write_mode(WriteMode::BufferAndFlush) // Buffer logs and flush periodically
        .start()
        .unwrap();
}

#[tokio::main]
async fn connect_to_database() {
    dotenv().ok(); // Reads the .env file
    let url = match env::var("DATABASE_URL") {
        Ok(key) => key, // If the environment variable exists, use its value
        Err(_) => {
            eprintln!("Error: DATABASE_URL environment variable not found.");
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };

    let pool = match sqlx::postgres::PgPool::connect(&url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1); // Exit the program with a non-zero status code
        }
    };

    let q = "SELECT * FROM newtable";
    let query = sqlx::query::<sqlx::Postgres>(q);

    match query.fetch_all(&pool).await {
        Ok(rows) => {
            dbg!(rows);
        }
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
        }
    }
}

fn main() {
    enable_logging();
    // log::error!("Critical error occurred: {}", "database connection failed");
    // log::warn!("Warning: approaching memory limit at {}%", 90);
    // log::info!("Server started successfully on port {}", 8080);
    // log::debug!("Processing request with id: {}", "12345");
    // log::trace!("Entering function with args: {:?}", vec![1, 2, 3]);

    log::info!("Program started");

    let mut main_portfolio = portfolio_code::blank_portfolio();
    log::info!("Created main_portfolio: {:?}", main_portfolio);

    /// attemping to connect to database
    ///
    connect_to_database();

    ///

    log::info!("Starting main loop");
    loop {
        let market_status = get_market_status();

        println!("\nMarket Status: {}", market_status);
        println!("Commands:\ns: status of all trades\no: open new single trade\nc: close single trade\nd: download stock data\na: algorithm mode\nm: add cash\nS: save state\nR: load state\nq: quit\n");
        let mut line = String::new();
        println!("Enter command :");

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let command = line.trim();
        log::info!("Reading user input: \'{}\'", command);

        if command == "s" {
            main_portfolio = menu_functions::status_of_all_trades_menu_function(main_portfolio);
        } else if command == "o" {
            main_portfolio = menu_functions::open_trade_menu_function(main_portfolio);
        } else if command == "c" {
            main_portfolio = menu_functions::closeing_trade_menu_function(main_portfolio);
        } else if command == "a" {
            main_portfolio = menu_functions::algorithm_menu_function(main_portfolio);
        } else if command == "d" {
            menu_functions::download_stock_data_menu_function();
        } else if command == "m" {
            main_portfolio = menu_functions::add_cash_menu_function(main_portfolio);
        } else if command == "R" {
            main_portfolio = menu_functions::load_state_menu_function(main_portfolio);
        } else if command == "S" {
            main_portfolio = menu_functions::save_state_menu_function(main_portfolio);
        } else if command == "q" {
            println!("Exiting..");
            break;
        }
    }
}

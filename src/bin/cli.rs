use std::str::FromStr;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgConnectOptions, PgPool};
use tokio::time::{sleep, Duration};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    UpdateAssets,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    match &cli.command {
        Commands::UpdateAssets => update_assets().await,
    }
}

#[derive(Debug, Deserialize)]
struct TokenInfo {
    address: String,
    decimals: i64,
    name: String,
    symbol: String,
    #[serde(rename = "logoURI")]
    logo_uri: Option<String>,
}

async fn update_assets() {
    println!("fetching assets from Jupiter...");

    let database_url = std::env::var("DB_CONNECTION_STRING")
        .expect("DATABASE_URL must be set in environment");

    let options = PgConnectOptions::from_str(&database_url)
        .expect("failed to parse database url");

    // create connection pool
    let pool = PgPool::connect_with(options)
        .await
        .expect("failed to create connection pool");

    // fetch existing token addresses from the database
    let existing_addresses: Vec<String> = sqlx::query_scalar("SELECT address FROM tokens")
        .fetch_all(&pool)
        .await
        .expect("failed to fetch existing token addresses");

    println!("found {} existing tokens in database", existing_addresses.len());

    let client = reqwest::Client::new();
    let response = client
        .get("https://token.jup.ag/all")
        .send()
        .await
        .expect("failed to fetch assets from Jupiter");

    let tokens = response
        .json::<Vec<TokenInfo>>()
        .await
        .expect("failed to parse assets from Jupiter");

    // filter out tokens without logo_uri and those that already exist in the database
    let tokens: Vec<TokenInfo> = tokens.into_iter()
        .filter(|token| token.logo_uri.is_some() && !existing_addresses.contains(&token.address))
        .collect();

    println!("found {} new tokens with logos to add", tokens.len());

    if tokens.is_empty() {
        println!("no new tokens to add to database");
        return;
    }

    // process tokens in batches of 1000
    for (i, chunk) in tokens.chunks(1000).enumerate() {
        println!("processing chunk {}", i + 1);

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO tokens (address, decimals, logo_uri, name, symbol) "
        );

        query_builder.push_values(chunk, |mut b, token| {
            b.push_bind(&token.address)
                .push_bind(token.decimals)
                .push_bind(&token.logo_uri)
                .push_bind(&token.name)
                .push_bind(&token.symbol);
        });

        match query_builder.build().execute(&pool).await {
            Ok(result) => println!("successfully inserted batch of {} tokens", result.rows_affected()),
            Err(e) => eprintln!("error inserting batch: {}", e),
        }

        sleep(Duration::from_millis(100)).await;
    }

    println!("finished updating tokens database");
}

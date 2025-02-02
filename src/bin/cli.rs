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

    // filter out tokens without logo_uri
    let tokens: Vec<TokenInfo> = tokens.into_iter()
        .filter(|token| token.logo_uri.is_some())
        .collect();

    println!("successfully fetched {} tokens with logos", tokens.len());

    // process tokens in batches of 1000
    for (i, chunk) in tokens.chunks(1000).enumerate() {
        println!("processing chunk ({})", (i+1) * chunk.len());

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

        query_builder.push(
            " ON CONFLICT (address) DO UPDATE SET
                decimals = EXCLUDED.decimals,
                logo_uri = EXCLUDED.logo_uri,
                name = EXCLUDED.name,
                symbol = EXCLUDED.symbol"
        );

        match query_builder.build().execute(&pool).await {
            Ok(result) => println!("successfully upserted batch of {} tokens", result.rows_affected()),
            Err(e) => eprintln!("error upserting batch: {}", e),
        }

        sleep(Duration::from_millis(100)).await;
    }

    println!("finished updating tokens database");
}

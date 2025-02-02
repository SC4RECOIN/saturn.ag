use clap::{Parser, Subcommand};
use dotenv::dotenv;
use serde::Deserialize;

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

    println!("successfully fetched {} tokens", tokens.len());

    for token in tokens.iter().take(5) {
        println!("Token: {} ({:?})", token.name, token);
    }
}

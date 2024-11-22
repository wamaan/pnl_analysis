//Amaan Haque
use reqwest;
use serde::Deserialize;
use tokio;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
use std::error::Error;
use rand::Rng;

#[derive(Debug, Deserialize)]
struct MarketChart {
    prices: Vec<[f64; 2]>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Fetch historical USDT/USD rates
    let usdt_usd_rates = fetch_historical_rates("tether").await?;
    // Fetch historical BTC prices
    let btc_prices = fetch_historical_rates("bitcoin").await?;
    // Fetch historical ETH prices
    let eth_prices = fetch_historical_rates("ethereum").await?;

    // Ensure all datasets have the same length
    let min_length = usdt_usd_rates.len().min(btc_prices.len()).min(eth_prices.len());

    // Truncate datasets to the minimum length
    let usdt_usd_rates = &usdt_usd_rates[..min_length];
    let btc_prices = &btc_prices[..min_length];
    let eth_prices = &eth_prices[..min_length];

    // Generate dummy PnL data
    let pnl_data = generate_dummy_pnl(usdt_usd_rates);

    // Prepare data for regression
    let data_for_regression =
        prepare_regression_data(&pnl_data, usdt_usd_rates, btc_prices, eth_prices);

    // Build regression data
    let regression_data = RegressionDataBuilder::new().build_from(data_for_regression)?;

    // Build and fit the regression model
    let formula = "y ~ usdt_usd_rate + btc_price + eth_price";
    let model = FormulaRegressionBuilder::new()
        .data(&regression_data)
        .formula(formula)
        .fit()?;

    // Output the regression results
    println!("Regression coefficients:");
    println!("Intercept: {}", model.parameters.intercept_value);
    for (variable, coefficient) in model.parameters.regressor_names.iter().zip(model.parameters.regressor_values.iter()) {
        println!("{}: {}", variable, coefficient);
    }
    println!("R-squared: {}", model.rsquared);

    Ok(())
}

async fn fetch_historical_rates(coin_id: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency=usd&days=2",
        coin_id
    );
    let response = reqwest::get(&url).await?.json::<MarketChart>().await?;
    // Extract only prices
    let rates: Vec<f64> = response.prices.iter().map(|entry| entry[1]).collect();
    Ok(rates)
}

fn generate_dummy_pnl(usdt_usd_rates: &[f64]) -> Vec<f64> {
    let mut pnl_data = Vec::new();
    let mut rng = rand::thread_rng();

    for &usdt_usd_rate in usdt_usd_rates {
        // Simulate PnL influenced by USDT/USD rate
        let noise: f64 = rng.gen_range(-50.0..50.0);
        let pnl = 1000.0 + (usdt_usd_rate - 1.0) * 100000.0 + noise;
        pnl_data.push(pnl);
    }
    pnl_data
}

fn prepare_regression_data(
    pnl_data: &[f64],
    usdt_usd_rates: &[f64],
    btc_prices: &[f64],
    eth_prices: &[f64],
) -> Vec<(&'static str, Vec<f64>)> {
    vec![
        ("y", pnl_data.to_vec()),
        ("usdt_usd_rate", usdt_usd_rates.to_vec()),
        ("btc_price", btc_prices.to_vec()),
        ("eth_price", eth_prices.to_vec()),
    ]
}
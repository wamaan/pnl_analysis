# pnl_analysis
case study task for typoon.
results:
usdt_usd_rate: 91489.45939874882
R-squared: 0.9055018743189328


This Rust project performs linear regression analysis on intraday Profit and Loss (PnL) data using cryptocurrency market variables. It helps identify the relationship between PnL and key factors such as the USDT/USD conversion rate, Bitcoin price, and Ethereum price.

How It Works

The project uses a regression model to explain variations in PnL based on three independent variables:
1. usdt_usd_rate: The USDT/USD conversion rate.
2. btc_price: The price of Bitcoin.
3. eth_price: The price of Ethereum.

## **Setup Instructions**

### **1. Prerequisites**
- Install Rust from [rustup.rs](https://rustup.rs).
- Ensure you have `cargo`, Rust's package manager, installed.

### **2. Clone the Repository**
Clone this repository to your local machine:
```bash
git clone https://github.com/wamaan/pnl_analysis.git
cd pnl_analysis
```

### **3. Install Dependencies**
Add the following dependencies to your `Cargo.toml` file:
```toml
[dependencies]
csv = "1.1"
serde = { version = "1.0", features = ["derive"] }
ndarray = "0.15"
ndarray-stats = "0.4"
linregress = "0.2"
rand = "0.8"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

Run `cargo build` to download and compile these dependencies.

---

## **Usage**

### **1. Run the Program**
To execute the program, run:
```bash
cargo run
```

## **Code Overview**

### **Main Components**
1. **Data Structures**:
   - `Record`: Represents each row of data with fields like `timestamp`, `pnl`, `usdt_usd_rate`, etc.

2. **Dummy Data Generation**:
   - Simulates realistic PnL influenced by market variables for testing purposes.

3. **Regression Analysis**:
   - Uses `linregress` crate to fit a linear regression model.
   - Outputs regression coefficients and R-squared value.

4. **Real Data Loading**:
   - Reads historical market data from a CSV file using the `csv` crate.

### **Key Functions**
#### `generate_dummy_data(num_records)`
Generates dummy data for testing purposes.

#### `load_data_from_csv(path)`
Loads real data from a CSV file into a vector of `Record`.

#### `prepare_regression_data(pnl_data, usdt_usd_rate, btc_price, eth_price)`
Prepares data for regression analysis by combining dependent and independent variables.

#### Regression Model Output:
Displays coefficients for each variable and R-squared value.

---

## **Interpreting Results**

### Example Output:
```
Regression coefficients:
Intercept: -90522.86
usdt_usd_rate: 91489.46
btc_price: -0.00145
eth_price: 0.05448
R-squared: 0.9055
```

### Key Takeaways:
1. Significant Impact of USDT/USD Rate:
   - Large coefficient indicates that even small fluctuations in USDT/USD significantly affect PnL.
2. Minimal Influence of BTC and ETH Prices:
   - Small coefficients suggest these prices have limited impact on PnL within this model.
3. High R-squared Value:
   - Indicates that the model explains most of the variance in PnL.

---

## **Recommendations**

### Hedge Currency Exposure:
- Use derivatives or spot markets to hedge against fluctuations in USDT/USD rates.
  
### Enhance Risk Management:
- Incorporate currency exposure into risk models.
  
### Further Analysis:
- Test statistical significance of coefficients (e.g., t-tests).
- Explore additional variables that may influence PnL (e.g., liquidity or volatility).

---

## **Limitations**
homoscedasticity
Data Quality

## **License**
This project is licensed under the MIT License.

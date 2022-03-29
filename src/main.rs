use chrono::{Duration, Utc};
use coingecko::CoinGeckoClient;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let days: i32 = args[1].parse().unwrap();

    let client = CoinGeckoClient::default();
    let today = Utc::now().naive_utc();
    let mut total = 0.0;

    for i in 1..(days + 1) {
        let date = today.checked_sub_signed(Duration::days(i.into())).unwrap();
        let res = client.coin_history("stargaze", date.date(), true).await;
        let price = res.unwrap().market_data.current_price.usd.unwrap();
        println!("{} - {}", date.date(), price);
        total += price;
    }

    let avg = total / f64::from(days);
    println!("{}-day moving average: {:?}", days, avg);
}

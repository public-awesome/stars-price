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
    let mut count = 0;

    for i in 1..(days + 1) {
        let date = today.checked_sub_signed(Duration::days(i.into())).unwrap();
        let res = client.coin_history("stargaze", date.date(), true).await;
        match res {
            Ok(res) => {
                let price = res.market_data.current_price.usd.unwrap();
                println!("{} - {}", date.date(), price);
                total += price;
                count += 1;
            }
            Err(e) => println!("{} - Error", e),
        }
    }

    let avg = total / f64::from(count);
    println!("{}-day moving average: {:?}", count, avg);
}

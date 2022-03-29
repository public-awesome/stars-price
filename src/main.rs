use chrono::{Duration, Utc};
use coingecko::CoinGeckoClient;

#[tokio::main]
async fn main() {
    let client = CoinGeckoClient::default();
    let today = Utc::now().naive_utc();
    let mut total = 0.0;

    for i in 1..31 {
        let date = today.checked_sub_signed(Duration::days(i)).unwrap();
        let res = client.coin_history("stargaze", date.date(), true).await;
        let price = res.unwrap().market_data.current_price.usd.unwrap();
        println!("{} - {}", date.date(), price);
        total += price;
    }

    let avg = total / 30.0;
    println!("30-day moving average: {:?}", avg);
}

use chrono::{Duration, Utc};
use coingecko::CoinGeckoClient;
use std::env;
use tokio::time::{sleep, Duration as TokioDuration};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut days: i64 = args[1].parse().unwrap();
    let offset: i8 = args
        .get(2)
        .map_or(0, |offset_value| offset_value.parse().unwrap_or(0));

    let client = CoinGeckoClient::default();
    let today = Utc::now().naive_utc();
    let mut total = 0.0;
    let mut count: i8 = 0;
    let mut date = today
        .checked_sub_signed(Duration::days(offset.into()))
        .unwrap();
    while days > 0 {
        sleep(TokioDuration::from_millis(3000)).await;
        let res = client.coin_history("stargaze", date.date(), true).await;
        match res {
            Ok(res) => {
                let price = res.market_data.current_price.usd.unwrap();
                println!("{} - {}", date.date(), price);
                total += price;
                count += 1;
                date = today
                    .checked_sub_signed(Duration::days(count.checked_add(offset).unwrap().into()))
                    .unwrap();
                days -= 1;
            }
            Err(e) => {
                println!("{} - Error", e);
            }
        }
    }

    let avg = total / f64::from(count);
    println!("{}-day moving average: {:?}", count, avg);
}

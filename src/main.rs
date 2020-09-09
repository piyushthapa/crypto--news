mod fetch;
mod news_source;

use news_source::{CoinTelegraph, NewsBtc};

#[tokio::main]
async fn main() {
    //let news = CoinTelegraph::fetch().await;
    let btc_news = NewsBtc::fetch().await;
    // println!("{}", serde_json::to_string_pretty(&news).unwrap());
    println!("\n \n ");
    println!("{}", serde_json::to_string_pretty(&btc_news).unwrap());
}

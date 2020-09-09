mod fetch;
mod news_source;

use news_source::CoinTelegraph;

#[tokio::main]
async fn main() {
    let news = CoinTelegraph::fetch().await;
    println!("{}", serde_json::to_string_pretty(&news).unwrap());
}

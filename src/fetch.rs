extern crate reqwest;
pub struct FetchNews;

impl FetchNews {
    pub async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::get(url).await?.text().await?;
        Ok(resp)
    }
}

use serde::{Deserialize, Serialize};

pub trait News {
    /// methods for parsing news
    fn from_html(html: String) -> Vec<Box<Self>>;

    fn fetch() -> Vec<Box<Self>>;

    // TODO: Add other traits
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsData {
    pub title: Option<String>,
    pub image: Option<String>,
    pub content: Option<String>,
    pub url: Option<String>,
    pub tags: Vec<String>,
    pub source: String,
}

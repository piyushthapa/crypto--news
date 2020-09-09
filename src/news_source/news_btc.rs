use super::news::NewsData;
use crate::fetch::FetchNews;
use select::document::Document;
use select::predicate::{Attr, Class, Name};

pub struct NewsBtc;

impl NewsBtc {
    async fn fetch_html() -> Option<String> {
        FetchNews::fetch_html("https://www.newsbtc.com/category/crypto/")
            .await
            .ok()
    }

    pub async fn fetch() -> Vec<NewsData> {
        match NewsBtc::fetch_html().await {
            Some(data) => NewsBtc::from_html(data),
            None => vec![],
        }
    }

    fn from_html(html: String) -> Vec<NewsData> {
        let document = Document::from(html.as_ref());
        let mut news_vec: Vec<NewsData> = Vec::new();

        for node in document.find(Class("jeg_post")) {
            let mut title: Option<String> = None;
            let mut url: Option<String> = None;
            let mut image: Option<String> = None;
            let mut content: Option<String> = None;

            if let Some(el) = node.find(Class("jeg_post_title")).next() {
                title = Some(el.text().trim().to_string());
            }

            if let Some(el) = node.find(Name("a")).next() {
                if let Some(href) = el.attr("href") {
                    url = Some(href.to_string())
                }
            }

            if let Some(el) = node.find(Class("jeg_post_excerpt")).next() {
                content = Some(el.text().trim().to_string());
            }

            if let Some(el) = node.find(Name("img")).next() {
                if let Some(img) = el.attr("data-src") {
                    image = Some(img.to_string())
                }
            }

            if let Some(_) = content {
                news_vec.push(NewsData {
                    title,
                    content,
                    url,
                    image,
                    source: String::from("news_btc"),
                    tags: vec![],
                })
            }
        }

        news_vec
    }
}

use super::news::NewsData;
use crate::fetch::FetchNews;
use select::document::Document;
use select::predicate::{Attr, Class};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

pub struct CoinTelegraph;

impl CoinTelegraph {
    async fn fetch_html() -> Option<String> {
        FetchNews::fetch_html("https://cointelegraph.com/category/latest-news")
            .await
            .ok()
    }

    pub async fn fetch() -> Vec<NewsData> {
        match CoinTelegraph::fetch_html().await {
            Some(data) => CoinTelegraph::from_html(data),
            None => vec![],
        }
    }

    pub fn from_html(html: String) -> Vec<NewsData> {
        let document = Document::from(html.as_ref());
        let mut metadata: Vec<CoinTelegraphMetadata> = Vec::new();

        if let Some(m) = CoinTelegraphMetadata::get_metadata(&document) {
            metadata = m;
        }

        let mut news_vec: Vec<NewsData> = Vec::new();
        for node in document.find(Class("post-card-inline")) {
            let mut name = String::from("");
            let mut title: Option<String> = None;
            let mut url: Option<String> = None;
            let mut image: Option<String> = None;
            let mut content: Option<String> = None;

            if let Some(el) = node.find(Class("post-card-inline__title-link")).next() {
                title = Some(el.text());
                name = el.text().trim().to_owned();
                if let Some(attr) = el.attr("href") {
                    url = Some(attr.to_string());
                }
            }

            if let Some(el) = node.find(Class("post-card-inline__text")).next() {
                content = Some(el.text());
            }

            if let Some(el) = CoinTelegraphMetadata::search_metadata(&metadata, name) {
                image = Some(el.image.clone());
            }

            news_vec.push(NewsData {
                title,
                content,
                url,
                image,
                source: String::from("cointelegrap"),
                tags: vec![],
            })
        }

        news_vec
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinTelegraphMetadata {
    url: String,
    name: String,
    image: String,
}

impl CoinTelegraphMetadata {
    pub fn get_metadata(document: &Document) -> Option<Vec<CoinTelegraphMetadata>> {
        if let Some(el) = document.find(Attr("data-hid", "ldjson-schema")).next() {
            let data: serde_json::Value = serde_json::from_str(el.text().as_ref()).unwrap();
            let metadata: Vec<CoinTelegraphMetadata> =
                serde_json::from_value(data["itemListElement"].clone()).unwrap();

            return Some(metadata);
        };

        None
    }

    pub fn search_metadata(
        data: &Vec<CoinTelegraphMetadata>,
        name: String,
    ) -> Option<&CoinTelegraphMetadata> {
        data.iter().find(|m| *m == &name)
    }
}

impl PartialEq<String> for CoinTelegraphMetadata {
    fn eq(&self, other: &String) -> bool {
        self.name == *other
    }
}

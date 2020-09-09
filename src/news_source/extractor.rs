use super::news::NewsData;
use select::document::{Document, Find};
use select::predicate::Predicate;
use select::predicate::{Class, Name};
use std::collections::HashMap;

pub struct Extractor {
    request_url: String,
    request_type: RequestType,
    selectors: Selectors,
    content: Option<String>,
}

impl Extractor {
    pub fn new(url: &str, req_type: &str, selectors: HashMap<&str, (&str, &str)>) -> Self {
        let request_type = match req_type {
            "JSON" => RequestType::JSON,
            _ => RequestType::HTML,
        };

        Self {
            request_url: url.to_string(),
            request_type,
            selectors: Selectors::from(selectors),
            content: None,
        }
    }

    pub async fn parse(&self) -> Option<Vec<NewsData>> {
        let container = self.selectors.container.as_ref()?;
        let mut news: Vec<NewsData> = Vec::new();
        let html = self.fetch().await?;

        let document = Document::from(html.as_ref());
        Some(news)
    }

    async fn fetch(&self) -> Option<String> {
        reqwest::get(self.request_url.as_str())
            .await
            .ok()?
            .text()
            .await
            .ok()
    }
}

struct Selectors {
    pub title: Option<(String, String)>,
    pub image: Option<(String, String)>,
    pub link: Option<(String, String)>,
    pub container: Option<(String, String)>,
    pub content: Option<(String, String)>,
}

impl Selectors {
    fn from(selector: HashMap<&str, (&str, &str)>) -> Self {
        let mut sel = Selectors::init();

        for (k, (el, el_type)) in &selector {
            match *k {
                "title" => sel.title = Some((el.to_string(), el_type.to_string())),
                "image" => sel.image = Some((el.to_string(), el_type.to_string())),
                "link" => sel.link = Some((el.to_string(), el_type.to_string())),
                "container" => sel.container = Some((el.to_string(), el_type.to_string())),
                "content" => sel.content = Some((el.to_string(), el_type.to_string())),
                _ => (),
            }
        }

        sel
    }

    fn init() -> Self {
        Self {
            title: None,
            image: None,
            link: None,
            container: None,
            content: None,
        }
    }
}

enum RequestType {
    JSON,
    HTML,
}

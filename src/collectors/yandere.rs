use crate::collector::KyaniteCollector;
use crate::item::KyaniteItem;
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct YandereCollector;

impl YandereCollector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn boxed() -> Box<dyn KyaniteCollector> {
        Box::new(Self::new())
    }
}

impl KyaniteCollector for YandereCollector {
    fn id(&self) -> &'static str {
        "yandere"
    }

    fn name(&self) -> &'static str {
        "yande.re"
    }

    fn api_base(&self) -> &'static str {
        "https://yande.re/post.json?limit=100"
    }

    fn site_base(&self) -> &'static str {
        "https://yande.re"
    }

    fn tags_argument(&self) -> &'static str {
        "tags"
    }

    fn page_argument(&self) -> &'static str {
        "page"
    }

    fn collect(&self, tags: Vec<String>) -> anyhow::Result<Vec<KyaniteItem>> {
        info!("Starting {} collector...", &self.name());
        let mut items = Vec::new();
        let mut page = 0u64;
        let mut finished = false;
        while !finished {
            debug!("Grabbing page with Reqwest GET...");
            let joined_tags = tags.clone().join("+");
            let mut resp = reqwest::get(&self.api_by_page(joined_tags, page))?;
            debug!("Reading the page body as text...");
            let body = resp.text()?;
            debug!("Deserializing posts...");
            let posts: Vec<YanderePost> = match serde_json::from_str(&body) {
                Ok(posts) => posts,
                Err(why) => {
                    debug!(
                        "Failed getting page {} of {}, gracefully ending collection: {}",
                        page,
                        self.name(),
                        why
                    );
                    Vec::new()
                }
            };
            info!(
                "Found {} {} on page {} of {}...",
                posts.len(),
                if posts.len() == 1 { "post" } else { "posts" },
                page,
                self.name()
            );
            if posts.is_empty() {
                finished = true;
                info!("Page {} is empty, stopping collection.", &page);
            } else {
                for post in posts {
                    items.push(KyaniteItem::new(
                        post.file_url,
                        tags.clone(),
                        self.id().to_owned(),
                    ));
                }
                page += 1;
            }
        }
        Ok(items)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YanderePost {
    pub file_url: String,
    pub tags: String,
    pub md5: String,
}

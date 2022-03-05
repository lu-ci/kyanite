use crate::collector::KyaniteCollector;
use crate::error::KyaniteError;
use crate::item::KyaniteItem;
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct E621Collector;

impl E621Collector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn boxed() -> Box<dyn KyaniteCollector> {
        Box::new(Self::new())
    }
}

impl KyaniteCollector for E621Collector {
    fn id(&self) -> &'static str {
        "e621"
    }

    fn name(&self) -> &'static str {
        "E621"
    }

    fn api_base(&self) -> &'static str {
        "https://e621.net/posts.json"
    }

    fn site_base(&self) -> &'static str {
        "https://e621.net"
    }

    fn tags_argument(&self) -> &'static str {
        "tags"
    }

    fn page_argument(&self) -> &'static str {
        "page"
    }

    fn collect(&self, tags: Vec<String>) -> Result<Vec<KyaniteItem>, KyaniteError> {
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
            let posts: Vec<E621Post> = match serde_json::from_str::<E621Response>(&body) {
                Ok(resp) => resp.posts,
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
                    let url = post.file.url.unwrap_or_else(|| "".to_owned());
                    if !url.is_empty() {
                        items.push(KyaniteItem::new(url, tags.clone(), self.id().to_owned()));
                    }
                }
                page += 1;
            }
        }
        Ok(items)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct E621Response {
    pub posts: Vec<E621Post>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct E621Post {
    pub file: E621PostFile,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct E621PostFile {
    pub md5: String,
    pub url: Option<String>,
}

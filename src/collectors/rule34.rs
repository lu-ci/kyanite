use crate::collector::KyaniteCollector;
use crate::item::KyaniteItem;
use serde::{Deserialize, Serialize};

use log::{debug, info};

#[derive(Clone, Debug, Default)]
pub struct Rule34Collector;

impl Rule34Collector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn boxed() -> Box<dyn KyaniteCollector> {
        Box::new(Self::new())
    }
}

impl KyaniteCollector for Rule34Collector {
    fn id(&self) -> &'static str {
        "rule34"
    }

    fn name(&self) -> &'static str {
        "Rule34"
    }

    fn api_base(&self) -> &'static str {
        "https://rule34.xxx/index.php?page=dapi&s=post&q=index"
    }

    fn site_base(&self) -> &'static str {
        "https://rule34.xxx"
    }

    fn tags_argument(&self) -> &'static str {
        "tags"
    }

    fn page_argument(&self) -> &'static str {
        "pid"
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
            let posts: Rule34Posts = match serde_xml_rs::from_str(&body) {
                Ok(posts) => posts,
                Err(why) => {
                    debug!(
                        "Failed getting page {} of {}, gracefully ending collection: {}",
                        page,
                        self.name(),
                        why
                    );
                    Rule34Posts { posts: Vec::new() }
                }
            };
            info!(
                "Found {} {} on page {} of {}...",
                posts.posts.len(),
                if posts.posts.len() == 1 {
                    "post"
                } else {
                    "posts"
                },
                page,
                self.name()
            );
            if posts.posts.is_empty() {
                finished = true;
                info!("Page {} is empty, stopping collection.", &page);
            } else {
                for post in posts.posts {
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
pub struct Rule34Posts {
    #[serde(rename = "post")]
    pub posts: Vec<Rule34Post>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule34Post {
    pub file_url: String,
    pub tags: String,
    pub md5: String,
}

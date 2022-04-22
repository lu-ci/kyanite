use crate::collector::KyaniteCollector;
use crate::item::KyaniteItem;
use serde::{Deserialize, Serialize};

use log::{debug, info};

#[derive(Clone, Debug, Default)]
pub struct GelbooruCollector;

impl GelbooruCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn boxed() -> Box<dyn KyaniteCollector> {
        Box::new(Self::new())
    }
}

#[async_trait::async_trait]
impl KyaniteCollector for GelbooruCollector {
    fn id(&self) -> &'static str {
        "gelbooru"
    }

    fn name(&self) -> &'static str {
        "Gelbooru"
    }

    fn api_base(&self) -> &'static str {
        "https://gelbooru.com/index.php?page=dapi&s=post&q=index"
    }

    fn site_base(&self) -> &'static str {
        "https://gelbooru.com"
    }

    fn tags_argument(&self) -> &'static str {
        "tags"
    }

    fn page_argument(&self) -> &'static str {
        "pid"
    }

    async fn collect(&self, tags: Vec<String>) -> anyhow::Result<Vec<KyaniteItem>> {
        info!("Starting {} collector...", &self.name());
        let mut items = Vec::new();
        let mut page = 0u64;
        let mut finished = false;
        while !finished {
            debug!("Grabbing page with Reqwest GET...");
            let joined_tags = tags.clone().join("+");
            let resp = reqwest::get(&self.api_by_page(joined_tags, page)).await?;
            debug!("Reading the page body as text...");
            let body = resp.text().await?;
            debug!("Deserializing posts...");
            let posts: GelbooruPosts = match serde_xml_rs::from_str(&body) {
                Ok(posts) => posts,
                Err(why) => {
                    debug!(
                        "Failed getting page {} of {}, gracefully ending collection: {}",
                        page,
                        self.name(),
                        why
                    );
                    GelbooruPosts { posts: Vec::new() }
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
pub struct GelbooruPosts {
    #[serde(rename = "post")]
    pub posts: Vec<GelbooruPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelbooruPost {
    pub file_url: String,
    pub tags: String,
    pub md5: String,
}

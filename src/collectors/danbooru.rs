use crate::collector::KyaniteCollector;
use crate::http::KyaniteClient;
use crate::item::KyaniteItem;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct DanbooruCollector;

impl DanbooruCollector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn boxed() -> Box<dyn KyaniteCollector> {
        Box::new(Self::new())
    }
}

#[async_trait::async_trait]
impl KyaniteCollector for DanbooruCollector {
    fn id(&self) -> &'static str {
        "danbooru"
    }

    fn name(&self) -> &'static str {
        "danbooru"
    }

    fn api_base(&self) -> &'static str {
        "https://danbooru.donmai.us/posts.json?limit=1000"
    }

    fn site_base(&self) -> &'static str {
        "https://danbooru.donmai.us"
    }

    fn tags_argument(&self) -> &'static str {
        "tags"
    }

    fn page_argument(&self) -> &'static str {
        "page"
    }

    async fn collect(&self, tags: Vec<String>) -> anyhow::Result<Vec<KyaniteItem>> {
        info!("Starting {} collector...", &self.name());
        let mut items = Vec::new();
        if tags.len() <= 2 {
            let mut page = 1u64; // Starts at 1.
            let mut finished = false;
            while !finished {
                debug!("Grabbing page with Reqwest GET...");
                let joined_tags = tags.clone().join("+");
                let resp = KyaniteClient::new()
                    .client
                    .get(&self.api_by_page(joined_tags, page))
                    .send()
                    .await?;
                debug!("Reading the page body as text...");
                let body = resp.text().await?;
                debug!("Deserializing posts...");
                let posts: Vec<DanbooruPost> = match serde_json::from_str(&body) {
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
                        if post.valid() {
                            items.push(KyaniteItem::new(
                                post.file_url,
                                tags.clone(),
                                self.id().to_owned(),
                            ));
                        }
                    }
                    page += 1;
                }
            }
        } else {
            error!("Danbooru disallows more than 2 tags, so it's being skipped.")
        }
        Ok(items)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DanbooruPost {
    #[serde(default = "String::default")]
    pub file_url: String,
    #[serde(default = "String::default")]
    pub tag_string_general: String,
    #[serde(default = "String::default")]
    pub md5: String,
}

impl DanbooruPost {
    pub fn valid(&self) -> bool {
        !(self.file_url.is_empty() || self.md5.is_empty())
    }
}

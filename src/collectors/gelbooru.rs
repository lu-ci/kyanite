use crate::collector::KyaniteCollector;
use crate::item::KyaniteItem;
use serde::{Deserialize, Serialize};

use crate::error::KyaniteError;
use log::{debug, info};

#[derive(Default)]
pub struct GelbooruCollector;

impl GelbooruCollector {
    pub fn new() -> Self {
        Self::default()
    }
}

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

    fn starting_marker(&self) -> &'static str {
        "&"
    }

    fn collect(&self, tags: Vec<String>) -> Result<Vec<KyaniteItem>, KyaniteError> {
        let mut items = Vec::new();
        let mut page = 0u64;
        let mut empty = false;
        while !empty {
            info!("Scanning page {}...", &page);
            debug!("Grabbing page with Reqwest GET...");
            let mut resp = reqwest::get(&self.api_by_page(tags.clone().join("+"), page))?;
            debug!("Reading the page body as text...");
            let body = resp.text()?;
            debug!("Deserializing posts...");
            let posts: GelboruPosts = match serde_xml_rs::from_str(&body) {
                Ok(posts) => posts,
                Err(_) => GelboruPosts { posts: Vec::new() },
            };
            info!("Found {} posts on page {}...", posts.posts.len(), &page);
            if posts.posts.len() == 0 {
                empty = true;
            } else {
                for post in posts.posts {
                    items.push(KyaniteItem::new(post.file_url));
                }
                page += 1;
            }
        }
        dbg!(&items);
        Ok(items)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelboruPosts {
    #[serde(rename = "post")]
    pub posts: Vec<GelboruPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelboruPost {
    pub file_url: String,
    pub tags: String,
    pub md5: String,
}

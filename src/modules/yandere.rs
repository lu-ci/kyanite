use crate::error::Error;
use crate::collector::CollectorSlave;

#[derive(Serialize, Deserialize, Debug)]
pub struct YanderePosts {
    pub id: u32,
    pub tags: String,
    pub created_at: u32,
    pub creator_id: u32,
    pub author: String,
    pub source: String,
    pub score: i32,
    pub md5: String,
    pub file_size: u32,
    pub file_url: String,
    pub is_shown_in_index: bool,
    pub preview_url: String,
    pub preview_width: u32,
    pub preview_height: u32,
    pub sample_url: String,
    pub sample_width: u32,
    pub sample_height: u32,
    pub rating: String,
    pub status: String,
    pub width: u32,
    pub height: u32,
    pub has_children: bool
}

impl YanderePosts {
    pub fn new(content: &str) -> Result<Vec<Self>, Error> {
        let posts: Vec<Self> = serde_json::from_str(&content)?;
        Ok(posts)
    }
    pub fn collect(slave: &CollectorSlave, tags: &Vec<&str>, limit: u32) -> Result<Vec<String>, Error> {
        let mut empty_page: bool = false;
        let mut file_urls: Vec<String> = Vec::new();
        let mut current_page: u32 = 0;
        while !empty_page && (file_urls.len() as u32) < limit {
            let api_url: String = CollectorSlave::make_api_url(slave, tags, current_page);
            let page_body: String = crate::utils::get_page(&api_url)?;
            let posts: Vec<Self> = Self::new(&page_body)?;
            if !posts.is_empty() {
                println!("Found {} files on page {} of {}...", &posts.len(), &current_page, slave.domain);
                for post in &posts {
                    let owned_url: String = post.file_url.to_owned();
                    if !file_urls.contains(&owned_url) && (file_urls.len() as u32) < limit {
                        file_urls.push(owned_url);
                    }
                }
                current_page += 1;
            } else {
                println!("Page {} has nothing on it, stopping scrapper.", &current_page);
                empty_page = true;
            }
        };
        Ok(file_urls)
    }
}

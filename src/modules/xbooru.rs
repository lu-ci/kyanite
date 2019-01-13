use crate::error::Error;
use crate::collector::CollectorSlave;

#[derive(Serialize, Deserialize, Debug)]
pub struct XbooruPost {
    pub width: u32,
    pub height: u32,
    pub score: i32,
    pub file_url: String,
    #[serde(skip_deserializing)]
    pub parent_id: Option<u32>,
    pub sample_url: String,
    pub sample_width: u32,
    pub sample_height: u32,
    pub preview_url: String,
    pub rating: String,
    pub tags: String,
    pub id: u64,
    pub change: u32,
    pub md5: String,
    pub creator_id: u32,
    pub has_children: bool,
    pub created_at: String,
    pub status: String,
    pub source: String,
    pub has_notes: bool,
    pub has_comments: bool,
    pub preview_width: u32,
    pub preview_height: u32
}

impl XbooruPost {
    pub fn _new(content: &str) -> Result<Self, Error> {
        let post: Self = serde_xml_rs::from_str(&content)?;
        Ok(post)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XbooruPosts {
    #[serde(rename = "post")]
    pub posts: Vec<XbooruPost>,
    pub count: u32,
    pub offset: u32
}

impl XbooruPosts {
    pub fn new(content: &str) -> Self {
        let posts: Self = match serde_xml_rs::from_str(&content) {
            Ok(posts) => posts,
            Err(_) => {
                let dummy_posts: Vec<XbooruPost> = Vec::new();
                Self {posts: dummy_posts, count: 0, offset: 0}
            }
        };
        return posts;
    }
    pub fn collect(slave: &CollectorSlave, tags: &Vec<&str>, limit: u32) -> Result<Vec<String>, Error> {
        let mut empty_page: bool = false;
        let mut file_urls: Vec<String> = Vec::new();
        let mut current_page: u32 = 0;
        while !empty_page && (file_urls.len() as u32) < limit {
            let api_url: String = CollectorSlave::make_api_url(slave, tags, current_page);
            let page_body: String = crate::utils::get_page(&api_url)?;
            let posts: Self = Self::new(&page_body);
            if !posts.posts.is_empty() {
                println!("Found {} files on page {} of {}...", &posts.posts.len(), &current_page, slave.domain);
                for post in &posts.posts {
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

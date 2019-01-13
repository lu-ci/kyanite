use super::error::Error;
use super::modules::e621::E621Posts;
use super::modules::rule34::Rule34Posts;
use super::modules::yandere::YanderePosts;
use super::modules::konachan::KonachanPosts;
use super::modules::gelbooru::GelbooruPosts;

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectorSlave {
    pub domain: String,
    pub api_base: String,
    pub tag_arg: String,
    pub page_arg: String
}

impl CollectorSlave {
    pub fn make_api_url(slave: &Self, tags: &Vec<&str>, current_page: u32) -> String {
        format!(
            "{}&{}={}&{}={}",
            slave.api_base,
            slave.tag_arg,
            tags.join("+"),
            slave.page_arg,
            &current_page
        )
    }
    pub fn collect(&self, tags: &Vec<&str>, limit: u32) -> Result<Vec<String>, Error> {
        let mut file_urls: Vec<String> = Vec::new();
        if self.domain == "gelbooru" {
            file_urls = GelbooruPosts::collect(&self, tags, limit)?;
        } else if self.domain == "e621" {
            file_urls = E621Posts::collect(&self, tags, limit)?;
        } else if self.domain == "konachan" {
            file_urls = KonachanPosts::collect(&self, tags, limit)?;
        } else if self.domain == "rule34" {
            file_urls = Rule34Posts::collect(&self, tags, limit)?;
        } else if self.domain == "yandere" {
            file_urls = YanderePosts::collect(&self, tags, limit)?;
        };
        Ok(file_urls)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collector {
    pub slaves: Vec<CollectorSlave>
}

impl Collector {
    pub fn new() -> Result<Self, Error> {
        let slave_params: &str = include_str!("parameters/slaves.yml");
        let collector: Collector = serde_yaml::from_str(slave_params)?;
        Ok(collector)
    }
    pub fn get(&self, domain: &str) -> Option<&CollectorSlave> {
        for slave in &self.slaves {
            if slave.domain == domain {
                return Some(slave);
            }
        }
        return None;
    }
}
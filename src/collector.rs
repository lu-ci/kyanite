use crate::error::KyaniteError;
use crate::item::KyaniteItem;

pub trait KyaniteCollector {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn api_base(&self) -> &'static str;
    fn site_base(&self) -> &'static str;
    fn tags_argument(&self) -> &'static str;
    fn page_argument(&self) -> &'static str;
    fn starting_marker(&self) -> &'static str;
    fn api_by_page(&self, tags: String, page: u64) -> String {
        format!(
            "{}{}{}={}&{}={}",
            &self.api_base(),
            &self.starting_marker(),
            &self.tags_argument(),
            tags,
            &self.page_argument(),
            page
        )
    }
    fn collect(&self, tags: Vec<String>) -> Result<Vec<KyaniteItem>, KyaniteError>;
}

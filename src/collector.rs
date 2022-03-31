use log::{debug, error, info};

use crate::collectors::e621::E621Collector;
use crate::collectors::gelbooru::GelbooruCollector;
use crate::collectors::konachan::KonachanCollector;
use crate::collectors::rule34::Rule34Collector;
use crate::collectors::xbooru::XBooruCollector;
use crate::collectors::yandere::YandereCollector;
use crate::error::KyaniteError;
use crate::item::KyaniteItem;
use crate::manifest::{KyaniteManifest, KyaniteManifestItem};
use crate::params::KyaniteParams;
use crate::stats::StatsContainer;
use crate::utility::KyaniteUtility;

pub trait KyaniteCollector {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn manifest(&self) -> KyaniteManifest {
        let manifest = KyaniteManifest::new(self.id().to_owned());
        match manifest.load() {
            Ok(man) => man,
            Err(why) => {
                debug!(
                    "Failed loading {} manifest, creating a new one: {:#?}",
                    manifest.downloader, why
                );
                manifest
            }
        }
    }
    fn api_base(&self) -> &'static str;
    fn site_base(&self) -> &'static str;
    fn tags_argument(&self) -> &'static str;
    fn page_argument(&self) -> &'static str;
    fn starting_marker(&self) -> &'static str {
        if self.api_base().contains('?') {
            "&"
        } else {
            "?"
        }
    }
    fn api_by_page(&self, tags: String, page: u64) -> String {
        let api = format!(
            "{}{}{}={}&{}={}",
            &self.api_base(),
            &self.starting_marker(),
            &self.tags_argument(),
            tags,
            &self.page_argument(),
            page
        );
        debug!("{} API: {}", &self.name(), &api);
        api
    }
    fn collect(&self, tags: Vec<String>) -> Result<Vec<KyaniteItem>, KyaniteError>;
}

pub struct CollectorCore {
    stats: StatsContainer,
    params: KyaniteParams,
    collectors: Vec<Box<dyn KyaniteCollector>>,
}

impl CollectorCore {
    pub fn new(params: KyaniteParams) -> Self {
        let stats = StatsContainer::new();
        let collectors = vec![
            E621Collector::boxed(),
            GelbooruCollector::boxed(),
            KonachanCollector::boxed(),
            Rule34Collector::boxed(),
            XBooruCollector::boxed(),
            YandereCollector::boxed(),
        ];
        Self {
            stats,
            params,
            collectors,
        }
    }

    pub fn collect(&self) -> Vec<KyaniteItem> {
        info!(
            "Searching for {} on {}.",
            &self.params.tags.join(", "),
            &self.params.sources.join(", ")
        );
        let mut items = Vec::new();
        for collector in &self.collectors {
            for source in &self.params.sources {
                if source == collector.id() || source == collector.name() || source == "all" {
                    let collected = match collector.collect((&self.params.tags).to_owned()) {
                        Ok(clctd) => clctd,
                        Err(why) => {
                            error!(
                                "Failed collecting items from {}: {:#?}",
                                collector.id(),
                                why
                            );
                            Vec::new()
                        }
                    };
                    let mut manifest = collector.manifest();
                    for item in collected {
                        match item.path() {
                            Ok(path) => {
                                let manifest_item = KyaniteManifestItem::new(
                                    item.url.clone(),
                                    path,
                                    item.tags.clone(),
                                );
                                manifest.add(manifest_item);
                            }
                            Err(why) => {
                                error!(
                                    "Failed getting item path for {} manifest: {:#?}",
                                    collector.name(),
                                    why
                                );
                            }
                        }
                        items.push(item);
                    }
                    match manifest.save() {
                        Ok(_) => {}
                        Err(why) => {
                            error!("Failed saving the {} manifest: {:#?}", collector.id(), why);
                        }
                    }
                }
            }
        }
        KyaniteItem::trim(items)
    }

    pub fn get_manifest(&self, name: String) -> Option<KyaniteManifest> {
        let mut manifest = None;
        for collector in &self.collectors {
            if collector.id() == name {
                manifest = Some(collector.manifest());
                break;
            }
        }
        manifest
    }

    pub fn download(&mut self, items: Option<Vec<KyaniteItem>>) -> Result<(), KyaniteError> {
        let items = match items {
            Some(items) => items,
            None => self.collect(),
        };
        let total = items.len();
        let mut result = Ok(());
        for mut item in items {
            match self.get_manifest(item.coll.clone()) {
                Some(_manifest) => {
                    let index = item.exists().unwrap_or(None);
                    let resp = item.save(&mut self.stats, index)?;
                    info!(
                        "{} [{}] [{}] [{}/{}]: {}",
                        resp,
                        self.stats.describe(),
                        KyaniteUtility::human_size(self.stats.size, 3f64, "GiB"),
                        self.stats.count(),
                        &total,
                        item.describe()
                    );
                }
                None => {
                    result = Err(KyaniteError::from(format!(
                        "An item tried referencing and unknown manifest type: {}",
                        &item.coll
                    )));
                    break;
                }
            }
        }
        result
    }
}

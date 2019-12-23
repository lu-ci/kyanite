use crate::collectors::gelbooru::GelbooruCollector;
use crate::error::KyaniteError;
use crate::item::KyaniteItem;
use crate::manifest::{KyaniteManifest, KyaniteManifestItem};
use crate::params::KyaniteParams;
use crate::stats::StatsContainer;
use log::{debug, error, info};
use std::collections::HashMap;

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

pub struct CollectorCore {
    stats: StatsContainer,
    params: KyaniteParams,
    collectors: Vec<Box<dyn KyaniteCollector>>,
}

impl CollectorCore {
    pub fn new(params: KyaniteParams) -> Self {
        let stats = StatsContainer::new();
        let mut collectors = Vec::<Box<dyn KyaniteCollector>>::new();
        collectors.push(GelbooruCollector::boxed());
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
                    for item in collected {
                        match item.path() {
                            Ok(path) => {
                                let manifest_item = KyaniteManifestItem::new(
                                    item.url.clone(),
                                    path,
                                    self.params.clone().tags,
                                );
                                collector.manifest().add(manifest_item);
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
                }
            }
        }
        KyaniteItem::trim(items)
    }

    pub fn get_manifests(&self) -> Result<HashMap<String, KyaniteManifest>, KyaniteError> {
        let mut manifests = HashMap::new();
        for collector in &self.collectors {
            manifests.insert(collector.id().to_owned(), collector.manifest());
        }
        Ok(manifests)
    }

    pub fn save_manifests(&self) -> Result<(), KyaniteError> {
        for collector in &self.collectors {
            collector.manifest().save()?;
        }
        Ok(())
    }

    pub fn download(&mut self, items: Option<Vec<KyaniteItem>>) -> Result<(), KyaniteError> {
        let manifests = &self.get_manifests()?;
        let items = match items {
            Some(items) => items,
            None => self.collect(),
        };
        let total = items.len();
        let mut result = Ok(());
        for item in items {
            match manifests.get(&item.coll) {
                Some(manifest) => {
                    let index = item.indexed(manifest);
                    let mut copy = item.clone();
                    let resp = copy.save(&mut self.stats, index)?;
                    info!(
                        "{} [{}] [{}/{}]: {}",
                        resp,
                        self.stats.describe(),
                        &total,
                        self.stats.count(),
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

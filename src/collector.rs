use log::{debug, error, info};

use crate::collectors::e621::E621Collector;
use crate::collectors::gelbooru::GelbooruCollector;
use crate::collectors::konachan::KonachanCollector;
use crate::collectors::rule34::Rule34Collector;
use crate::collectors::xbooru::XBooruCollector;
use crate::collectors::yandere::YandereCollector;
use crate::item::KyaniteItem;
use crate::manifest::KyaniteManifest;
use crate::params::KyaniteParams;
use crate::stats::StatsContainer;
use crate::utility::KyaniteUtility;

pub trait KyaniteCollector {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
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
    fn collect(&self, tags: Vec<String>) -> anyhow::Result<Vec<KyaniteItem>>;
}

pub struct CollectorCore {
    stats: StatsContainer,
    params: KyaniteParams,
    manifest: KyaniteManifest,
    collectors: Vec<Box<dyn KyaniteCollector>>,
}

impl CollectorCore {
    pub fn new(params: KyaniteParams) -> anyhow::Result<Self> {
        let stats = StatsContainer::new();
        let collectors = vec![
            E621Collector::boxed(),
            GelbooruCollector::boxed(),
            KonachanCollector::boxed(),
            Rule34Collector::boxed(),
            XBooruCollector::boxed(),
            YandereCollector::boxed(),
        ];
        let manifest = KyaniteManifest::new()?;
        Ok(Self {
            stats,
            params,
            manifest,
            collectors,
        })
    }

    pub fn collect(&mut self) -> anyhow::Result<Vec<KyaniteItem>> {
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
                        items.push(item);
                    }
                }
            }
        }
        Ok(KyaniteItem::sort(KyaniteItem::skip(KyaniteItem::trim(
            items,
        ))?))
    }

    pub fn download(&mut self, items: Option<Vec<KyaniteItem>>) -> anyhow::Result<()> {
        let items = match items {
            Some(items) => items,
            None => self.collect()?,
        };
        let total = items.len();
        for mut item in items {
            let resp = item.save(&mut self.stats, &mut self.manifest)?;
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
        Ok(())
    }
}

use std::io::prelude::*;

use log::{debug, info};

use crate::manifest::{KyaniteManifest, KyaniteManifestItem};
use crate::stats::StatsContainer;
use crate::utility::KyaniteUtility;

#[derive(Clone, Debug)]
pub struct KyaniteItemMD5 {
    pub url: String,
    pub image: String,
}

#[derive(Clone, Debug)]
pub struct KyaniteItem {
    pub url: String,
    pub ext: String,
    pub md5: KyaniteItemMD5,
    pub data: Option<Vec<u8>>,
    pub size: f64,
    pub tags: Vec<String>,
    pub coll: String,
}

impl KyaniteItem {
    pub fn new(url: String, tags: Vec<String>, coll: String) -> Self {
        let raw_pieces = url.split('.');
        let mut pieces = Vec::<String>::new();
        for rp in raw_pieces {
            pieces.push(rp.to_owned());
        }
        let last_piece = &pieces[pieces.len() - 1];
        let last_piece_pieces = last_piece.split('?');
        let mut clean_last_piece_pieces = Vec::<String>::new();
        for lpp in last_piece_pieces {
            clean_last_piece_pieces.push(lpp.to_owned());
        }
        let clean_last_piece = clean_last_piece_pieces[0].clone();
        Self {
            url: url.to_owned(),
            ext: clean_last_piece,
            md5: KyaniteItemMD5 {
                url: format!("{:x}", md5::compute(&url)),
                image: "".to_owned(),
            },
            data: None,
            size: 0.0,
            tags,
            coll,
        }
    }

    pub fn name(&self) -> String {
        format!("{}.{}", &self.md5.clone().url, &self.ext)
    }

    pub fn download(&mut self) -> anyhow::Result<()> {
        let mut data: Vec<u8> = Vec::new();
        let mut resp = reqwest::get(&self.url)?;
        resp.copy_to(&mut data)?;
        let item_url_md5 = format!("{:x}", md5::compute(&self.url));
        let item_data_md5 = format!("{:x}", md5::compute(&data));
        self.md5 = KyaniteItemMD5 {
            url: item_url_md5,
            image: item_data_md5,
        };
        self.size = data.len() as f64;
        self.data = Some(data);
        Ok(())
    }

    pub fn describe(&self) -> String {
        format!(
            "{}.{} [{}]",
            &self.md5.clone().url,
            &self.ext,
            KyaniteUtility::human_size(self.size, 2f64, "MiB"),
        )
    }

    pub fn expunge(&mut self) {
        self.data = None;
    }

    pub fn path(&self) -> anyhow::Result<String> {
        let folder = format!(
            "downloads/{}/{}",
            &self.coll,
            slug::slugify(&self.tags.join("-"))
        );
        if !std::path::Path::new(&folder).exists() {
            std::fs::create_dir_all(&folder)?;
        }
        Ok(format!(
            "{}/{}.{}",
            folder,
            &self.md5.clone().url,
            &self.ext
        ))
    }

    pub fn indexed(&self, manifest: &KyaniteManifest) -> Option<String> {
        let mut location = None;
        for item in &manifest.files {
            if item.name == self.name() {
                location = Some(item.file.clone())
            }
        }
        location
    }

    pub fn store(&mut self, path: String) -> anyhow::Result<()> {
        self.download()?;
        let mut file = std::fs::File::create(&path)?;
        file.write_all(&self.data.clone().unwrap())?;
        file.sync_all()?;
        Ok(())
    }

    pub fn save(
        &mut self,
        stats: &mut StatsContainer,
        manifest: &mut KyaniteManifest,
    ) -> anyhow::Result<String> {
        let response: &'static str;
        let path = self.path()?;
        match self.indexed(manifest) {
            Some(idx) => {
                let source = std::path::Path::new(&idx);
                let destination = std::path::Path::new(&path);
                if source.exists() && !destination.exists() {
                    std::fs::copy(source, destination)?;
                    response = stats.add_inherited();
                } else {
                    response = stats.add_skipped();
                }
            }
            None => {
                if !std::path::Path::new(&path).exists() {
                    match &self.store(path) {
                        Ok(_) => {
                            stats.add_size(self.size);
                            response = stats.add_ok();
                            let item = KyaniteManifestItem::new(
                                self.name(),
                                self.path()?,
                                self.tags.clone(),
                            );
                            manifest.add(item);
                        }
                        Err(_) => {
                            response = stats.add_failed();
                        }
                    }
                    self.expunge();
                } else {
                    response = stats.add_skipped();
                }
            }
        }
        Ok(response.to_owned())
    }

    pub fn trim(items: Vec<Self>) -> Vec<Self> {
        debug!("Trimming vector of items, starting count: {}", items.len());
        let mut clean = Vec::<Self>::new();
        for item in &items {
            let mut exists = false;
            for ci in &clean {
                if item.url == ci.url {
                    exists = true;
                    break;
                }
            }
            if !exists {
                clean.push(item.to_owned());
            }
        }
        debug!("Item trimming complete, final count: {}", items.len());
        clean
    }

    pub fn skip(items: Vec<Self>) -> anyhow::Result<Vec<Self>> {
        let mut new = Vec::<Self>::new();
        let mut stats = StatsContainer::new();
        for item in &items {
            if !std::path::Path::new(&item.path()?).exists() {
                let _ = stats.add_skipped();
            } else {
                new.push(item.clone());
            }
        }
        if stats.skipped > 0 {
            info!("Pre-Skipped: {} items.", stats.skipped);
        }
        Ok(new)
    }

    pub fn sort(mut items: Vec<Self>) -> Vec<Self> {
        items.sort_by_key(|item| item.name());
        items
    }
}

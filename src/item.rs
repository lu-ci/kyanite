use crate::error::KyaniteError;
use crate::manifest::KyaniteManifest;
use crate::stats::StatsContainer;
use log::debug;
use std::io::prelude::*;

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

    pub fn download(&mut self) -> Result<(), KyaniteError> {
        let mut data: Vec<u8> = Vec::new();
        let mut resp = reqwest::get(&self.url)?;
        resp.copy_to(&mut data)?;
        let item_url_md5 = format!("{:x}", md5::compute(&self.url));
        let item_data_md5 = format!("{:x}", md5::compute(&data));
        self.md5 = KyaniteItemMD5 {
            url: item_url_md5,
            image: item_data_md5,
        };
        self.size = (data.len() as f64) / 1048576f64;
        self.data = Some(data);
        Ok(())
    }

    pub fn describe(&self) -> String {
        format!(
            "{}.{} [{:.2} MiB]",
            &self.md5.clone().url,
            &self.ext,
            &self.size
        )
    }

    pub fn expunge(&mut self) {
        self.data = None;
    }

    pub fn path(&self) -> Result<String, KyaniteError> {
        let folder = format!(
            "downloads/{}/{}",
            &self.coll,
            slug::slugify(&self.tags.join("_"))
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

    pub fn exists(path: String) -> bool {
        std::path::Path::new(&path).exists()
    }

    pub fn indexed(&self, manifest: &KyaniteManifest) -> Option<String> {
        let mut location = None;
        for file in &manifest.files {
            if &file.url == &self.url {
                if std::path::Path::new(&file.file).exists() {
                    location = Some(file.file.to_owned());
                }
            }
        }
        location
    }

    pub fn store(&mut self, path: String) -> Result<(), KyaniteError> {
        &self.download()?;
        let mut file = std::fs::File::create(&path)?;
        file.write_all(&self.data.clone().unwrap())?;
        file.sync_all()?;
        Ok(())
    }

    pub fn save(
        &mut self,
        stats: &mut StatsContainer,
        index: Option<String>,
    ) -> Result<String, KyaniteError> {
        let response: &'static str;
        let path = self.path()?;
        match index {
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
                if !Self::exists(path.clone()) {
                    match &self.store(path.clone()) {
                        Ok(_) => {
                            response = stats.add_ok();
                        }
                        Err(_) => {
                            response = stats.add_failed();
                        }
                    }
                    &self.expunge();
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
}

use std::io::prelude::*;

use log::debug;

use crate::error::KyaniteError;
use crate::manifest::KyaniteManifest;
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

    pub fn path(&self) -> Result<String, KyaniteError> {
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

    pub fn _indexed(&self, _manifest: &KyaniteManifest) -> Option<String> {
        let mut location = None;
        let path = self.path().unwrap_or_else(|_| "".to_owned());
        if !path.is_empty() && std::path::Path::new(&path).exists() {
            location = Some(path);
        }
        location
    }

    pub fn exists(&self) -> anyhow::Result<Option<String>> {
        let name = self.name();
        let mut location = None;
        let service_folders = std::fs::read_dir("downloads/")?;
        'sfl: for service_folder in service_folders {
            let sf = service_folder?;
            let tag_folders = std::fs::read_dir(sf.path())?;
            for tag_folder in tag_folders {
                let tf = tag_folder?;
                let files = std::fs::read_dir(tf.path())?;
                for file in files {
                    let ff = file?;
                    let fname = ff.file_name();
                    let file_name = fname.to_str().unwrap_or("");
                    if file_name == name {
                        location = Some(ff.path().to_str().unwrap_or("").to_string());
                        break 'sfl;
                    }
                }
            }
        }
        Ok(location)
    }

    pub fn store(&mut self, path: String) -> Result<(), KyaniteError> {
        self.download()?;
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
                if !std::path::Path::new(&path).exists() {
                    match &self.store(path) {
                        Ok(_) => {
                            stats.add_size(self.size);
                            response = stats.add_ok();
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
}

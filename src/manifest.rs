use serde::{Deserialize, Serialize};
use std::io::prelude::*;

use crate::error::KyaniteError;
use flate2::Compression;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KyaniteManifestItem {
    pub url: String,
    pub file: String,
    pub tags: Vec<String>,
}

impl KyaniteManifestItem {
    pub fn new(url: String, file: String, tags: Vec<String>) -> Self {
        Self { url, file, tags }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KyaniteManifest {
    pub files: Vec<KyaniteManifestItem>,
    pub downloader: String,
}

impl KyaniteManifest {
    pub fn new(downloader: String) -> Self {
        Self {
            files: Vec::new(),
            downloader,
        }
    }

    fn get_path(&self) -> Result<String, KyaniteError> {
        let folder = format!("downloads/{}", &self.downloader);
        if !std::path::Path::new(&folder).exists() {
            std::fs::create_dir_all(&folder)?;
        }
        Ok(format!("{}/manifest.json.gz", folder))
    }

    pub fn add(&mut self, item: KyaniteManifestItem) {
        self.files.push(item);
    }

    pub fn save(&self) -> Result<(), KyaniteError> {
        let path = &self.get_path()?;
        let serialized = serde_json::ser::to_string(self)?;
        let mut gz = flate2::GzBuilder::new()
            .filename(format!("Kyanite Manifest: {}", &self.downloader))
            .comment(format!("Kyanite manifest for {}", &self.downloader))
            .write(std::fs::File::create(&path)?, Compression::best());
        let mut ser_bytes: Vec<u8> = serialized.into_bytes();
        gz.write_all(ser_bytes.as_slice());
        Ok(())
    }
}

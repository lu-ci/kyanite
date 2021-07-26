use serde::{Deserialize, Serialize};

use crate::error::KyaniteError;

use log::debug;

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

    fn _get_path(&self) -> Result<String, KyaniteError> {
        let folder = format!("downloads/{}", &self.downloader);
        if !std::path::Path::new(&folder).exists() {
            debug!(
                "Manifest folder for {} doesn't exist, creating it.",
                &self.downloader
            );
            std::fs::create_dir_all(&folder)?;
        }
        Ok(format!("{}/manifest.json.gz", folder))
    }

    pub fn add(&mut self, _item: KyaniteManifestItem) -> Self {
        self.clone()
    }

    pub fn load(&self) -> Result<Self, KyaniteError> {
        Ok(self.to_owned())
    }

    pub fn save(&self) -> Result<(), KyaniteError> {
        Ok(())
    }
}

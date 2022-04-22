use log::info;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KyaniteManifestItem {
    pub name: String,
    pub file: String,
    pub tags: Vec<String>,
}

impl KyaniteManifestItem {
    pub fn new(name: String, file: String, tags: Vec<String>) -> Self {
        Self { name, file, tags }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KyaniteManifest {
    pub files: Vec<KyaniteManifestItem>,
}

impl KyaniteManifest {
    pub fn new() -> anyhow::Result<Self> {
        let mut man = Self { files: Vec::new() };
        info!("Loading manifest...");
        man.load()?;
        info!("Loaded {} manifest items.", man.files.len());
        Ok(man)
    }

    pub fn add(&mut self, item: KyaniteManifestItem) {
        self.files.push(item);
    }

    fn load(&mut self) -> anyhow::Result<()> {
        if std::path::Path::new("downloads/").exists() {
            let service_folders = std::fs::read_dir("downloads/")?;
            for service_folder in service_folders {
                let sf = service_folder?;
                let tag_folders = std::fs::read_dir(sf.path())?;
                for tag_folder in tag_folders {
                    let tf = tag_folder?;
                    let files = std::fs::read_dir(tf.path())?;
                    for file in files {
                        let ff = file?;
                        let fname = ff.file_name();
                        let file_name = fname.to_str().unwrap_or("");
                        let file_path = ff.path().to_str().unwrap_or("").to_string();
                        let manifest_item =
                            KyaniteManifestItem::new(file_name.to_string(), file_path, vec![]);
                        self.add(manifest_item);
                    }
                }
            }
        }
        Ok(())
    }
}

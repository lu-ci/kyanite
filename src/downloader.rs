use std::io::prelude::*;
use crate::error::Error;

pub struct DownloadStatistics {
    ok: u32,
    nok: u32,
    skip: u32,
    total: u32,
    current: u32
}

impl DownloadStatistics {
    pub fn new(total: u32) -> Self {
        DownloadStatistics {ok: 0, nok: 0, skip: 0, total, current: 0}
    }
    pub fn add_ok(&mut self, content: &str) {
        self.add_current("DONE", content);
        self.ok += 1;
    }
    pub fn add_nok(&mut self, content: &str) {
        self.add_current("FAIL", content);
        self.nok += 1;
    }
    pub fn add_skip(&mut self, content: &str) {
        self.add_current("SKIP", content);
        self.skip += 1;
    }
    fn add_current(&mut self, event: &str, content: &str) {
        self.current += 1;
        self.log_event(event, content);
    }
    fn log_event(&self, event: &str, content: &str) {
        println!(
            "{} | {}/{} | [O: {} | X: {} | S: {}] | {} ",
            event,
            &self.current,
            &self.total,
            &self.ok,
            &self.nok,
            &self.skip,
            content
        );
    }
}

pub struct Downloader {
    urls: Vec<String>,
    path_base: String
}

impl Downloader {
    pub fn new(items: Vec<String>) -> Self {
        let downloader: Self = Downloader {urls: items, path_base: "".to_owned()};
        return downloader;
    }
    pub fn set_path(&mut self, path: String) {
        self.path_base = path;
    }

    fn download_single(&self, url: &str, path: &str) -> Result<(), Error> {
        std::fs::create_dir_all(&self.path_base)?;
        let file_bytes: Vec<u8> = crate::utils::get_page_bytes(&url)?;
        let mut new_file: std::fs::File = std::fs::File::create(path)?;
        new_file.write_all(&file_bytes)?;
        new_file.sync_all()?;
        Ok(())
    }

    pub fn download(&mut self) {
        let mut stats: DownloadStatistics = DownloadStatistics::new(self.urls.len() as u32);
        for url in &self.urls{
            let file_ext: String = crate::utils::get_file_ext(&url);
            let file_md5: String = crate::utils::hash_string(&url);
            let file_name: String = format!("{}.{}", &file_md5, &file_ext);
            let file_path: String = format!("{}/{}", &self.path_base, &file_name);
            if !std::path::Path::new(&file_path).exists() {
                match self.download_single(&url, &file_path) {
                    Ok(_) => {
                        stats.add_ok(&file_name);
                    },
                    Err(_) => {
                        stats.add_nok(&file_name);
                    }
                }
            } else {
                stats.add_skip(&file_name);
            }
        }
    }
}
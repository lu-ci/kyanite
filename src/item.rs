use crate::error::KyaniteError;
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
    pub md5: Option<KyaniteItemMD5>,
    pub data: Option<Vec<u8>>,
}

impl KyaniteItem {
    pub fn new(url: String) -> Self {
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
            md5: None,
            data: None,
        }
    }

    pub fn download(&mut self) -> Result<(), KyaniteError> {
        let mut data: Vec<u8> = Vec::new();
        let mut resp = reqwest::get(&self.url)?;
        resp.copy_to(&mut data)?;
        let item_url_md5 = format!("{:x}", md5::compute(&self.url));
        let item_data_md5 = format!("{:x}", md5::compute(&data));
        self.md5 = Some(KyaniteItemMD5 {
            url: item_url_md5,
            image: item_data_md5,
        });
        self.data = Some(data);
        Ok(())
    }

    pub fn expunge(&mut self) {
        self.data = None;
    }

    pub fn save(&mut self, dlr: &'static str, tags: &'static str) -> Result<String, KyaniteError> {
        &self.download()?;
        let folder = format!("downloads/{}/{}", dlr, tags);
        if !std::path::Path::new(&folder).exists() {
            std::fs::create_dir_all(&folder)?;
        }
        let path = format!(
            "{}/{}.{}",
            folder,
            &self.md5.clone().unwrap().image,
            &self.ext
        );
        let mut file: std::fs::File = std::fs::File::create(&path)?;
        file.write_all(&self.data.clone().unwrap())?;
        file.sync_all()?;
        &self.expunge();
        Ok(path)
    }
}

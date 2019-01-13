use super::error::Error;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn get_page(url: &str) -> Result<String, Error> {
    let body = reqwest::get(url)?.text()?;
    Ok(body)
}

pub fn get_page_bytes(url: &str) -> Result<Vec<u8>, Error> {
    let mut container: Vec<u8> = Vec::new();
    let mut response = reqwest::get(url)?;
    response.copy_to(&mut container)?;
    Ok(container)
}

pub fn get_file_ext(url: &str) -> String {
    let pieces: Vec<&str> = url.split(".").collect::<Vec<&str>>();
    pieces[pieces.len() - 1].to_owned()
}

pub fn hash_string(text: &str) -> String {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish().to_string()
}

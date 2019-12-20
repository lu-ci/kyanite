use crate::collector::KyaniteCollector;
use crate::collectors::gelbooru::GelbooruCollector;
use crate::error::KyaniteError;
use crate::logger::KyaniteLogger;
use crate::params::KyaniteParams;

mod collector;
mod collectors;
mod error;
mod item;
mod logger;
mod manifest;
mod params;

fn main() -> Result<(), KyaniteError> {
    let params = KyaniteParams::new()?;
    KyaniteLogger::init(params.verbose)?;
    GelbooruCollector::new().collect(vec!["rating:safe".to_owned(), "fertilization".to_owned()])?;
    Ok(())
}

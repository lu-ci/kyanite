use crate::error::KyaniteError;
use log::{debug, info};

pub struct KyaniteLogger;

impl KyaniteLogger {
    pub fn init(verbose: bool) -> Result<(), KyaniteError> {
        let log_level = if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        };
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} | {} | {}] {}",
                    record.level(),
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                    record.target(),
                    message
                ))
            })
            .level(log_level)
            .chain(std::io::stdout())
            .apply()?;
        info!("Logger initialized!");
        debug!("Logging level set to debug verbosity.");
        Ok(())
    }
}

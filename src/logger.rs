use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::{debug, info};

pub struct KyaniteLogger;

impl KyaniteLogger {
    pub fn init(verbose: bool) -> anyhow::Result<()> {
        let log_level = if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        };
        let colors = ColoredLevelConfig::new()
            .trace(Color::Magenta)
            .debug(Color::Blue)
            .info(Color::Green)
            .warn(Color::Yellow)
            .error(Color::Red);
        Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} | {} | {}] {}",
                    colors.color(record.level()),
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

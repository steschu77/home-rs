use crate::error::{Error, Result};
use crate::util::datetime::DateTime;
use log::Log;
use std::io::Write;
use std::sync::RwLock;

// ----------------------------------------------------------------------------
struct TextLogger {
    file: RwLock<std::fs::File>,
}

// ----------------------------------------------------------------------------
impl TextLogger {
    fn init(path: &std::path::Path, level: log::LevelFilter) -> Result<()> {
        let file_name = path.join("app.log");
        let logger = TextLogger {
            file: RwLock::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_name)
                    .map_err(|_| Error::Logging)?,
            ),
        };
        log::set_max_level(level);
        log::set_boxed_logger(Box::new(logger)).map_err(|_| Error::Logging)?;
        Ok(())
    }
}

// ----------------------------------------------------------------------------
impl Log for TextLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut file = self.file.write().unwrap();
            let timestamp = DateTime::now();
            writeln!(
                &mut file,
                "{} [{:5}] {}",
                timestamp,
                record.level(),
                record.args()
            )
            .unwrap();
        }
    }

    fn flush(&self) {
        let mut file = self.file.write().unwrap();
        file.flush().unwrap();
    }
}

// ----------------------------------------------------------------------------
pub fn init_logger(level: log::LevelFilter) -> Result<()> {
    let log_dir = std::path::PathBuf::from("log");
    std::fs::create_dir_all(&log_dir).map_err(|_| Error::Logging)?;
    TextLogger::init(&log_dir, level)
}

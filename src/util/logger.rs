use crate::error::{Error, Result};
use crate::util::datetime::DateTime;
use log::Log;
use std::io::Write;
use std::sync::RwLock;

// ----------------------------------------------------------------------------
struct FileLogger {
    file: RwLock<std::fs::File>,
}

// ----------------------------------------------------------------------------
impl FileLogger {
    fn init(path: &std::path::Path, level: log::LevelFilter) -> Result<()> {
        let date_time = DateTime::now().as_timestamp();
        let file_name = path.join(format!("{date_time}.log"));
        let logger = FileLogger {
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
impl Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            if let Ok(mut file) = self.file.write() {
                let timestamp = DateTime::now();
                let _ = writeln!(
                    &mut file,
                    "{timestamp} [{:5}] {}",
                    record.level(),
                    record.args()
                );
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.write() {
            let _ = file.flush();
        }
    }
}

// ----------------------------------------------------------------------------
pub fn init_logger(level: log::LevelFilter) -> Result<()> {
    let log_dir = std::path::PathBuf::from("log");
    std::fs::create_dir_all(&log_dir)?;
    FileLogger::init(&log_dir, level)
}

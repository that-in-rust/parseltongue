use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use log::{Level, LevelFilter, Metadata, Record};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

pub struct AvengersLogger {
    file: Mutex<File>,
}

impl AvengersLogger {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::create(path).context("Failed to create log file")?;
        Ok(Self {
            file: Mutex::new(file),
        })
    }
}

impl log::Log for AvengersLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                Level::Error => "red",
                Level::Warn => "yellow",
                Level::Info => "green",
                Level::Debug => "blue",
                Level::Trace => "magenta",
            };

            let avenger = match record.level() {
                Level::Error => "Iron Man",
                Level::Warn => "Captain America",
                Level::Info => "Black Widow",
                Level::Debug => "Hulk",
                Level::Trace => "Thor",
            };

            let message = format!("[{}] {} - {}: {}", 
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                avenger,
                record.level(),
                record.args()
            );

            println!("{}", message.color(color));

            if let Ok(mut file) = self.file.lock() {
                writeln!(file, "{}", message).expect("Failed to write to log file");
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            file.flush().expect("Failed to flush log file");
        }
    }
}

pub fn init_logger(path: &Path) -> Result<()> {
    let logger = AvengersLogger::new(path)?;
    log::set_boxed_logger(Box::new(logger)).context("Failed to set logger")?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}

pub struct ErrorLogger {
    file: Mutex<File>,
}

impl ErrorLogger {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::create(path).context("Failed to create error log file")?;
        Ok(Self {
            file: Mutex::new(file),
        })
    }

    pub fn log_error(&self, message: &str) -> Result<()> {
        let mut file = self.file.lock().map_err(|e| anyhow::anyhow!("Failed to lock error log file: {}", e))?;
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to error log file")?;
        Ok(())
    }
}

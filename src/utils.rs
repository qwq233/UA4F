use std::fs::File;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

const PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] {M} - {m}{n}";

#[cfg(target_os = "linux")]
const LOG_FILE_PATH: &str = "/var/log/ua4f.log";

#[cfg(target_os = "windows")]
const LOG_FILE_PATH: &str = "./log/ua4f.log";

pub fn init_logger(level: String, no_file_log: bool) {
    let log_level = match level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    if !no_file_log {
        let file = match File::create(LOG_FILE_PATH) {
            Ok(file) => file,
            Err(e) => {
                panic!("Unable to write log to log file. \n{}", e)
            }
        };
        if file.metadata().unwrap().permissions().readonly() {
            panic!("Unable to write log to log file. This file is read only.")
        }
    }

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build(LOG_FILE_PATH)
        .unwrap();

    let root = if no_file_log {
        Root::builder().appender("stdout").build(log_level)
    } else {
        Root::builder()
            .appender("logfile")
            .appender("stdout")
            .build(log_level)
    };

    let config = if no_file_log {
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(root)
            .unwrap()
    } else {
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(root)
            .unwrap()
    };

    log4rs::init_config(config).unwrap();
}

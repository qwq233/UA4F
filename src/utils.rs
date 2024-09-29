use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

const PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] {M} - {m}{n}";

#[cfg(target_os = "linux")]
pub fn init_logger(level: String, no_file_log: bool) {
    let log_level = match level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build("/var/log/ua4f.log")
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

#[cfg(target_os = "windows")]
pub fn init_logger(level: String, no_file_log: bool) {
    let log_level = match level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build("./log/ua4f.log")
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

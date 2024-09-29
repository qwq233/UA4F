use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

#[cfg(target_os = "linux")]
pub fn init_logger(level: String) {
    let log_level = match level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] - {m}{n}",
        )))
        .build();
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] - {m}{n}",
        )))
        .build("/var/log/ua4f.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", log_level))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", log_level),
        )
        .build(Root::builder().appender("stdout").build(log_level))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

#[cfg(target_os = "windows")]
pub fn init_logger(level: String) {
    let log_level = match level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] - {m}{n}",
        )))
        .build();
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] - {m}{n}",
        )))
        .build("./log/ua4f.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", log_level))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", log_level),
        )
        .build(Root::builder().appender("stdout").build(log_level))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

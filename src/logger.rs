use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::path::PathBuf;

pub fn setup_logger(log_path: PathBuf) -> Config {
    let level = log::LevelFilter::Warn;

    // build stderr logger
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    let pattern = "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} | {l} | {m}{n}";
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(log_path)
        .unwrap();

    // log trace level output to file where trace is the default level
    // and the programmatically specified level to stderr
    Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap()
}

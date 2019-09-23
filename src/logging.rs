use crate::config::CliArgs;

use log4rs::encode::json::JsonEncoder;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::encode::Encode;

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::Append;
use log4rs::config::{Appender, Config, Logger, Root};
use parking_lot::Once;

static LOG_FORMAT: &'static str =
    "{d(%Y-%m-%dT%H:%M:%S%.3f)} [{level:5.5}] {T} {M} \\({f}:{L}\\): {m}{n}";

static INIT: Once = Once::new();

pub fn init(cli: &CliArgs) {
    INIT.call_once(|| init_internal(cli));
}

fn init_internal(cli: &CliArgs) {
    let encoder: Box<dyn Encode> = if cli.json {
        Box::new(JsonEncoder::new())
    } else {
        Box::new(PatternEncoder::new(LOG_FORMAT))
    };

    let appender: Box<dyn Append> = Box::new(
        ConsoleAppender::builder()
            .encoder(encoder)
            .target(Target::Stderr)
            .build(),
    );

    let level = match cli.verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    log4rs::init_config(
        Config::builder()
            .appender(Appender::builder().build("stderr", appender))
            .logger(Logger::builder().build("slumberd", level))
            .build(Root::builder().appender("stderr").build(LevelFilter::Warn))
            .unwrap(),
    )
    .unwrap();
}

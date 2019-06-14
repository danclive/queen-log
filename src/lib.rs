use log::{self, Log, Metadata, Record, SetLoggerError, Level, LevelFilter};
use env_logger::filter::Filter;
use chrono::{Local, DateTime};

use color::{Print, Color};

pub mod color;

pub struct QueenLogger {
    filter: Filter
}

impl QueenLogger {
    fn new() -> QueenLogger {
        use env_logger::filter::Builder;
        let mut builder = Builder::new();

        if let Ok(ref filter) = std::env::var("QUEEN_LOG_LEVEL") {
            builder.parse(filter);
        }

        QueenLogger {
            filter: builder.build()
        }
    }
}

impl Log for QueenLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.filter.matches(record) {
            let (color, level) = match record.level() {
                Level::Trace => {
                    (Color::Purple, "TRACE")
                }
                Level::Debug => {
                    (Color::Blue, "DEBUG")
                }
                Level::Info => {
                    (Color::Green, "INFO")
                }
                Level::Warn => {
                    (Color::Yellow, "WARN")
                }
                Level::Error => {
                    (Color::Red, "ERROR")
                }
            };

            let time_now: DateTime<Local> = Local::now();

            println!(
                "{} {} {} {} {} {} {}",
                Print::new(format!("{}: ", level)).foreground(color),
                Print::new(format!("{}", record.target())).foreground(color),
                Print::new(time_now.format("%Y/%m/%d - %H:%M:%S %z").to_string()).foreground(color),
                Print::new("| {").foreground(color),
                Print::new(format!("{}", record.args())).foreground(color),
                Print::new("} |").foreground(color),
                Print::new(format!("{}:{}", record.file().unwrap_or("unknow"), record.line().unwrap_or(0))).foreground(color)
            );
        }
    }

    fn flush(&self) {}
}

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(QueenLogger::new()))
        .map(|()| log::set_max_level(level))
}

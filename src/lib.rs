use std::io::{Write, Stdout, stdout};
use std::time::SystemTime;

use log::{self, Log, Metadata, Record, SetLoggerError, Level, LevelFilter};
use filter::Filter;
use humantime;

use color::{Print, Color};

pub mod color;
pub mod filter;

pub struct QueenLogger<P> {
    filter: Filter,
    log_print: P,
    show_color: bool
}

pub trait LogPrint {
    fn println(&self, s: &impl std::fmt::Display);
}

impl LogPrint for Stdout {
    fn println(&self, s: &impl std::fmt::Display) {
        let mut handle = self.lock();
        let _ = writeln!(handle, "{}", s);
        let _ = handle.flush();
    }
}

impl<P: LogPrint> QueenLogger<P> {
    pub fn new(log_print: P, filter: Filter, show_color: bool) -> Self {
        Self {
            filter,
            log_print,
            show_color
        }
    }
}

impl Default for QueenLogger<Stdout> {
    fn default() -> Self {
        let mut builder = filter::Builder::new();

        if let Ok(ref filter) = std::env::var("LOG_LEVEL") {
            builder.parse(filter);
        }

        QueenLogger::new(stdout(), builder.build(), true)
    }
}

impl<P: LogPrint + Sync + Send> Log for QueenLogger<P> {
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

            let s = format!("[{} {} {}] {} | {}:{}",
                        humantime::format_rfc3339_millis(SystemTime::now()),
                        level,
                        record.target(),
                        record.args(),
                        record.file().unwrap_or("unknow"),
                        record.line().unwrap_or_default()
                    );

            if self.show_color {
                self.log_print.println(&Print::new(s).foreground(color));
            } else {
                self.log_print.println(&s);
            }
        }
    }

    fn flush(&self) {}
}

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(QueenLogger::default()))
        .map(|()| log::set_max_level(level))
}

pub fn init_with_logger<P: LogPrint + Sync + Send + 'static>(
    level: LevelFilter,
    logger: QueenLogger<P>
) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(level))
}

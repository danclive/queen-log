use std::io::{Write, Stdout, stdout};

use log::{self, Log, Metadata, Record, SetLoggerError, Level, LevelFilter};
use env_logger::filter::Filter;
use chrono::{Local, DateTime, SecondsFormat};

use color::{Print, Color};

pub mod color;

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
    fn new(log_print: P, show_color: bool) -> Self {
        use env_logger::filter::Builder;
        let mut builder = Builder::new();

        if let Ok(ref filter) = std::env::var("QUEEN_LOG_LEVEL") {
            builder.parse(filter);
        }

        Self {
            filter: builder.build(),
            log_print,
            show_color
        }
    }
}

impl Default for QueenLogger<Stdout> {
    fn default() -> Self {
        QueenLogger::new(stdout(), true)
    }
}

impl<W: LogPrint + Sync + Send> Log for QueenLogger<W> {
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

            let s = format!("[{} {} {}] {} | {}",
                        time_now.to_rfc3339_opts(SecondsFormat::Millis, true),
                        level,
                        record.target(),
                        record.args(),
                        record.file().unwrap_or("unknow")
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

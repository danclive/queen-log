use std::io::{Write, Stdout, stdout};

use log::{self, Log, Metadata, Record, SetLoggerError, Level, LevelFilter};
use filter::Filter;

use chrono::{Local, DateTime};

use termcolor::{Buffer, Color, ColorSpec, WriteColor};
use once_cell::sync::Lazy;

pub mod filter;

pub struct Logger<Writer> {
    filter: Filter,
    writer: Writer,
}

static IS_STDOUT: Lazy<bool> = Lazy::new(|| atty::is(atty::Stream::Stdout));
pub trait Writer {
    fn writer(&self, record: &Record);
}

impl Writer for Stdout {
    fn writer(&self, record: &Record) {
        let time_now: DateTime<Local> = Local::now();

        let s = format!("[{}] {} | {} | {} | {}:{}",
                    time_now.format("%Y/%m/%d %H:%M:%S %z").to_string(),
                    record.level(),
                    record.target(),
                    record.args(),
                    record.file().unwrap_or("unknow"),
                    record.line().unwrap_or_default()
                );

        if *IS_STDOUT {
            let mut buffer = Buffer::ansi();
            let mut color_spec = ColorSpec::new();

            match record.level() {
                Level::Trace => {
                    color_spec.set_fg(Some(Color::Magenta));
                },
                Level::Debug => {
                    color_spec.set_fg(Some(Color::Blue));
                },
                Level::Info => {
                    color_spec.set_fg(Some(Color::Green));
                },
                Level::Warn => {
                    color_spec.set_fg(Some(Color::Yellow));
                },
                Level::Error => {
                    color_spec.set_fg(Some(Color::Red));
                }
            }

            color_spec.set_bold(true);
            let _ = buffer.set_color(&color_spec);
            let _ = buffer.write_all(s.as_bytes());

            let mut handle = self.lock();
            let _ = handle.write_all(buffer.as_slice());
            let _ = handle.write_all(b"\n");
            let _ = handle.flush();
        } else {
            let mut handle = self.lock();
            let _ = handle.write_all(s.as_bytes());
            let _ = handle.write_all(b"\n");
            let _ = handle.flush();
        }
    }
}

impl<W: Writer> Logger<W> {
    pub fn new(filter: Filter, writer: W) -> Self {
        Self {
            filter,
            writer
        }
    }
}

impl Default for Logger<Stdout> {
    fn default() -> Self {
        let mut builder = filter::Builder::new();

        if let Ok(ref filter) = std::env::var("LOG_LEVEL") {
            builder.parse(filter);
        }

        Logger::new( builder.build(), stdout())
    }
}

impl<P: Writer + Sync + Send> Log for Logger<P> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.filter.matches(record) {
            self.writer.writer(record);
        }
    }

    fn flush(&self) {}
}

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(Logger::default()))
        .map(|()| log::set_max_level(level))
}

pub fn init_with_logger<P: Writer + Sync + Send + 'static>(
    level: LevelFilter,
    logger: Logger<P>
) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(level))
}

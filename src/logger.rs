use std::thread;
use std::sync::{Once, ONCE_INIT};
use std::sync::Arc;
use std::cmp;
use std::fmt;

use queue::MessagesQueue;

lazy_static! {
    pub static ref LOG: Log = Log::init();
}

pub static mut MAX_LEVEL: Level = Level::Info;
static mut LOGGER: &'static Logger = &DefaultLogger;
static START: Once = ONCE_INIT;

static LOG_LEVEL_NAMES: [&'static str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

pub struct Log {
    pub queue: Arc<MessagesQueue<Record>>
}

impl Log {
    pub fn init() -> Log {
        let queue = MessagesQueue::with_capacity(1024);

        let queue2 = queue.clone();
        thread::spawn(move || {
            loop {
                let recored = queue2.pop();
                unsafe {LOGGER.log(&recored)};
            }
        });

        Log {
            queue: queue
        }
    }

    pub fn push(&self, record: Record) {
        self.queue.push(record);
    }
}

pub fn init(level: Level, logger: &'static Logger) {
    unsafe {
        START.call_once(move || {
            MAX_LEVEL = level;
            LOGGER = logger;
        });
    }
}

#[repr(usize)]
#[derive(Copy, Eq, Debug, Hash)]
pub enum Level {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Clone for Level {
    #[inline]
    fn clone(&self) -> Level {
        *self
    }
}

impl PartialEq for Level {
    #[inline]
    fn eq(&self, other: &Level) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd for Level {
    #[inline]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    #[inline]
    fn cmp(&self, other: &Level) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", LOG_LEVEL_NAMES[*self as usize])
    }
}

pub trait Logger {
    fn log(&self, record: &Record);
}

#[derive(Debug, Clone)]
pub struct Record {
    pub metadata: Metadata,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>
}

impl Record {
    pub fn new(metadata: Metadata, message: String, module_path: Option<String>, file: Option<String>, line: Option<u32>) -> Record {
        Record {
            metadata: metadata,
            message: message,
            module_path: module_path,
            file: file,
            line: line
        }
    }

    pub fn empty() -> Record {
        Record::new(Metadata::empty(), "".to_owned(), None, None, None)
    }

    pub fn metadata(&mut self, metadata: Metadata) -> &mut Record {
        self.metadata = metadata;
        self
    }

    pub fn message(&mut self, message: String) -> &mut Record {
        self.message = message;
        self
    }

    pub fn module_path(&mut self, module_path: Option<String>) -> &mut Record {
        self.module_path = module_path;
        self
    }

    pub fn file(&mut self, file: Option<String>) -> &mut Record {
        self.file = file;
        self
    }

    pub fn line(&mut self, line: Option<u32>) -> &mut Record {
        self.line = line;
        self
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    pub level: Level,
    pub target: String
}

impl Metadata {
    pub fn new(level: Level, target: String) -> Metadata {
        Metadata {
            level: level,
            target: target
        }
    }

    pub fn empty() -> Metadata {
        Metadata::new(Level::Info, "".to_owned())
    }

    pub fn level(&mut self, level: Level) -> &mut Metadata {
        self.level = level;
        self
    }

    pub fn target(&mut self, target: String) -> &mut Metadata {
        self.target = target;
        self
    }
}

pub struct DefaultLogger;

use color::{Print, Color};
use chrono::{Local, DateTime};

impl Logger for DefaultLogger {
    fn log(&self, record: &Record) {
        
        let color = match record.metadata.level {
            Level::Trace => {
                Color::Purple
            }
            Level::Debug => {
                Color::Blue
            }
            Level::Info => {
                Color::Green
            }
            Level::Warn => {
                Color::Yellow
            }
            Level::Error => {
                Color::Red
            }
        };

        let time_now: DateTime<Local> = Local::now();

        println!(
            "{} {} | {} | {}",
            Print::new(format!("[{}]", record.metadata.target)).foreground(color),
            Print::new(time_now.format("%Y/%m/%d - %H:%M:%S %z").to_string()).foreground(color),
            Print::new(&record.message).foreground(color),
            Print::new(format!("module:{:?} file:{:?} line:{:?}", &record.module_path, &record.file, &record.line)).foreground(color)
        );
    }
}

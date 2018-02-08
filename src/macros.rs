#[macro_export]
macro_rules! LOG {
    (target: $target:expr, $level:expr, $($arg:tt)+) => ({
        let level = $level;
        if level <= unsafe { $crate::MAX_LEVEL } {
            let metadata = $crate::Metadata::new(level, $target.to_owned());
            let record = $crate::Record::new(
                metadata,
                format!($($arg)+),
                Some(module_path!().to_string()),
                Some(file!().to_string()),
                Some(line!())

            );
            $crate::LOG.push(record);
        }
    });
    ($level:expr, $($arg:tt)+) => (LOG!(target: module_path!().to_string(), $level, $($arg)+))
}

#[macro_export]
macro_rules! ERROR {
    (target: $target:expr, $($arg:tt)*) => (
        LOG!(target: $target, $crate::Level::Error, $($arg)+);
    );
    ($($arg:tt)*) => (
        LOG!($crate::Level::Error, $($arg)+);
    )
}

#[macro_export]
macro_rules! WARN {
    (target: $target:expr, $($arg:tt)*) => (
        LOG!(target: $target, $crate::Level::Warn, $($arg)+);
    );
    ($($arg:tt)*) => (
        LOG!($crate::Level::Warn, $($arg)+);
    )
}

#[macro_export]
macro_rules! INFO {
    (target: $target:expr, $($arg:tt)*) => (
        LOG!(target: $target, $crate::Level::Info, $($arg)+);
    );
    ($($arg:tt)*) => (
        LOG!($crate::Level::Info, $($arg)+);
    )
}

#[macro_export]
macro_rules! DEBUG {
    (target: $target:expr, $($arg:tt)*) => (
        LOG!(target: $target, $crate::Level::Debug, $($arg)+);
    );
    ($($arg:tt)*) => (
        LOG!($crate::Level::Debug, $($arg)+);
    )
}

#[macro_export]
macro_rules! TRACE {
    (target: $target:expr, $($arg:tt)*) => (
        LOG!(target: $target, $crate::Level::Trace, $($arg)+);
    );
    ($($arg:tt)*) => (
        LOG!($crate::Level::Trace, $($arg)+);
    )
}

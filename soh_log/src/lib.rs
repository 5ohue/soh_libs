//-----------------------------------------------------------------------------
//! Very simple logger. It logs messages to the file and to stderr.
//-----------------------------------------------------------------------------
use anyhow::{anyhow, Result};
use std::{io::Write, ops::DerefMut};
//-----------------------------------------------------------------------------
/// The priority of a log message.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Prio {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl Prio {
    fn get_color(&self) -> &'static [u8] {
        match self {
            Self::Debug => return b"\x1b[1;35m",
            Self::Info => return b"\x1b[1;34m",
            Self::Warning => return b"\x1b[1;33m",
            Self::Error => return b"\x1b[1;31m",
            Self::Fatal => return b"\x1b[1;91m",
        }
    }
}

impl std::fmt::Display for Prio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => return f.write_str("DEBUG"),
            Self::Info => return f.write_str("INFO"),
            Self::Warning => return f.write_str("WARNING"),
            Self::Error => return f.write_str("ERROR"),
            Self::Fatal => return f.write_str("FATAL"),
        }
    }
}

//-----------------------------------------------------------------------------

/// `Logger` is a simple logger that logs messages to both the console (stderr) and a file. It can
/// be used as a local instance or using the global instance already provided in this crate.
///
/// Logger can be configured to only output messages with high enough priority.
///
/// # Examples
///
/// Creating and using the logger looks like this:
///
/// ```rust
/// use soh_log::Logger;
///
/// let logger = Logger::new(soh_log::Prio::Debug, soh_log::Prio::Info);
/// logger.log(soh_log::Prio::Info, "This is an info message");
/// ```
///
/// For convinience, you can use the global logger instance:
///
/// ```rust
/// use soh_log::*;
///
/// log_info!("This is an info message");
/// log_error!("You can use the `{}` syntax here!", "println!"); // Prints: You can use the `println!` syntax here!
///
/// ```
///
/// # Macros
///
/// The following macros are provided to simplify logging with the global logger:
///
/// - `open_logfile!($filepath)`: Opens a log file at the specified file path. (By default no file
///     is opened and messages are only logged to stderr).
/// - `set_min_priority_stderr!($priority)`: Sets the minimum priority that should be logged to
///     stderr.
/// - `set_min_priority_file!($priority)`: Sets the minimum priority that should be logged
///     to the file.
///
/// - `log_fatal!(...)`: Logs a fatal message.
/// - `log_error!(...)`: Logs an error message.
/// - `log_warning!(...)`: Logs a warning message.
/// - `log_info!(...)`: Logs an info message.
/// - `log_debug!(...)`: Logs a debug message (available only in a debug build).
///
/// The `log_X` macros use the `println!` style of arguments, which means they support formatted
/// strings.
pub struct Logger {
    file: std::sync::Mutex<Option<std::fs::File>>,
    min_priority_stderr: std::sync::RwLock<Prio>,
    min_priority_file: std::sync::RwLock<Prio>,
}

impl Logger {
    /// Creates a new logger with the specified minimum priority levels.
    ///
    /// By default no file is opened and messages are only logged to stderr. To save to file
    /// you need to manually call [Logger::open_logfile]
    pub const fn new(min_priority_stderr: Prio, min_priority_file: Prio) -> Logger {
        return Logger {
            file: std::sync::Mutex::new(None),
            min_priority_stderr: std::sync::RwLock::new(min_priority_stderr),
            min_priority_file: std::sync::RwLock::new(min_priority_file),
        };
    }

    /// Loads the file at the specified path and opens it for logging.
    pub fn open_logfile(&self, filename: &str) -> Result<()> {
        let file = std::fs::File::create(filename)?;

        let Ok(mut lock) = self.file.lock() else {
            return Err(anyhow!("Failed to acquire lock for the file"));
        };
        *lock = Some(file);

        return Ok(());
    }

    pub fn log(&self, priority: Prio, msg: &str) {
        self.log_stderr(priority, msg);
        self.log_file(priority, msg);
    }

    /// Sets the minimum priority that should be logged to stderr.
    pub fn set_min_priority_stderr(&self, min_priority_stderr: Prio) -> Result<()> {
        let Ok(mut p) = self.min_priority_stderr.write() else {
            return Err(anyhow!("Cannot get write lock for logger"));
        };

        *p = min_priority_stderr;
        return Ok(());
    }

    /// Sets the minimum priority that should be logged to the file.
    pub fn set_min_priority_file(&self, min_priority_file: Prio) -> Result<()> {
        let Ok(mut p) = self.min_priority_file.write() else {
            return Err(anyhow!("Cannot get write lock for logger"));
        };

        *p = min_priority_file;
        return Ok(());
    }

    fn log_stderr(&self, priority: Prio, msg: &str) {
        if priority < *self.min_priority_stderr.read().unwrap() {
            return;
        }

        let mut stderr = std::io::stderr();
        let _ = stderr.write(priority.get_color());
        let _ = stderr.write(b"[");
        let _ = stderr.write(priority.to_string().as_bytes());
        let _ = stderr.write(b"] ");
        let _ = stderr.write(b"\x1b[0m");
        let _ = stderr.write(msg.as_bytes());
        let _ = stderr.write(b"\n");
        let _ = stderr.flush();
    }

    fn log_file(&self, priority: Prio, msg: &str) {
        if priority < *self.min_priority_file.read().unwrap() {
            return;
        }

        let Ok(mut lock) = self.file.lock() else {
            return;
        };

        if let Some(file) = lock.deref_mut() {
            let _ = writeln!(file, "[{priority}] {msg}");
        }
    }
}

//-----------------------------------------------------------------------------
/// Trait which adds the `expect_log` and `unwrap_log` methods
///
/// Those methods use the global logger instance.
pub trait LogError {
    type Output;

    fn expect_log(self, msg: &str) -> Self::Output;
    fn unwrap_log(self) -> Self::Output;
}

impl<T, E> LogError for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Output = T;

    fn expect_log(self, msg: &str) -> Self::Output {
        match self {
            Ok(val) => return val,
            Err(error) => {
                log_fatal!("{msg}: {error:?}");
                panic!();
            }
        };
    }
    fn unwrap_log(self) -> Self::Output {
        match self {
            Ok(val) => return val,
            Err(error) => {
                let msg = format!("called `unwrap_log()` on an `Err` value: {error:?}");
                log_fatal!("{msg}");
                panic!();
            }
        }
    }
}

impl <T> LogError for Option<T>
{
    type Output = T;

    fn expect_log(self, msg: &str) -> Self::Output {
        match self {
            Some(val) => return val,
            None => {
                log_fatal!("{msg}");
                panic!();
            }
        };
    }
    fn unwrap_log(self) -> Self::Output {
        match self {
            Some(val) => return val,
            None => {
                let msg = format!("called `unwrap_log()` on an `None` value");
                log_fatal!("{msg}");
                panic!();
            }
        }
    }
}

//-----------------------------------------------------------------------------
/// Global instance
pub static LOGGER: Logger = Logger::new(Prio::Debug, Prio::Info);
//-----------------------------------------------------------------------------
// Macros for the global instance

/// Opens a log file at the specified file path.
#[macro_export]
macro_rules! open_logfile {
    ($filepath:expr) => {
        $crate::LOGGER.open_logfile($filepath)
    };
}

/// Sets the minimum priority that should be logged to stderr.
#[macro_export]
macro_rules! set_min_priority_stderr {
    ($priority:expr) => {
        $crate::LOGGER.set_min_priority_stderr($priority)
    };
}

/// Sets the minimum priority that should be logged to the file.
#[macro_export]
macro_rules! set_min_priority_file {
    ($priority:expr) => {
        $crate::LOGGER.set_min_priority_file($priority)
    };
}

/// Logs a message with the specified priority.
#[macro_export]
macro_rules! log_prio {
    ($priority:expr, $($args:tt)*) => {
        let msg = std::fmt::format(format_args!($($args)*));
        $crate::LOGGER.log($priority, &msg);
    };
}

/// Logs a fatal error message.
#[macro_export]
macro_rules! log_fatal {
    ($($args:tt)*) => {
        let msg = std::fmt::format(format_args!($($args)*));
        $crate::LOGGER.log($crate::Prio::Fatal, &msg);
    };
}

/// Logs an error message.
#[macro_export]
macro_rules! log_error {
    ($($args:tt)*) => {
        let msg = std::fmt::format(format_args!($($args)*));
        $crate::LOGGER.log($crate::Prio::Error, &msg);
    };
}

/// Logs a warning message.
#[macro_export]
macro_rules! log_warning {
    ($($args:tt)*) => {
        let msg = std::fmt::format(format_args!($($args)*));
        $crate::LOGGER.log($crate::Prio::Warning, &msg);
    };
}

/// Logs an info message.
#[macro_export]
macro_rules! log_info {
    ($($args:tt)*) => {
        let msg = std::fmt::format(format_args!($($args)*));
        $crate::LOGGER.log($crate::Prio::Info, &msg);
    };
}

/// Logs a debug message.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log_debug {
    ($($args:tt)*) => {
        let msg = std::fmt::format(format_args!($($args)*));
        $crate::LOGGER.log($crate::Prio::Debug, &msg);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log_debug {
    ($($args:tt)*) => {};
}

//-----------------------------------------------------------------------------

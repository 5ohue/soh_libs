//-----------------------------------------------------------------------------
//! Very simple logger. It logs messages to the file and to stderr.
//-----------------------------------------------------------------------------
use anyhow::{anyhow, Result};
use std::ops::DerefMut;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Priority {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fatal => return f.write_str("FATAL"),
            Self::Error => return f.write_str("ERROR"),
            Self::Warning => return f.write_str("WARNING"),
            Self::Info => return f.write_str("INFO"),
            Self::Debug => return f.write_str("DEBUG"),
        }
    }
}

//-----------------------------------------------------------------------------
pub static LOGGER: Logger = Logger::new(Priority::Debug, Priority::Info);
//-----------------------------------------------------------------------------

pub struct Logger {
    file: std::sync::Mutex<Option<std::fs::File>>,
    max_priority_stderr: std::sync::RwLock<Priority>,
    max_priority_file: std::sync::RwLock<Priority>,
}

impl Logger {
    pub const fn new(max_priority_stderr: Priority, max_priority_file: Priority) -> Logger {
        return Logger {
            file: std::sync::Mutex::new(None),
            max_priority_stderr: std::sync::RwLock::new(max_priority_stderr),
            max_priority_file: std::sync::RwLock::new(max_priority_file),
        };
    }

    pub fn open_logfile(&self, filename: &str) -> Result<()> {
        let file = std::fs::File::create(filename)?;

        let Ok(mut lock) = self.file.lock() else {
            return Err(anyhow!("Failed to acquire lock for the file"));
        };
        *lock = Some(file);

        return Ok(());
    }

    pub fn log(&self, priority: Priority, msg: &str) -> Result<()> {
        use std::io::Write;

        // Write to stderr
        let mut stderr = std::io::stderr();
        writeln!(stderr, "[{priority}] {msg}")?;

        // Write to file
        let Ok(mut lock) = self.file.lock() else {
            return Err(anyhow!("Failed to acquire lock for the file"));
        };

        if let Some(file) = lock.deref_mut() {
            writeln!(file, "[{priority}] {msg}")?;
        }

        return Ok(());
    }

    pub fn set_max_priority_stderr(&self, max_priority_stderr: Priority) -> Result<()> {
        let Ok(mut p) = self.max_priority_stderr.write() else {
            return Err(anyhow!("Cannot get write lock for logger"));
        };

        *p = max_priority_stderr;
        return Ok(());
    }

    pub fn set_max_priority_file(&self, max_priority_file: Priority) -> Result<()> {
        let Ok(mut p) = self.max_priority_file.write() else {
            return Err(anyhow!("Cannot get write lock for logger"));
        };

        *p = max_priority_file;
        return Ok(());
    }
}

//-----------------------------------------------------------------------------

use std::thread;

use crate::CircularBuffer;
use crate::TuiLoggerFile;
use log::LevelFilter;
use log::Record;
use log::SetLoggerError;

use crate::TUI_LOGGER;

// Lots of boilerplate code, so that init_logger can return two error types...
#[derive(Debug)]
pub enum TuiLoggerError {
    SetLoggerError(SetLoggerError),
    ThreadError(std::io::Error),
}
impl std::error::Error for TuiLoggerError {
    fn description(&self) -> &str {
        match self {
            TuiLoggerError::SetLoggerError(_) => "SetLoggerError",
            TuiLoggerError::ThreadError(_) => "ThreadError",
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            TuiLoggerError::SetLoggerError(_) => None,
            TuiLoggerError::ThreadError(err) => Some(err),
        }
    }
}
impl std::fmt::Display for TuiLoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TuiLoggerError::SetLoggerError(err) => write!(f, "SetLoggerError({})", err),
            TuiLoggerError::ThreadError(err) => write!(f, "ThreadError({})", err),
        }
    }
}

/// Init the logger.
pub fn init_logger(max_level: LevelFilter) -> Result<(), TuiLoggerError> {
    let join_handle = thread::Builder::new()
        .name("tui-logger::move_events".into())
        .spawn(|| {
            let duration = std::time::Duration::from_millis(10);
            loop {
                thread::park_timeout(duration);
                TUI_LOGGER.move_events();
            }
        })
        .map_err(|err| TuiLoggerError::ThreadError(err))?;
    TUI_LOGGER.hot_log.lock().mover_thread = Some(join_handle);
    if cfg!(feature = "tracing-support") {
        set_default_level(max_level);
        Ok(())
    } else {
        log::set_max_level(max_level);
        log::set_logger(&*TUI_LOGGER).map_err(|err| TuiLoggerError::SetLoggerError(err))
    }
}

/// Set the depth of the hot buffer in order to avoid message loss.
/// This is effective only after a call to move_events()
pub fn set_hot_buffer_depth(depth: usize) {
    TUI_LOGGER.inner.lock().hot_depth = depth;
}

/// Set the depth of the circular buffer in order to avoid message loss.
/// This will delete all existing messages in the circular buffer.
pub fn set_buffer_depth(depth: usize) {
    TUI_LOGGER.inner.lock().events = CircularBuffer::new(depth);
}

/// Define filename and log formmating options for file dumping.
pub fn set_log_file(file_options: TuiLoggerFile) {
    TUI_LOGGER.inner.lock().dump = Some(file_options);
}

/// Set default levelfilter for unknown targets of the logger
pub fn set_default_level(levelfilter: LevelFilter) {
    TUI_LOGGER.hot_select.lock().default = levelfilter;
    TUI_LOGGER.inner.lock().default = levelfilter;
}

/// Remove env filter - for debugging purposes
pub fn remove_env_filter() {
    TUI_LOGGER.hot_select.lock().filter = None;
    TUI_LOGGER.inner.lock().filter = None;
}

fn set_env_filter(filter1: env_filter::Filter, filter2: env_filter::Filter) {
    // Filter does not support Copy. In order to unnecessary lock hot_select,
    // we use a manual copy of the env filter.
    TUI_LOGGER.hot_select.lock().filter = Some(filter1);
    TUI_LOGGER.inner.lock().filter = Some(filter2);
}

/// Parse environment variable for env_filter
pub fn set_env_filter_from_string(filterstring: &str) {
    let mut builder1 = env_filter::Builder::new();
    let mut builder2 = env_filter::Builder::new();

    builder1.parse(filterstring);
    builder2.parse(filterstring);

    set_env_filter(builder1.build(), builder2.build());
}

/// Parse environment variable for env_filter
pub fn set_env_filter_from_env(env_name: Option<&str>) {
    let mut builder1 = env_filter::Builder::new();
    let mut builder2 = env_filter::Builder::new();

    // Parse a directives string from an environment variable
    if let Ok(ref filter) = std::env::var(env_name.unwrap_or("RUST_LOG")) {
        builder1.parse(filter);
        builder2.parse(filter);

        set_env_filter(builder1.build(), builder2.build());
    }
}

/// Set levelfilter for a specific target in the logger
pub fn set_level_for_target(target: &str, levelfilter: LevelFilter) {
    let h = fxhash::hash64(&target);
    TUI_LOGGER.inner.lock().targets.set(target, levelfilter);
    let mut hs = TUI_LOGGER.hot_select.lock();
    hs.hashtable.insert(h, levelfilter);
}

// Move events from the hot log to the main log
pub fn move_events() {
    TUI_LOGGER.move_events();
}

/// A simple `Drain` to log any event directly.
#[derive(Default)]
pub struct Drain;

impl Drain {
    /// Create a new Drain
    pub fn new() -> Self {
        Drain
    }
    /// Log the given record to the main tui-logger
    pub fn log(&self, record: &Record) {
        TUI_LOGGER.raw_log(record)
    }
}

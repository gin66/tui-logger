//! # Logger with smart widget for the `tui` and `ratatui` crate
//!
//! [![dependency status](https://deps.rs/repo/github/gin66/tui-logger/status.svg?service=github&nocache=0_9_1)](https://deps.rs/repo/github/gin66/tui-logger)
//! ![Build examples](https://github.com/gin66/tui-logger/workflows/Build%20examples/badge.svg?service=github)
//!
//!
//! ## Demo of the widget
//!
//! ![Demo](https://github.com/gin66/tui-logger/blob/master/doc/demo_v0.6.6.gif?raw=true)
//!
//! ## Documentation
//!
//! [Documentation](https://docs.rs/tui-logger/latest/tui_logger/)
//!
//! ## Important note for `tui`
//!
//! The `tui` crate has been archived and `ratatui` has taken over.
//! In order to avoid supporting compatibility for an inactive crate,
//! the v0.9.x releases are the last to support `tui`. In case future bug fixes
//! are needed, the branch `tui_legacy` has been created to track changes to 0.9.x releases.
//!
//! Starting with v0.10 `tui-logger` is `ratatui` only.
//!
//! ## Features
//!
//! - [X] Logger implementation for the `log` crate
//! - [X] Logger enable/disable detection via hash table (avoid string compare)
//! - [X] Hot logger code only copies enabled log messages with timestamp into a circular buffer
//! - [X] Widgets/move_message() retrieve captured log messages from hot circular buffer
//! - [X] Lost message detection due to circular buffer
//! - [X] Log filtering performed on log record target
//! - [X] Simple Widgets to view logs and configure debuglevel per target
//! - [X] Logging of enabled logs to file
//! - [X] Scrollback in log history
//! - [x] Title of target and log pane can be configured
//! - [X] `slog` support, providing a Drain to integrate into your `slog` infrastructure
//! - [X] `tracing` support
//! - [ ] Allow configuration of target dependent loglevel specifically for file logging
//! - [ ] Avoid duplicating of target, module and filename in every log record
//! - [ ] Simultaneous modification of all targets' display/hot logging loglevel by key command
//!
//! ## Smart Widget
//!
//! Smart widget consists of two widgets. Left is the target selector widget and
//! on the right side the logging messages view scrolling up. The target selector widget
//! can be hidden/shown during runtime via key command.
//! The key command to be provided to the TuiLoggerWidget via transition() function.
//!
//! The target selector widget looks like this:
//!
//! ![widget](https://github.com/gin66/tui-logger/blob/master/doc/example.png?raw=true)
//!
//! It controls:
//!
//! - Capturing of log messages by the logger
//! - Selection of levels for display in the logging message view
//!
//! The two columns have the following meaning:
//!
//! - Code EWIDT: E stands for Error, W for Warn, Info, Debug and Trace.
//!   + Inverted characters (EWIDT) are enabled log levels in the view
//!   + Normal characters show enabled capturing of a log level per target
//!   + If any of EWIDT are not shown, then the respective log level is not captured
//! - Target of the log events can be defined in the log e.g. `warn!(target: "demo", "Log message");`
//!
//! ## Smart Widget Key Commands
//! ```ignore
//! |  KEY     | ACTION
//! |----------|-----------------------------------------------------------|
//! | h        | Toggles target selector widget hidden/visible
//! | f        | Toggle focus on the selected target only
//! | UP       | Select previous target in target selector widget
//! | DOWN     | Select next target in target selector widget
//! | LEFT     | Reduce SHOWN (!) log messages by one level
//! | RIGHT    | Increase SHOWN (!) log messages by one level
//! | -        | Reduce CAPTURED (!) log messages by one level
//! | +        | Increase CAPTURED (!) log messages by one level
//! | PAGEUP   | Enter Page Mode and scroll approx. half page up in log history.
//! | PAGEDOWN | Only in page mode: scroll 10 events down in log history.
//! | ESCAPE   | Exit page mode and go back to scrolling mode
//! | SPACE    | Toggles hiding of targets, which have logfilter set to off
//! ```
//!
//! The mapping of key to action has to be done in the application. The respective TuiWidgetEvent
//! has to be provided to TuiWidgetState::transition().
//!
//! Remark to the page mode: The timestamp of the event at event history's bottom line is used as
//! reference. This means, changing the filters in the EWIDT/focus from the target selector window
//! should work as expected without jumps in the history. The page next/forward advances as
//! per visibility of the events.
//!
//! ## Basic usage to initialize logger-system:
//! ```rust
//! #[macro_use]
//! extern crate log;
//! //use tui_logger;
//!
//! fn main() {
//!     // Early initialization of the logger
//!
//!     // Set max_log_level to Trace
//!     tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
//!
//!     // Set default level for unknown targets to Trace
//!     tui_logger::set_default_level(log::LevelFilter::Trace);
//!
//!     // code....
//! }
//! ```
//!
//! For use of the widget please check examples/demo.rs
//!
//! ## Demo
//!
//! Run demo using termion:
//!
//! ```ignore
//! cargo run --example demo --features termion
//! ```
//!
//! Run demo with crossterm:
//!
//! ```ignore
//! cargo run --example demo --features crossterm
//! ```
//!
//! ## `slog` support
//!
//! `tui-logger` provides a [`TuiSlogDrain`] which implements `slog::Drain` and will route all records
//! it receives to the `tui-logger` widget.
//!
//! Enabled by feature "slog-support"
//!
//! ## `tracing-subscriber` support
//!
//! `tui-logger` provides a [`TuiTracingSubscriberLayer`] which implements
//! `tracing_subscriber::Layer` and will collect all events
//! it receives to the `tui-logger` widget
//!
//! Enabled by feature "tracing-support"
//!
//! ## Custom filtering
//! ```rust
//! #[macro_use]
//! extern crate log;
//! //use tui_logger;
//! use env_logger;
//!
//! fn main() {
//!     // Early initialization of the logger
//!     let drain = tui_logger::Drain::new();
//!     // instead of tui_logger::init_logger, we use `env_logger`
//!     env_logger::Builder::default()
//!         .format(move |buf, record|
//!             // patch the env-logger entry through our drain to the tui-logger
//!             Ok(drain.log(record))
//!         ).init(); // make this the global logger
//!     // code....
//! }
//! ```
//!
//! ## Internals
//!
//! For logging there are two circular buffers in use:
//! * "hot" buffer, which is written to during any logging macro invocation
//! * main buffer, which holds events to be displayed by the widgets.
//!
//! The size of the "hot" buffer is 1000 and can be modified by `set_hot_buffer_depth()`.
//! The size of the main buffer is 10000 and can be modified by `set_buffer_depth()`.
//!
//! Reason for this scheme: The main buffer is locked for a while during widget updates.
//! In order to block the log-macros, this scheme is in use.
//!
//! The copy from "hot" buffer to main buffer is performed by a call to `move_events()`,
//! which is done in a cyclic task, which repeats every 10 ms, or when the hot buffer is half full.
//!
//! ## THANKS TO
//!
//! * [Florian Dehau](https://github.com/fdehau) for his great crate [tui-rs](https://github.com/fdehau/tui-rs)
//! * [Antoine Büsch](https://github.com/abusch) for providing the patches to tui-rs v0.3.0 and v0.6.0
//! * [Adam Sypniewski](https://github.com/ajsyp) for providing the patches to tui-rs v0.6.2
//! * [James aka jklong](https://github.com/jklong) for providing the patch to tui-rs v0.7
//! * [icy-ux](https://github.com/icy-ux) for adding slog support and example
//! * [alvinhochun](https://github.com/alvinhochun) for updating to tui 0.10 and crossterm support
//! * [brooksmtownsend](https://github.com/brooksmtownsend) Patch to remove verbose timestamp info
//! * [Kibouo](https://github.com/Kibouo) Patch to change Rc/Refcell to thread-safe counterparts
//! * [Afonso Bordado](https://github.com/afonso360) for providing the patch to tui-rs v0.17
//! * [Benjamin Kampmann](https://github.com/gnunicorn) for providing patch to tui-rs v0.18
//! * [Paul Sanders](https://github.com/pms1969) for providing patch in [issue #30](https://github.com/gin66/tui-logger/issues/30)
//! * [Ákos Hadnagy](https://github.com/ahadnagy) for providing patch in [#31](https://github.com/gin66/tui-logger/issues/31) for tracing-subscriber support
//! * [Orhun Parmaksız](https://github.com/orhun) for providing patches in [#33](https://github.com/gin66/tui-logger/issues/33), [#34](https://github.com/gin66/tui-logger/issues/34), [#37](https://github.com/gin66/tui-logger/issues/37) and [#46](https://github.com/gin66/tui-logger/issues/46)
//! * [purephantom](https://github.com/purephantom) for providing patch in [#42](https://github.com/gin66/tui-logger/issues/42) for ratatui update
//! * [Badr Bouslikhin](https://github.com/badrbouslikhin) for providing patch in [#47](https://github.com/gin66/tui-logger/issues/47) for ratatui update
//! * [ganthern](https://github.com/ganthern) for providing patch in [#49](https://github.com/gin66/tui-logger/issues/49) for tui support removal
//! * [Linda_pp](https://github.com/rhysd) for providing patch in [#51](https://github.com/gin66/tui-logger/issues/51) for Cell:set_symbol
//! * [Josh McKinney](https://github.com/joshka) for providing patch in
//! [#56](https://github.com/gin66/tui-logger/issues/56) for Copy on TuiWidgetEvent and
//! TuiLoggerWidget
//! * [nick42d](https://github.com/nick42d) for providing patch in
//! [#63](https://github.com/gin66/tui-logger/issues/63) for semver checks
//! * [Tom Groenwoldt](https://github.com/tomgroenwoldt) for providing patch in [#65](https://github.com/gin66/tui-logger/issues/65) for ratatui update
//!
//!## Star History
//!
//![![Star History Chart](https://api.star-history.com/svg?repos=gin66/tui-logger&type=Date)](https://star-history.com/#gin66/tui-logger&Date)
// Enable docsrs doc_cfg - to display non-default feature documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]
#[macro_use]
extern crate lazy_static;

use std::collections::hash_map::Iter;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::io::Write;
use std::mem;
use std::sync::Arc;
use std::thread;

use chrono::{DateTime, Local};
use log::{Level, Log, Metadata, Record, SetLoggerError};
use parking_lot::Mutex;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Widget},
};
use widget::inner::TuiLoggerInner;
use widget::inner::TuiWidgetInnerState;

mod circular;
#[cfg(feature = "slog-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "slog-support")))]
mod slog;
#[cfg(feature = "tracing-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing-support")))]
mod tracing_subscriber;

pub use crate::circular::CircularBuffer;
#[cfg(feature = "slog-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "slog-support")))]
pub use crate::slog::TuiSlogDrain;
#[cfg(feature = "tracing-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing-support")))]
pub use crate::tracing_subscriber::TuiTracingSubscriberLayer;
#[doc(no_inline)]
pub use log::LevelFilter;

pub mod widget;
pub use widget::inner::TuiWidgetState;
pub use widget::smart::TuiLoggerSmartWidget;
pub use widget::standard::TuiLoggerWidget;

pub mod file;
pub use file::TuiLoggerFile;

pub struct ExtLogRecord {
    timestamp: DateTime<Local>,
    level: Level,
    target: String,
    file: String,
    line: u32,
    msg: String,
}

fn advance_levelfilter(levelfilter: LevelFilter) -> (Option<LevelFilter>, Option<LevelFilter>) {
    match levelfilter {
        LevelFilter::Trace => (None, Some(LevelFilter::Debug)),
        LevelFilter::Debug => (Some(LevelFilter::Trace), Some(LevelFilter::Info)),
        LevelFilter::Info => (Some(LevelFilter::Debug), Some(LevelFilter::Warn)),
        LevelFilter::Warn => (Some(LevelFilter::Info), Some(LevelFilter::Error)),
        LevelFilter::Error => (Some(LevelFilter::Warn), Some(LevelFilter::Off)),
        LevelFilter::Off => (Some(LevelFilter::Error), None),
    }
}

/// LevelConfig stores the relation target->LevelFilter in a hash table.
///
/// The table supports copying from the logger system LevelConfig to
/// a widget's LevelConfig. In order to detect changes, the generation
/// of the hash table is compared with any previous copied table.
/// On every change the generation is incremented.
#[derive(Default)]
pub struct LevelConfig {
    config: HashMap<String, LevelFilter>,
    generation: u64,
    origin_generation: u64,
    default_display_level: Option<LevelFilter>,
}
impl LevelConfig {
    /// Create an empty LevelConfig.
    pub fn new() -> LevelConfig {
        LevelConfig {
            config: HashMap::new(),
            generation: 0,
            origin_generation: 0,
            default_display_level: None,
        }
    }
    /// Set for a given target the LevelFilter in the table and update the generation.
    pub fn set(&mut self, target: &str, level: LevelFilter) {
        if let Some(lev) = self.config.get_mut(target) {
            if *lev != level {
                *lev = level;
                self.generation += 1;
            }
            return;
        }
        self.config.insert(target.to_string(), level);
        self.generation += 1;
    }
    /// Set default display level filter for new targets - independent from recording
    pub fn set_default_display_level(&mut self, level: LevelFilter) {
        self.default_display_level = Some(level);
    }
    /// Retrieve an iter for all the targets stored in the hash table.
    pub fn keys(&self) -> Keys<String, LevelFilter> {
        self.config.keys()
    }
    /// Get the levelfilter for a given target.
    pub fn get(&self, target: &str) -> Option<LevelFilter> {
        self.config.get(target).cloned()
    }
    /// Retrieve an iterator through all entries of the table.
    pub fn iter(&self) -> Iter<String, LevelFilter> {
        self.config.iter()
    }
    /// Merge an origin LevelConfig into this one.
    ///
    /// The origin table defines the maximum levelfilter.
    /// If this table has a higher levelfilter, then it will be reduced.
    /// Unknown targets will be copied to this table.
    fn merge(&mut self, origin: &LevelConfig) {
        if self.origin_generation != origin.generation {
            for (target, origin_levelfilter) in origin.iter() {
                if let Some(levelfilter) = self.get(target) {
                    if levelfilter <= *origin_levelfilter {
                        continue;
                    }
                }
                let levelfilter = self
                    .default_display_level
                    .map(|lvl| {
                        if lvl > *origin_levelfilter {
                            *origin_levelfilter
                        } else {
                            lvl
                        }
                    })
                    .unwrap_or(*origin_levelfilter);
                self.set(target, levelfilter);
            }
            self.generation = origin.generation;
        }
    }
}

/// These are the sub-structs for the static TUI_LOGGER struct.
struct HotSelect {
    hashtable: HashMap<u64, LevelFilter>,
    default: LevelFilter,
}
struct HotLog {
    events: CircularBuffer<ExtLogRecord>,
    mover_thread: Option<thread::JoinHandle<()>>,
}

struct TuiLogger {
    hot_select: Mutex<HotSelect>,
    hot_log: Mutex<HotLog>,
    inner: Mutex<TuiLoggerInner>,
}
impl TuiLogger {
    pub fn move_events(&self) {
        // If there are no new events, then just return
        if self.hot_log.lock().events.total_elements() == 0 {
            return;
        }
        // Exchange new event buffer with the hot buffer
        let mut received_events = {
            let hot_depth = self.inner.lock().hot_depth;
            let new_circular = CircularBuffer::new(hot_depth);
            let mut hl = self.hot_log.lock();
            mem::replace(&mut hl.events, new_circular)
        };
        let mut tli = self.inner.lock();
        let total = received_events.total_elements();
        let elements = received_events.len();
        tli.total_events += total;
        let mut consumed = received_events.take();
        let mut reversed = Vec::with_capacity(consumed.len() + 1);
        while let Some(log_entry) = consumed.pop() {
            reversed.push(log_entry);
        }
        if total > elements {
            // Too many events received, so some have been lost
            let new_log_entry = ExtLogRecord {
                timestamp: reversed[reversed.len() - 1].timestamp,
                level: Level::Warn,
                target: "TuiLogger".to_string(),
                file: "?".to_string(),
                line: 0,
                msg: format!(
                    "There have been {} events lost, {} recorded out of {}",
                    total - elements,
                    elements,
                    total
                ),
            };
            reversed.push(new_log_entry);
        }
        let default_level = tli.default;
        while let Some(log_entry) = reversed.pop() {
            if tli.targets.get(&log_entry.target).is_none() {
                tli.targets.set(&log_entry.target, default_level);
            }
            if let Some(ref mut file_options) = tli.dump {
                let mut output = String::new();
                let (lev_long, lev_abbr, with_loc) = match log_entry.level {
                    log::Level::Error => ("ERROR", "E", true),
                    log::Level::Warn => ("WARN ", "W", true),
                    log::Level::Info => ("INFO ", "I", false),
                    log::Level::Debug => ("DEBUG", "D", true),
                    log::Level::Trace => ("TRACE", "T", true),
                };
                if let Some(fmt) = file_options.timestamp_fmt.as_ref() {
                    output.push_str(&format!("{}", log_entry.timestamp.format(fmt)));
                    output.push(file_options.format_separator);
                }
                match file_options.format_output_level {
                    None => {}
                    Some(TuiLoggerLevelOutput::Abbreviated) => {
                        output.push_str(lev_abbr);
                        output.push(file_options.format_separator);
                    }
                    Some(TuiLoggerLevelOutput::Long) => {
                        output.push_str(lev_long);
                        output.push(file_options.format_separator);
                    }
                }
                if file_options.format_output_target {
                    output.push_str(&log_entry.target);
                    output.push(file_options.format_separator);
                }
                if with_loc {
                    if file_options.format_output_file {
                        output.push_str(&log_entry.file);
                        output.push(file_options.format_separator);
                    }
                    if file_options.format_output_line {
                        output.push_str(&format!("{}", log_entry.line));
                        output.push(file_options.format_separator);
                    }
                }
                output.push_str(&log_entry.msg);
                if let Err(_e) = writeln!(file_options.dump, "{}", output) {
                    // TODO: What to do in case of write error ?
                }
            }
            tli.events.push(log_entry);
        }
    }
}
lazy_static! {
    static ref TUI_LOGGER: TuiLogger = {
        let hs = HotSelect {
            hashtable: HashMap::with_capacity(1000),
            default: LevelFilter::Info,
        };
        let hl = HotLog {
            events: CircularBuffer::new(1000),
            mover_thread: None,
        };
        let tli = TuiLoggerInner {
            hot_depth: 1000,
            events: CircularBuffer::new(10000),
            total_events: 0,
            dump: None,
            default: LevelFilter::Info,
            targets: LevelConfig::new(),
        };
        TuiLogger {
            hot_select: Mutex::new(hs),
            hot_log: Mutex::new(hl),
            inner: Mutex::new(tli),
        }
    };
}

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

#[cfg(feature = "slog-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "slog-support")))]
pub fn slog_drain() -> TuiSlogDrain {
    TuiSlogDrain
}

#[cfg(feature = "tracing-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing-support")))]
pub fn tracing_subscriber_layer() -> TuiTracingSubscriberLayer {
    TuiTracingSubscriberLayer
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

/// Set levelfilter for a specific target in the logger
pub fn set_level_for_target(target: &str, levelfilter: LevelFilter) {
    let h = fxhash::hash64(&target);
    TUI_LOGGER.inner.lock().targets.set(target, levelfilter);
    let mut hs = TUI_LOGGER.hot_select.lock();
    hs.hashtable.insert(h, levelfilter);
}

impl TuiLogger {
    fn raw_log(&self, record: &Record) {
        let log_entry = ExtLogRecord {
            timestamp: chrono::Local::now(),
            level: record.level(),
            target: record.target().to_string(),
            file: record.file().unwrap_or("?").to_string(),
            line: record.line().unwrap_or(0),
            msg: format!("{}", record.args()),
        };
        let mut events_lock = self.hot_log.lock();
        events_lock.events.push(log_entry);
        let need_signal =
            (events_lock.events.total_elements() % (events_lock.events.capacity() / 2)) == 0;
        if need_signal {
            events_lock
                .mover_thread
                .as_ref()
                .map(|jh| thread::Thread::unpark(jh.thread()));
        }
    }
}

impl Log for TuiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let h = fxhash::hash64(metadata.target());
        let hs = self.hot_select.lock();
        if let Some(&levelfilter) = hs.hashtable.get(&h) {
            metadata.level() <= levelfilter
        } else {
            metadata.level() <= hs.default
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.raw_log(record)
        }
    }

    fn flush(&self) {}
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TuiWidgetEvent {
    SpaceKey,
    UpKey,
    DownKey,
    LeftKey,
    RightKey,
    PlusKey,
    MinusKey,
    HideKey,
    FocusKey,
    PrevPageKey,
    NextPageKey,
    EscapeKey,
}

/// This is the definition for the TuiLoggerTargetWidget,
/// which allows configuration of the logger system and selection of log messages.
pub struct TuiLoggerTargetWidget<'b> {
    block: Option<Block<'b>>,
    /// Base style of the widget
    style: Style,
    style_show: Style,
    style_hide: Style,
    style_off: Option<Style>,
    highlight_style: Style,
    state: Arc<Mutex<TuiWidgetInnerState>>,
    targets: Vec<String>,
}
impl<'b> Default for TuiLoggerTargetWidget<'b> {
    fn default() -> TuiLoggerTargetWidget<'b> {
        //TUI_LOGGER.move_events();
        TuiLoggerTargetWidget {
            block: None,
            style: Default::default(),
            style_off: None,
            style_hide: Style::default(),
            style_show: Style::default().add_modifier(Modifier::REVERSED),
            highlight_style: Style::default().add_modifier(Modifier::REVERSED),
            state: Arc::new(Mutex::new(TuiWidgetInnerState::new())),
            targets: vec![],
        }
    }
}
impl<'b> TuiLoggerTargetWidget<'b> {
    pub fn block(mut self, block: Block<'b>) -> TuiLoggerTargetWidget<'b> {
        self.block = Some(block);
        self
    }
    fn opt_style(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.style = s;
        }
        self
    }
    fn opt_style_off(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if style.is_some() {
            self.style_off = style;
        }
        self
    }
    fn opt_style_hide(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.style_hide = s;
        }
        self
    }
    fn opt_style_show(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.style_show = s;
        }
        self
    }
    fn opt_highlight_style(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.highlight_style = s;
        }
        self
    }
    pub fn style(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style = style;
        self
    }
    pub fn style_off(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style_off = Some(style);
        self
    }
    pub fn style_hide(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style_hide = style;
        self
    }
    pub fn style_show(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style_show = style;
        self
    }
    pub fn highlight_style(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.highlight_style = style;
        self
    }
    fn inner_state(mut self, state: Arc<Mutex<TuiWidgetInnerState>>) -> TuiLoggerTargetWidget<'b> {
        self.state = state;
        self
    }
    pub fn state(mut self, state: &TuiWidgetState) -> TuiLoggerTargetWidget<'b> {
        self.state = state.inner.clone();
        self
    }
}
impl<'b> Widget for TuiLoggerTargetWidget<'b> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        if list_area.width < 8 || list_area.height < 1 {
            return;
        }

        let la_left = list_area.left();
        let la_top = list_area.top();
        let la_width = list_area.width as usize;

        {
            let inner = &TUI_LOGGER.inner.lock();
            let hot_targets = &inner.targets;
            let mut state = self.state.lock();
            let hide_off = state.hide_off;
            let offset = state.offset;
            let focus_selected = state.focus_selected;
            {
                let targets = &mut state.config;
                targets.merge(hot_targets);
                self.targets.clear();
                for (t, levelfilter) in targets.iter() {
                    if hide_off && levelfilter == &LevelFilter::Off {
                        continue;
                    }
                    self.targets.push(t.clone());
                }
                self.targets.sort();
            }
            state.nr_items = self.targets.len();
            if state.selected >= state.nr_items {
                state.selected = state.nr_items.max(1) - 1;
            }
            if state.selected < state.nr_items {
                state.opt_selected_target = Some(self.targets[state.selected].clone());
                let t = &self.targets[state.selected];
                let (more, less) = if let Some(levelfilter) = state.config.get(t) {
                    advance_levelfilter(levelfilter)
                } else {
                    (None, None)
                };
                state.opt_selected_visibility_less = less;
                state.opt_selected_visibility_more = more;
                let (more, less) = if let Some(levelfilter) = hot_targets.get(t) {
                    advance_levelfilter(levelfilter)
                } else {
                    (None, None)
                };
                state.opt_selected_recording_less = less;
                state.opt_selected_recording_more = more;
            }
            let list_height = (list_area.height as usize).min(self.targets.len());
            let offset = if list_height > self.targets.len() {
                0
            } else if state.selected < state.nr_items {
                let sel = state.selected;
                if sel >= offset + list_height {
                    // selected is below visible list range => make it the bottom
                    sel - list_height + 1
                } else if sel.min(offset) + list_height > self.targets.len() {
                    self.targets.len() - list_height
                } else {
                    sel.min(offset)
                }
            } else {
                0
            };
            state.offset = offset;

            let targets = &(&state.config);
            let default_level = inner.default;
            for i in 0..list_height {
                let t = &self.targets[i + offset];
                // Comment in relation to issue #69:
                // Widgets maintain their own list of level filters per target.
                // These lists are not forwarded to the TUI_LOGGER, but kept widget private.
                // Example: This widget's private list contains a target named "not_yet",
                // and the application hasn't logged an entry with target "not_yet".
                // If displaying the target list, then "not_yet" will be only present in target,
                // but not in hot_targets. In issue #69 the problem has been, that
                // `hot_targets.get(t).unwrap()` has caused a panic. Which is to be expected.
                // The remedy is to use unwrap_or with default_level.
                let hot_level_filter = hot_targets.get(t).unwrap_or(default_level);
                let level_filter = targets.get(t).unwrap_or(default_level);
                for (j, sym, lev) in &[
                    (0, "E", Level::Error),
                    (1, "W", Level::Warn),
                    (2, "I", Level::Info),
                    (3, "D", Level::Debug),
                    (4, "T", Level::Trace),
                ] {
                    if let Some(cell) = buf.cell_mut((la_left + j, la_top + i as u16)) {
                        let cell_style = if hot_level_filter >= *lev {
                            if level_filter >= *lev {
                                if !focus_selected || i + offset == state.selected {
                                    self.style_show
                                } else {
                                    self.style_hide
                                }
                            } else {
                                self.style_hide
                            }
                        } else if let Some(style_off) = self.style_off {
                            style_off
                        } else {
                            cell.set_symbol(" ");
                            continue;
                        };
                        cell.set_style(cell_style);
                        cell.set_symbol(sym);
                    }
                }
                buf.set_stringn(la_left + 5, la_top + i as u16, ":", la_width, self.style);
                buf.set_stringn(
                    la_left + 6,
                    la_top + i as u16,
                    t,
                    la_width,
                    if i + offset == state.selected {
                        self.highlight_style
                    } else {
                        self.style
                    },
                );
            }
        }
    }
}

/// The TuiLoggerWidget shows the logging messages in an endless scrolling view.
/// It is controlled by a TuiWidgetState for selected events.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TuiLoggerLevelOutput {
    Abbreviated,
    Long,
}

// Generate README by gawk 'substr($0,1,3) == "//!"{print(substr($0,4))}' src/lib.rs

//! # Logger with smart widget for the `tui` crate
//!
//! ![Build Status](https://travis-ci.org/gin66/tui-logger.svg?branch=master)
//! ![dependency status](https://deps.rs/repo/github/gin66/tui-logger/status.svg?service=github)
//! ![Build examples](https://github.com/gin66/tui-logger/workflows/Build%20examples/badge.svg?service=github)
//!
//!
//! ## Demo of the widget
//!
//! [Demo](https://github.com/gin66/tui-logger/blob/master/doc/example.svg?raw=true)
//!
//! ## Documentation
//!
//! [Documentation](https://docs.rs/tui-logger/0.5.0/tui_logger/)
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
//! ## `slog` support
//!
//! `tui-logger` provides a TuiSlogDrain which implements `slog::Drain` and will route all records
//! it receives to the `tui-logger` widget
//!
//! ## Applications using tui-logger
//!
//! * [wash](https://github.com/wasmCloud/wash)
//! * [rocker](https://github.com/atlassian/rocker)
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
//!
#[macro_use]
extern crate lazy_static;

use std::collections::hash_map::Iter;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::mem;
use std::sync::Arc;

use chrono::{DateTime, Local};
use log::{Level, LevelFilter, Log, Metadata, Record};
use parking_lot::Mutex;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Widget};

mod circular;
mod slog;

pub use crate::circular::CircularBuffer;
pub use crate::slog::TuiSlogDrain;

struct ExtLogRecord {
    timestamp: DateTime<Local>,
    level: Level,
    target: String,
    file: String,
    line: u32,
    msg: String,
}

fn advance_levelfilter(levelfilter: &LevelFilter) -> (Option<LevelFilter>, Option<LevelFilter>) {
    match *levelfilter {
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
    pub fn get(&self, target: &str) -> Option<&LevelFilter> {
        self.config.get(target)
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
                    if levelfilter <= origin_levelfilter {
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
}
struct TuiLoggerInner {
    hot_depth: usize,
    events: CircularBuffer<ExtLogRecord>,
    total_events: usize,
    dump: Option<File>,
    default: LevelFilter,
    targets: LevelConfig,
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
            let new_circular = CircularBuffer::new(self.inner.lock().hot_depth);
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
            if let Some(ref mut file) = tli.dump {
                if let Err(_e) = writeln!(
                    file,
                    "{}:{}:{}:{}:{}:{}",
                    &log_entry.timestamp.format("[%Y:%m:%d %H:%M:%S]"),
                    log_entry.level,
                    log_entry.target,
                    &log_entry.file,
                    log_entry.line,
                    &log_entry.msg
                ) {
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

/// Init the logger and record with `log` crate.
pub fn init_logger(max_level: LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_max_level(max_level);
    log::set_logger(&*TUI_LOGGER)
}

pub fn slog_drain() -> TuiSlogDrain {
    TuiSlogDrain
}

/// Set the depth of the hot buffer in order to avoid message loss.
/// This is effective only after a call to move_events()
pub fn set_hot_buffer_depth(depth: usize) {
    TUI_LOGGER.inner.lock().hot_depth = depth;
}

/// Move events from hot circular buffer to the main one.
/// If defined, log records will be written to file.
pub fn move_events() {
    TUI_LOGGER.move_events();
}

/// Define filename for logging.
pub fn set_log_file(fname: &str) -> io::Result<()> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(fname)
        .map(|file| {
            TUI_LOGGER.inner.lock().dump = Some(file);
        })
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
            let log_entry = ExtLogRecord {
                timestamp: chrono::Local::now(),
                level: record.level(),
                target: record.target().to_string(),
                file: record.file().unwrap_or("?").to_string(),
                line: record.line().unwrap_or(0),
                msg: format!("{}", record.args()),
            };
            self.hot_log.lock().events.push(log_entry);
        }
    }

    fn flush(&self) {}
}

#[derive(Debug, Clone, PartialEq, Hash)]
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

#[derive(Default)]
struct TuiWidgetInnerState {
    config: LevelConfig,
    nr_items: usize,
    selected: usize,
    opt_timestamp_bottom: Option<DateTime<Local>>,
    opt_timestamp_next_page: Option<DateTime<Local>>,
    opt_timestamp_prev_page: Option<DateTime<Local>>,
    opt_selected_target: Option<String>,
    opt_selected_visibility_more: Option<LevelFilter>,
    opt_selected_visibility_less: Option<LevelFilter>,
    opt_selected_recording_more: Option<LevelFilter>,
    opt_selected_recording_less: Option<LevelFilter>,
    offset: usize,
    hide_off: bool,
    hide_target: bool,
    focus_selected: bool,
}
impl TuiWidgetInnerState {
    pub fn new() -> TuiWidgetInnerState {
        TuiWidgetInnerState::default()
    }
    fn transition(&mut self, event: &TuiWidgetEvent) {
        use TuiWidgetEvent::*;
        match *event {
            SpaceKey => {
                self.hide_off ^= true;
            }
            HideKey => {
                self.hide_target ^= true;
            }
            FocusKey => {
                self.focus_selected ^= true;
            }
            UpKey => {
                if !self.hide_target && self.selected > 0 {
                    self.selected -= 1;
                }
            }
            DownKey => {
                if !self.hide_target && self.selected + 1 < self.nr_items {
                    self.selected += 1;
                }
            }
            LeftKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_visibility_less) = self.opt_selected_visibility_less.take()
                    {
                        self.config.set(&selected_target, selected_visibility_less);
                    }
                }
            }
            RightKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_visibility_more) = self.opt_selected_visibility_more.take()
                    {
                        self.config.set(&selected_target, selected_visibility_more);
                    }
                }
            }
            PlusKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_recording_more) = self.opt_selected_recording_more.take() {
                        set_level_for_target(&selected_target, selected_recording_more);
                    }
                }
            }
            MinusKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_recording_less) = self.opt_selected_recording_less.take() {
                        set_level_for_target(&selected_target, selected_recording_less);
                    }
                }
            }
            PrevPageKey => self.opt_timestamp_bottom = self.opt_timestamp_prev_page,
            NextPageKey => self.opt_timestamp_bottom = self.opt_timestamp_next_page,
            EscapeKey => self.opt_timestamp_bottom = None,
        }
    }
}

/// This struct contains the shared state of a TuiLoggerWidget and a TuiLoggerTargetWidget.
#[derive(Default)]
pub struct TuiWidgetState {
    inner: Arc<Mutex<TuiWidgetInnerState>>,
}
impl TuiWidgetState {
    /// Create a new TuiWidgetState
    pub fn new() -> TuiWidgetState {
        TuiWidgetState {
            inner: Arc::new(Mutex::new(TuiWidgetInnerState::new())),
        }
    }
    pub fn set_default_display_level(self, levelfilter: LevelFilter) -> TuiWidgetState {
        self.inner.lock().config.default_display_level = Some(levelfilter);
        self
    }
    pub fn set_level_for_target(self, target: &str, levelfilter: LevelFilter) -> TuiWidgetState {
        self.inner.lock().config.set(target, levelfilter);
        self
    }
    pub fn transition(&mut self, event: &TuiWidgetEvent) {
        self.inner.lock().transition(event);
    }
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
        TUI_LOGGER.move_events();
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
            let hot_targets = &TUI_LOGGER.inner.lock().targets;
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
            for i in 0..list_height {
                let t = &self.targets[i + offset];
                let hot_level_filter = hot_targets.get(t).unwrap();
                let level_filter = targets.get(t).unwrap();
                for (j, sym, lev) in &[
                    (0, "E", Level::Error),
                    (1, "W", Level::Warn),
                    (2, "I", Level::Info),
                    (3, "D", Level::Debug),
                    (4, "T", Level::Trace),
                ] {
                    let mut cell = buf.get_mut(la_left + j, la_top + i as u16);
                    let cell_style = if *hot_level_filter >= *lev {
                        if *level_filter >= *lev {
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
                        cell.symbol = " ".to_string();
                        continue;
                    };
                    cell.set_style(cell_style);
                    cell.symbol = sym.to_string();
                }
                buf.set_stringn(la_left + 5, la_top + i as u16, &":", la_width, self.style);
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
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TuiLoggerLevelOutput {
    Abbreviated,
    Long,
}

pub struct TuiLoggerWidget<'b> {
    block: Option<Block<'b>>,
    /// Base style of the widget
    style: Style,
    /// Level based style
    style_error: Option<Style>,
    style_warn: Option<Style>,
    style_debug: Option<Style>,
    style_trace: Option<Style>,
    style_info: Option<Style>,
    format_separator: char,
    format_timestamp: Option<String>,
    format_output_level: Option<TuiLoggerLevelOutput>,
    format_output_target: bool,
    format_output_file: bool,
    format_output_line: bool,
    state: Arc<Mutex<TuiWidgetInnerState>>,
}
impl<'b> Default for TuiLoggerWidget<'b> {
    fn default() -> TuiLoggerWidget<'b> {
        TUI_LOGGER.move_events();
        TuiLoggerWidget {
            block: None,
            style: Default::default(),
            style_error: None,
            style_warn: None,
            style_debug: None,
            style_trace: None,
            style_info: None,
            format_separator: ':',
            format_timestamp: Some("%H:%M:%S".to_string()),
            format_output_level: Some(TuiLoggerLevelOutput::Long),
            format_output_target: true,
            format_output_file: true,
            format_output_line: true,
            state: Arc::new(Mutex::new(TuiWidgetInnerState::new())),
        }
    }
}
impl<'b> TuiLoggerWidget<'b> {
    pub fn block(mut self, block: Block<'b>) -> Self {
        self.block = Some(block);
        self
    }
    fn opt_style(mut self, style: Option<Style>) -> Self {
        if let Some(s) = style {
            self.style = s;
        }
        self
    }
    fn opt_style_error(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_error = style;
        }
        self
    }
    fn opt_style_warn(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_warn = style;
        }
        self
    }
    fn opt_style_info(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_info = style;
        }
        self
    }
    fn opt_style_trace(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_trace = style;
        }
        self
    }
    fn opt_style_debug(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_debug = style;
        }
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn style_error(mut self, style: Style) -> Self {
        self.style_error = Some(style);
        self
    }
    pub fn style_warn(mut self, style: Style) -> Self {
        self.style_warn = Some(style);
        self
    }
    pub fn style_info(mut self, style: Style) -> Self {
        self.style_info = Some(style);
        self
    }
    pub fn style_trace(mut self, style: Style) -> Self {
        self.style_trace = Some(style);
        self
    }
    pub fn style_debug(mut self, style: Style) -> Self {
        self.style_debug = Some(style);
        self
    }
    fn opt_output_separator(mut self, opt_sep: Option<char>) -> Self {
        if let Some(ch) = opt_sep {
            self.format_separator = ch;
        }
        self
    }
    /// Separator character between field.
    /// Default is ':'
    pub fn output_separator(mut self, sep: char) -> Self {
        self.format_separator = sep;
        self
    }
    fn opt_output_timestamp(mut self, opt_fmt: Option<Option<String>>) -> Self {
        if let Some(fmt) = opt_fmt {
            self.format_timestamp = fmt;
        }
        self
    }
    /// The format string can be defined as described in
    /// <https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html>
    ///
    /// If called with None, timestamp is not included in output.
    ///
    /// Default is %H:%M:%S
    pub fn output_timestamp(mut self, fmt: Option<String>) -> Self {
        self.format_timestamp = fmt;
        self
    }
    fn opt_output_level(mut self, opt_fmt: Option<Option<TuiLoggerLevelOutput>>) -> Self {
        if let Some(fmt) = opt_fmt {
            self.format_output_level = fmt;
        }
        self
    }
    /// Possible values are
    /// - TuiLoggerLevelOutput::Long        => DEBUG/TRACE/...
    /// - TuiLoggerLevelOutput::Abbreviated => D/T/...
    ///
    /// If called with None, level is not included in output.
    ///
    /// Default is Long
    pub fn output_level(mut self, level: Option<TuiLoggerLevelOutput>) -> Self {
        self.format_output_level = level;
        self
    }
    fn opt_output_target(mut self, opt_enabled: Option<bool>) -> Self {
        if let Some(enabled) = opt_enabled {
            self.format_output_target = enabled;
        }
        self
    }
    /// Enables output of target field of event
    ///
    /// Default is true
    pub fn output_target(mut self, enabled: bool) -> Self {
        self.format_output_target = enabled;
        self
    }
    fn opt_output_file(mut self, opt_enabled: Option<bool>) -> Self {
        if let Some(enabled) = opt_enabled {
            self.format_output_file = enabled;
        }
        self
    }
    /// Enables output of file field of event
    ///
    /// Default is true
    pub fn output_file(mut self, enabled: bool) -> Self {
        self.format_output_file = enabled;
        self
    }
    fn opt_output_line(mut self, opt_enabled: Option<bool>) -> Self {
        if let Some(enabled) = opt_enabled {
            self.format_output_line = enabled;
        }
        self
    }
    /// Enables output of line field of event
    ///
    /// Default is true
    pub fn output_line(mut self, enabled: bool) -> Self {
        self.format_output_line = enabled;
        self
    }
    fn inner_state(mut self, state: Arc<Mutex<TuiWidgetInnerState>>) -> Self {
        self.state = state;
        self
    }
    pub fn state(&mut self, state: &TuiWidgetState) -> &mut TuiLoggerWidget<'b> {
        self.state = state.inner.clone();
        self
    }
    fn format_event(&self, evt: &ExtLogRecord) -> (String, Option<Style>) {
        let mut output = String::new();
        let (col_style, lev_long, lev_abbr, with_loc) = match evt.level {
            log::Level::Error => (self.style_error, "ERROR", "E", true),
            log::Level::Warn => (self.style_warn, "WARN ", "W", true),
            log::Level::Info => (self.style_info, "INFO ", "I", false),
            log::Level::Debug => (self.style_debug, "DEBUG", "D", true),
            log::Level::Trace => (self.style_trace, "TRACE", "T", true),
        };
        if let Some(fmt) = self.format_timestamp.as_ref() {
            output.push_str(&format!("{}", evt.timestamp.format(fmt)));
            output.push(self.format_separator);
        }
        match &self.format_output_level {
            None => {}
            Some(TuiLoggerLevelOutput::Abbreviated) => {
                output.push_str(lev_abbr);
                output.push(self.format_separator);
            }
            Some(TuiLoggerLevelOutput::Long) => {
                output.push_str(lev_long);
                output.push(self.format_separator);
            }
        }
        if self.format_output_target {
            output.push_str(&evt.target);
            output.push(self.format_separator);
        }
        if with_loc {
            if self.format_output_file {
                output.push_str(&evt.file);
                output.push(self.format_separator);
            }
            if self.format_output_line {
                output.push_str(&format!("{}", evt.line));
                output.push(self.format_separator);
            }
        }
        (output, col_style)
    }
}
impl<'b> Widget for TuiLoggerWidget<'b> {
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

        let mut state = self.state.lock();
        let la_height = list_area.height as usize;
        let mut lines: Vec<(Option<Style>, u16, String)> = vec![];
        let indent = 9;
        {
            state.opt_timestamp_next_page = None;
            let opt_timestamp_bottom = state.opt_timestamp_bottom;
            let mut opt_timestamp_prev_page = None;
            let mut tui_lock = TUI_LOGGER.inner.lock();
            let mut circular = CircularBuffer::new(10); // MAGIC constant
            for evt in tui_lock.events.rev_iter() {
                if let Some(level) = state.config.get(&evt.target) {
                    if *level < evt.level {
                        continue;
                    }
                }
                if state.focus_selected {
                    if let Some(target) = state.opt_selected_target.as_ref() {
                        if target != &evt.target {
                            continue;
                        }
                    }
                }
                // Here all filters have been applied,
                // So check, if user is paging through history
                if let Some(timestamp) = opt_timestamp_bottom.as_ref() {
                    if *timestamp < evt.timestamp {
                        circular.push(evt.timestamp);
                        continue;
                    }
                }
                if !circular.is_empty() {
                    state.opt_timestamp_next_page = circular.take().first().cloned();
                }
                let (mut output, col_style) = self.format_event(evt);
                let mut sublines: Vec<&str> = evt.msg.lines().rev().collect();
                output.push_str(sublines.pop().unwrap());
                for subline in sublines {
                    lines.push((col_style, indent, subline.to_string()));
                }
                lines.push((col_style, 0, output));
                if lines.len() == la_height {
                    break;
                }
                if opt_timestamp_prev_page.is_none() && lines.len() >= la_height / 2 {
                    opt_timestamp_prev_page = Some(evt.timestamp);
                }
            }
            state.opt_timestamp_prev_page = opt_timestamp_prev_page.or(state.opt_timestamp_bottom);
        }
        let la_left = list_area.left();
        let la_top = list_area.top();
        let la_width = list_area.width as usize;

        // lines is a vector with bottom line at index 0
        // wrapped_lines will be a vector with top line first
        let mut wrapped_lines = CircularBuffer::new(la_height);
        while let Some((style, left, line)) = lines.pop() {
            if line.chars().count() > la_width {
                wrapped_lines.push((style, left, line.chars().take(la_width).collect()));
                let mut remain: String = line.chars().skip(la_width).collect();
                let rem_width = la_width - indent as usize;
                while remain.chars().count() > rem_width {
                    let remove: String = remain.chars().take(rem_width).collect();
                    wrapped_lines.push((style, indent, remove));
                    remain = remain.chars().skip(rem_width).collect();
                }
                wrapped_lines.push((style, indent, remain.to_owned()));
            } else {
                wrapped_lines.push((style, left, line));
            }
        }

        let offset: u16 = if state.opt_timestamp_bottom.is_none() {
            0
        } else {
            let lines_cnt = wrapped_lines.len();
            (la_height - lines_cnt) as u16
        };

        for (i, (sty, left, l)) in wrapped_lines.iter().enumerate() {
            buf.set_stringn(
                la_left + left,
                la_top + i as u16 + offset,
                l,
                l.len(),
                sty.unwrap_or(self.style),
            );
        }
    }
}

/// The Smart Widget combines the TuiLoggerWidget and the TuiLoggerTargetWidget
/// into a nice combo, where the TuiLoggerTargetWidget can be shown/hidden.
///
/// In the title the number of logging messages/s in the whole buffer is shown.
pub struct TuiLoggerSmartWidget<'a> {
    title_log: Spans<'a>,
    title_target: Spans<'a>,
    style: Option<Style>,
    border_style: Style,
    highlight_style: Option<Style>,
    style_error: Option<Style>,
    style_warn: Option<Style>,
    style_debug: Option<Style>,
    style_trace: Option<Style>,
    style_info: Option<Style>,
    style_show: Option<Style>,
    style_hide: Option<Style>,
    style_off: Option<Style>,
    format_separator: Option<char>,
    format_timestamp: Option<Option<String>>,
    format_output_level: Option<Option<TuiLoggerLevelOutput>>,
    format_output_target: Option<bool>,
    format_output_file: Option<bool>,
    format_output_line: Option<bool>,
    state: Arc<Mutex<TuiWidgetInnerState>>,
}
impl<'a> Default for TuiLoggerSmartWidget<'a> {
    fn default() -> Self {
        TUI_LOGGER.move_events();
        TuiLoggerSmartWidget {
            title_log: Spans::from("Tui Log"),
            title_target: Spans::from("Tui Target Selector"),
            style: None,
            border_style: Style::default(),
            highlight_style: None,
            style_error: None,
            style_warn: None,
            style_debug: None,
            style_trace: None,
            style_info: None,
            style_show: None,
            style_hide: None,
            style_off: None,
            format_separator: None,
            format_timestamp: None,
            format_output_level: None,
            format_output_target: None,
            format_output_file: None,
            format_output_line: None,
            state: Arc::new(Mutex::new(TuiWidgetInnerState::new())),
        }
    }
}
impl<'a> TuiLoggerSmartWidget<'a> {
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
    pub fn style_error(mut self, style: Style) -> Self {
        self.style_error = Some(style);
        self
    }
    pub fn style_warn(mut self, style: Style) -> Self {
        self.style_warn = Some(style);
        self
    }
    pub fn style_info(mut self, style: Style) -> Self {
        self.style_info = Some(style);
        self
    }
    pub fn style_trace(mut self, style: Style) -> Self {
        self.style_trace = Some(style);
        self
    }
    pub fn style_debug(mut self, style: Style) -> Self {
        self.style_debug = Some(style);
        self
    }
    pub fn style_off(mut self, style: Style) -> Self {
        self.style_off = Some(style);
        self
    }
    pub fn style_hide(mut self, style: Style) -> Self {
        self.style_hide = Some(style);
        self
    }
    pub fn style_show(mut self, style: Style) -> Self {
        self.style_show = Some(style);
        self
    }
    /// Separator character between field.
    /// Default is ':'
    pub fn output_separator(mut self, sep: char) -> Self {
        self.format_separator = Some(sep);
        self
    }
    /// The format string can be defined as described in
    /// <https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html>
    ///
    /// If called with None, timestamp is not included in output.
    ///
    /// Default is %H:%M:%S
    pub fn output_timestamp(mut self, fmt: Option<String>) -> Self {
        self.format_timestamp = Some(fmt);
        self
    }
    /// Possible values are
    /// - TuiLoggerLevelOutput::Long        => DEBUG/TRACE/...
    /// - TuiLoggerLevelOutput::Abbreviated => D/T/...
    ///
    /// If called with None, level is not included in output.
    ///
    /// Default is Long
    pub fn output_level(mut self, level: Option<TuiLoggerLevelOutput>) -> Self {
        self.format_output_level = Some(level);
        self
    }
    /// Enables output of target field of event
    ///
    /// Default is true
    pub fn output_target(mut self, enabled: bool) -> Self {
        self.format_output_target = Some(enabled);
        self
    }
    /// Enables output of file field of event
    ///
    /// Default is true
    pub fn output_file(mut self, enabled: bool) -> Self {
        self.format_output_file = Some(enabled);
        self
    }
    /// Enables output of line field of event
    ///
    /// Default is true
    pub fn output_line(mut self, enabled: bool) -> Self {
        self.format_output_line = Some(enabled);
        self
    }
    pub fn title_target<T>(mut self, title: T) -> Self
    where
        T: Into<Spans<'a>>,
    {
        self.title_target = title.into();
        self
    }
    pub fn title_log<T>(mut self, title: T) -> Self
    where
        T: Into<Spans<'a>>,
    {
        self.title_log = title.into();
        self
    }
    pub fn state(mut self, state: &TuiWidgetState) -> Self {
        self.state = state.inner.clone();
        self
    }
}
impl<'a> Widget for TuiLoggerSmartWidget<'a> {
    /// Nothing to draw for combo widget
    fn render(self, area: Rect, buf: &mut Buffer) {
        let entries_s = {
            let mut tui_lock = TUI_LOGGER.inner.lock();
            let first_timestamp = tui_lock
                .events
                .iter()
                .next()
                .map(|entry| entry.timestamp.timestamp_millis());
            let last_timestamp = tui_lock
                .events
                .rev_iter()
                .next()
                .map(|entry| entry.timestamp.timestamp_millis());
            if let Some(first) = first_timestamp {
                if let Some(last) = last_timestamp {
                    let dt = last - first;
                    if dt > 0 {
                        tui_lock.events.len() as f64 / (dt as f64) * 1000.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            } else {
                0.0
            }
        };

        let mut title_log = self.title_log.clone();
        title_log
            .0
            .push(format!(" [log={:.1}/s]", entries_s).into());

        let hide_target = self.state.lock().hide_target;
        if hide_target {
            let tui_lw = TuiLoggerWidget::default()
                .block(
                    Block::default()
                        .title(title_log)
                        .border_style(self.border_style)
                        .borders(Borders::ALL),
                )
                .opt_style(self.style)
                .opt_style_error(self.style_error)
                .opt_style_warn(self.style_warn)
                .opt_style_info(self.style_info)
                .opt_style_debug(self.style_debug)
                .opt_style_trace(self.style_trace)
                .opt_output_separator(self.format_separator)
                .opt_output_timestamp(self.format_timestamp)
                .opt_output_level(self.format_output_level)
                .opt_output_target(self.format_output_target)
                .opt_output_file(self.format_output_file)
                .opt_output_line(self.format_output_line)
                .inner_state(self.state);
            tui_lw.render(area, buf);
        } else {
            let mut width: usize = 0;
            {
                let hot_targets = &TUI_LOGGER.inner.lock().targets;
                let mut state = self.state.lock();
                let hide_off = state.hide_off;
                {
                    let targets = &mut state.config;
                    targets.merge(hot_targets);
                    for (t, levelfilter) in targets.iter() {
                        if hide_off && levelfilter == &LevelFilter::Off {
                            continue;
                        }
                        width = width.max(t.len())
                    }
                }
            }
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Length(width as u16 + 6 + 2),
                    Constraint::Min(10),
                ])
                .split(area);
            let tui_ltw = TuiLoggerTargetWidget::default()
                .block(
                    Block::default()
                        .title(self.title_target)
                        .border_style(self.border_style)
                        .borders(Borders::ALL),
                )
                .opt_style(self.style)
                .opt_highlight_style(self.highlight_style)
                .opt_style_off(self.style_off)
                .opt_style_hide(self.style_hide)
                .opt_style_show(self.style_show)
                .inner_state(self.state.clone());
            tui_ltw.render(chunks[0], buf);
            let tui_lw = TuiLoggerWidget::default()
                .block(
                    Block::default()
                        .title(title_log)
                        .border_style(self.border_style)
                        .borders(Borders::ALL),
                )
                .opt_style(self.style)
                .opt_style_error(self.style_error)
                .opt_style_warn(self.style_warn)
                .opt_style_info(self.style_info)
                .opt_style_debug(self.style_debug)
                .opt_style_trace(self.style_trace)
                .opt_output_separator(self.format_separator)
                .opt_output_timestamp(self.format_timestamp)
                .opt_output_level(self.format_output_level)
                .opt_output_target(self.format_output_target)
                .opt_output_file(self.format_output_file)
                .opt_output_line(self.format_output_line)
                .inner_state(self.state.clone());
            tui_lw.render(chunks[1], buf);
        }
    }
}

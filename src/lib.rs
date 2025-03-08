//! # Logger with smart widget for the `tui` and `ratatui` crate
//!
//! [![dependency status](https://deps.rs/repo/github/gin66/tui-logger/status.svg?service=github&nocache=0_9_1)](https://deps.rs/repo/github/gin66/tui-logger)
//! ![Build examples](https://github.com/gin66/tui-logger/workflows/Build%20examples/badge.svg?service=github)
//!
//!
//! ## Demo of the widget
//!
//! ![Demo](https://github.com/gin66/tui-logger/blob/master/doc/demo_v0.14.4.gif?raw=true)
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
//! - [X] Support to use custom formatter for log events
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
//! Run demo using termion and simple custom formatter in bottom right log widget:
//!
//! ```ignore
//! cargo run --example demo --features termion,formatter
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
//! ## Custom formatting
//!
//! For experts only ! Configure along the lines:
//! ```ignore
//! use tui_logger::LogFormatter;
//!
//! let formatter = MyLogFormatter();
//!
//! TuiLoggerWidget::default()
//! .block(Block::bordered().title("Filtered TuiLoggerWidget"))
//! .formatter(formatter)
//! .state(&filter_state)
//! .render(left, buf);
//! ```
//! The example demo can be invoked to use a custom formatter as example for the bottom right widget.
//!
// Enable docsrs doc_cfg - to display non-default feature documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]
#[macro_use]
extern crate lazy_static;

mod circular;
pub use crate::circular::CircularBuffer;

#[cfg(feature = "slog-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "slog-support")))]
mod slog;
#[cfg(feature = "slog-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "slog-support")))]
pub use crate::slog::TuiSlogDrain;

#[cfg(feature = "tracing-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing-support")))]
mod tracing_subscriber;
#[cfg(feature = "tracing-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing-support")))]
pub use crate::tracing_subscriber::TuiTracingSubscriberLayer;
#[doc(no_inline)]
pub use log::LevelFilter;

mod widget;
pub use widget::inner::TuiWidgetState;
pub use widget::logformatter::LogFormatter;
pub use widget::smart::TuiLoggerSmartWidget;
pub use widget::standard::TuiLoggerWidget;
pub use widget::target::TuiLoggerTargetWidget;
pub use widget::inner::TuiWidgetEvent;

mod config;
pub use config::LevelConfig;

mod file;
pub use file::TuiLoggerFile;

mod logger;
use crate::logger::*;
pub use crate::logger::TuiLoggerLevelOutput;
pub use crate::logger::ExtLogRecord;
pub use crate::logger::api::*;

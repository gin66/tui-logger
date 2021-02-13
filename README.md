# Logger with smart widget for the `tui` crate

[![Build Status](https://travis-ci.org/gin66/tui-logger.svg?branch=master)](https://travis-ci.org/gin66/tui-logger)
[![dependency status](https://deps.rs/repo/github/gin66/tui-logger/status.svg?service=github)](https://deps.rs/repo/github/gin66/tui-logger)
![Build examples](https://github.com/gin66/tui-logger/workflows/Build%20examples/badge.svg?service=github)

## Documentation

[Documentation](https://docs.rs/tui-logger/0.4.2/tui_logger/)

## Features

- [X] Logger implementation for the `log` crate
- [X] Logger enable/disable detection via hash table (avoid string compare)
- [X] Hot logger code only copies enabled log messages with timestamp into a circular buffer
- [X] Widgets/move_message() retrieve captured log messages from hot circular buffer
- [X] Lost message detection due to circular buffer
- [X] Log filtering performed on log record target
- [X] Simple Widgets to view logs and configure debuglevel per target
- [X] Smart Widget with dynamic event dispatcher for `termion` events (see demo code)
- [X] Logging of enabled logs to file
- [X] Event dispatcher for termion key events for smart/simple widget control
- [ ] Allow configuration of target dependent loglevel specifically for file logging
- [ ] Avoid duplicating of target, module and filename in every log record
- [ ] Simultaneous modification of all targets' display/hot logging loglevel by key command
- [X] Support both `termion` and `crossterm` backends of `tui`

## Smart Widget

Smart widget consists of two widgets. Left is the target selector widget and
on the right side the logging messages view scrolling up. The target selector widget
can be hidden/shown during runtime via key command.

The target selector widget looks like this:

   ![alt text](https://github.com/gin66/tui-logger/blob/master/doc/example.png?raw=true)

It controls:

- Capturing of log messages by the logger
- Selection of levels for display in the logging message view
 
The target selector widget consists of two columns:

- Code EWIDT: E stands for Error, W for Warn, Info, Debug and Trace.
  + Inverted characters (EWIDT) are enabled log levels in the view
  + Normal characters show enabled capturing of a log level per target
  + If any of EWIDT are not shown, then the respective log level is not captured
- Target of the log events can be defined in the log e.g. `warn!(target: "demo", "Log message");`
 
## Event Dispatcher

In order to allow above mentioned control via key events, a dispatcher has been integrated.
The dispatcher as module is independent from the backend, but the widgets are in the moment
specifically only for termion. The event handler queue is dynamically built during drawing of
the tui elements. This allows an easy link between complex ui layouts and the embedded widgets.
This could even be used for mouse events, but this is not yet implemented.

## Smart Widget Key Commands

|  KEY   | ACTION 
|:------:|-----------------------------------------------------------|
| `h`    | Toggles target selector widget hidden/visible 
| `f`    | Toggle focus on the selected target only
| `UP`   | Select previous target in target selector widget
| `DOWN` | Select next target in target selector widget
| `LEFT` | Reduce SHOWN (!) log messages by one level
| `RIGHT`| Increase SHOWN (!) log messages by one level
| `-`    | Reduce CAPTURED (!) log messages by one level
| `+`    | Increase CAPTURED (!) log messages by one level
| `SPACE`| Toggles hiding of targets, which have logfilter set to off
 
## Basic usage to initialize logger-system:
```
#[macro_use]
extern crate log;

use tui_logger::*;

fn main() {
    // Early initialization of the logger

    // Set max_log_level to Trace
    init_logger(log::LevelFilter::Trace).unwrap();

    // Set default level for unknown targets to Trace
    set_default_level(log::LevelFilter::Trace);

    // code....
}
```

For use of the widget please check examples/demo.rs

## THANKS TO

* [Florian Dehau](https://github.com/fdehau) for his great crate [tui-rs](https://github.com/fdehau/tui-rs)
* [Antoine Büsch](https://github.com/abusch) for providing the patches to tui-rs v0.3.0 and v0.6.0
* [Adam Sypniewski](https://github.com/ajsyp) for providing the patches to tui-rs v0.6.2
* [James aka jklong](https://github.com/jklong) for providing the patch to tui-rs v0.7
* [icy-ux](https://github.com/icy-ux) for adding slog support and example
* [alvinhochun](https://github.com/alvinhochun) for updating to tui 0.10 and crossterm support


# Logger with smart widget for the `tui` crate

[![Build Status](https://travis-ci.org/gin66/tui-logger.svg?branch=master)](https://travis-ci.org/gin66/tui-logger)

## Demo of the widget

[![alt](https://asciinema.org/a/6Jxk6i3lK6IDGyWGyLZkS5Rdl.png)](https://asciinema.org/a/6Jxk6i3lK6IDGyWGyLZkS5Rdl)

## Documentation

[Documentation](https://docs.rs/tui-logger/0.1.9/tui_logger/)

## Features

- [X] Logger implementation for the `log` crate
- [X] Logger enable/disable detection uses fast, collision free hash table
- [ ] Collision free hash table algorithm to be done. Currently use a big table
- [X] Hot logger code only copies enabled log messages with timestamp into a circular buffer
- [X] Widgets/move_message() retrieve captured log messages from hot circular buffer
- [X] Lost message detection of overwritten logs messages in circular buffer
- [X] Log filtering performed on log-target
- [X] Simple Widgets to view the logs and select Debuglevel of target
- [X] Smart Widget with dynamic event dispatcher for `termion` events (see demo code)
- [X] Logging of enabled logs to file

## Smart Widget

Smart widget consists of two widgets. Left is the target selector widget and
on the right side the logging messages view scrolling up.

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
 
## Smart Widget Key Commands

|  KEY   | ACTION 
|:------:|-----------------------------------------------------------|
| `h`    | Toggles target selector widget
| `UP`   | Select previous target in target selector widget
| `DOWN` | Select next target in target selector widget
| `LEFT` | Reduce SHOWN (!) log messages by one level
| `RIGHT`| Increase SHOWN (!) log messages by one level
| `-`    | Reduce CAPTURED (!) log messages by one level
| `+`    | Increase CAPTURED (!) log messages by one level
| `SPACE`| Toggles hiding of targets, which have logfilter set to off
 
## Basic usage to initialize logger-system:
```
extern crate log;
extern crate tui_logger;

use log::LevelFilter; 
use tui_logger::*;

fn main() {
    // Early initialization of the logger

    // Set max_log_level to Trace
    init_logger(LevelFilter::Trace).unwrap();

    // Set default level for unknown targets to Trace
    set_default_level(LevelFilter::Trace);

    // code....
}
```

For use of the widget please check examples/demo.rs


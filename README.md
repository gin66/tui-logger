 # Logger with smart widget for the `tui` crate

 ![Build Status](https://travis-ci.org/gin66/tui-logger.svg?branch=master)
 ![dependency status](https://deps.rs/repo/github/gin66/tui-logger/status.svg?service=github)
 ![Build examples](https://github.com/gin66/tui-logger/workflows/Build%20examples/badge.svg?service=github)


 ## Demo of the widget

 ![Demo](https://github.com/gin66/tui-logger/blob/master/doc/example.svg?raw=true)

 ## Documentation

 [Documentation](https://docs.rs/tui-logger/0.5.0/tui_logger/)

 ## Features

 - [X] Logger implementation for the `log` crate
 - [X] Logger enable/disable detection via hash table (avoid string compare)
 - [X] Hot logger code only copies enabled log messages with timestamp into a circular buffer
 - [X] Widgets/move_message() retrieve captured log messages from hot circular buffer
 - [X] Lost message detection due to circular buffer
 - [X] Log filtering performed on log record target
 - [X] Simple Widgets to view logs and configure debuglevel per target
 - [X] Logging of enabled logs to file
 - [X] `slog` support, providing a Drain to integrate into your `slog` infrastructure
 - [ ] Allow configuration of target dependent loglevel specifically for file logging
 - [ ] Avoid duplicating of target, module and filename in every log record
 - [ ] Simultaneous modification of all targets' display/hot logging loglevel by key command

 ## Smart Widget

 Smart widget consists of two widgets. Left is the target selector widget and
 on the right side the logging messages view scrolling up. The target selector widget
 can be hidden/shown during runtime via key command.
 The key command to be provided to the TuiLoggerWidget via transition() function.

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
 | `h`    | Toggles target selector widget hidden/visible
 | `f`    | Toggle focus on the selected target only
 | `UP`   | Select previous target in target selector widget
 | `DOWN` | Select next target in target selector widget
 | `LEFT` | Reduce SHOWN (!) log messages by one level
 | `RIGHT`| Increase SHOWN (!) log messages by one level
 | `-`    | Reduce CAPTURED (!) log messages by one level
 | `+`    | Increase CAPTURED (!) log messages by one level
 | `SPACE`| Toggles hiding of targets, which have logfilter set to off
  
 The mapping of key to action has to be done in the application. The respective TuiWidgetEvent
 has to be provided to the transition() function of TuiWidgetState

 ## Basic usage to initialize logger-system:
 ```
 #[macro_use]
 extern crate log;
 //use tui_logger;

 fn main() {
     // Early initialization of the logger

     // Set max_log_level to Trace
     tui_logger::init_logger(log::LevelFilter::Trace).unwrap();

     // Set default level for unknown targets to Trace
     tui_logger::set_default_level(log::LevelFilter::Trace);

     // code....
 }
 ```

 For use of the widget please check examples/demo.rs

 ## `slog` support

 `tui-logger` provides a TuiSlogDrain which implements `slog::Drain` and will route all records
 it receives to the `tui-logger` widget

 # Logger with smart widget for the `tui` crate

 [![dependency status](https://deps.rs/repo/github/gin66/tui-logger/status.svg?service=github&nocache=0_6_6)](https://deps.rs/repo/github/gin66/tui-logger)
 ![Build examples](https://github.com/gin66/tui-logger/workflows/Build%20examples/badge.svg?service=github)


 ## Demo of the widget

 ![Demo](https://github.com/gin66/tui-logger/blob/master/doc/demo_v0.6.6.gif?raw=true)

 ## Documentation

 [Documentation](https://docs.rs/tui-logger/latest/tui_logger/)

 ## Features

 - [X] Logger implementation for the `log` crate
 - [X] Logger enable/disable detection via hash table (avoid string compare)
 - [X] Hot logger code only copies enabled log messages with timestamp into a circular buffer
 - [X] Widgets/move_message() retrieve captured log messages from hot circular buffer
 - [X] Lost message detection due to circular buffer
 - [X] Log filtering performed on log record target
 - [X] Simple Widgets to view logs and configure debuglevel per target
 - [X] Logging of enabled logs to file
 - [X] Scrollback in log history
 - [x] Title of target and log pane can be configured
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

 ![widget](https://github.com/gin66/tui-logger/blob/master/doc/example.png?raw=true)

 It controls:

 - Capturing of log messages by the logger
 - Selection of levels for display in the logging message view
  
 The two columns have the following meaning:

 - Code EWIDT: E stands for Error, W for Warn, Info, Debug and Trace.
   + Inverted characters (EWIDT) are enabled log levels in the view
   + Normal characters show enabled capturing of a log level per target
   + If any of EWIDT are not shown, then the respective log level is not captured
 - Target of the log events can be defined in the log e.g. `warn!(target: "demo", "Log message");`
  
 ## Smart Widget Key Commands
 ```
 |  KEY     | ACTION
 |----------|-----------------------------------------------------------|
 | h        | Toggles target selector widget hidden/visible
 | f        | Toggle focus on the selected target only
 | UP       | Select previous target in target selector widget
 | DOWN     | Select next target in target selector widget
 | LEFT     | Reduce SHOWN (!) log messages by one level
 | RIGHT    | Increase SHOWN (!) log messages by one level
 | -        | Reduce CAPTURED (!) log messages by one level
 | +        | Increase CAPTURED (!) log messages by one level
 | PAGEUP   | Enter Page Mode and scroll approx. half page up in log history.
 | PAGEDOWN | Only in page mode: scroll 10 events down in log history.
 | ESCAPE   | Exit page mode and go back to scrolling mode
 | SPACE    | Toggles hiding of targets, which have logfilter set to off
 ```
  
 The mapping of key to action has to be done in the application. The respective TuiWidgetEvent
 has to be provided to TuiWidgetState::transition().

 Remark to the page mode: The timestamp of the event at event history's bottom line is used as
 reference. This means, changing the filters in the EWIDT/focus from the target selector window
 should work as expected without jumps in the history. The page next/forward advances as
 per visibility of the events.

 ## Basic usage to initialize logger-system:
 ```rust
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

 ## Applications using tui-logger

 * [wash](https://github.com/wasmCloud/wash)
 * [rocker](https://github.com/atlassian/rocker)

 ## THANKS TO

 * [Florian Dehau](https://github.com/fdehau) for his great crate [tui-rs](https://github.com/fdehau/tui-rs)
 * [Antoine Büsch](https://github.com/abusch) for providing the patches to tui-rs v0.3.0 and v0.6.0
 * [Adam Sypniewski](https://github.com/ajsyp) for providing the patches to tui-rs v0.6.2
 * [James aka jklong](https://github.com/jklong) for providing the patch to tui-rs v0.7
 * [icy-ux](https://github.com/icy-ux) for adding slog support and example
 * [alvinhochun](https://github.com/alvinhochun) for updating to tui 0.10 and crossterm support
 * [brooksmtownsend](https://github.com/brooksmtownsend) Patch to remove verbose timestamp info
 * [Kibouo](https://github.com/Kibouo) Patch to change Rc/Refcell to thread-safe counterparts
 * [Afonso Bordado](https://github.com/afonso360) for providing the patch to tui-rs v0.17
 * [Benjamin Kampmann](https://github.com/gnunicorn) for providing patch to tui-rs v0.18


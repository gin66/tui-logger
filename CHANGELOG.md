# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.15.0](https://github.com/gin66/tui-logger/compare/v0.14.5...v0.15.0) - 2025-03-08

### Fixed

- fix bug with page up at top

### Other

- lib.rs factoring complete
- refactor TuiLoggerError out of lib.rs
- refactor TuiLogger API out of lib.rs
- refactor TuiLogger out of lib.rs
- refactor LevelConfig out of lib.rs
- refactor TuiLoggerTargetWidget out of lib.rs
- reduce visibility of internal structures
- add test case for log message being larger than widget
- turn off debugging
- add test case with standard formatter
- add test with wrapped lines
- disable println in lib code with render_debug switch
- intermediate version with debug prints
- circular buffer allows absolute indexing
- test cases for standard widget display and scroll

## [0.14.5](https://github.com/gin66/tui-logger/compare/v0.14.4...v0.14.5) - 2025-02-22

### Other

- update Changelog and Readme
- Made target width also use unicode_segmentation
- Use unicode-segmentation for splitting chars
- Split lines safely
- change &mut to & reference in TuiWidgetState.transition as [#83](https://github.com/gin66/tui-logger/pull/83)
- Update mermaid diagram in README.md
- Update lib.rs for readme change
- Update README.md
- Update README.md with mermaid diagram
- rename dev_notes.md
- cleanup files in doc
- udpate readme
- new demo of the widget using vhs
- rename cargo test workflow
- remove stray space in github action

## [0.14.4](https://github.com/gin66/tui-logger/compare/v0.14.3...v0.14.4) - 2025-01-31

### Fixed

- fix cargo test

### Other

- update Readme for custom formatter example in demo
- example demo extended to use custom formatter in one of the widgets
- run cargo test in github actions

## [0.14.3](https://github.com/gin66/tui-logger/compare/v0.14.2...v0.14.3) - 2025-01-31

### Other

- work on Readme and add formatter() to smartwidget
- standard formatter appears to work as before, but using Line/Span
- assure LogFormatter Send+Sync
- implement formatter trait as discussed in [#77](https://github.com/gin66/tui-logger/pull/77) and [#82](https://github.com/gin66/tui-logger/pull/82)

## [0.14.2](https://github.com/gin66/tui-logger/compare/v0.14.1...v0.14.2) - 2025-01-30

### Fixed

- fix warnings

### Other

- split lib.rs into several files
- Merge pull request [#77](https://github.com/gin66/tui-logger/pull/77) from tofubert/add_style_for_file
- Merge pull request [#78](https://github.com/gin66/tui-logger/pull/78) from andrei-ng/fix-order-of-fields-tracing-feature
- Merge pull request [#79](https://github.com/gin66/tui-logger/pull/79) from andrei-ng/skip-printing-message-key

- use env::temp_dir for demo log file target
- do not print the 'message' key in the formatter for tracing support
- fix formatter for tracing events
- make comment for file logging a bit better
- give file logging format options
- Update CHANGELOG.md

0.14.1:
- re-export log::LevelFilter

0.14.0:
- Update version of ratatui

0.13.2:
- fix tracing support

0.13.1:
- fix missing `move_events()` on half full buffer in case hot buffer capacity was odd

0.13.0:
- `move_events()` is not published anymore, but called by a cyclic internal task.
  This task is called by timeout (10ms) unless the hot buffer is half full.
- `init_logger()` returns now `Result<(), TuiLoggerError>`

0.12.1:
- fix for issue #69: avoid unwrap panic by using default level
- add `set_buffer_depth()` to modify circular buffer size

0.12.0:
- update ratatui to 0.28

0.11.2:
- update ratatui to 0.27

0.11.1:
- one line error report for demo example, if feature flag crossterm or termion not given
- use cargo readme and test in github build
- Fix of issue #60: panic on too small widget size

0.11.0:
- BREAKING CHANGE: TuiWidgetEvent::transition() method now takes a TuiWidgetEvent by value instead of by reference.
- update ratatui to 0.25

0.10.1:
- update ratatui to ^0.25.0

0.10.0:
- Remove support for tui legacy crate
- Use `Cell::set_symbol()` as performance optimization from ratatui
- Require chrono >= 0.4.20 for avoid security vulnerability (https://rustsec.org/advisories/RUSTSEC-2020-0159.html)

0.9.6:
- update ratatui to 0.23.0

0.9.5:
- rework examples/demo to not use termion

0.9.4:
- fix breaking change in 0.9.3 as reported by issue #43

0.9.3:
- update to ratatui 0.22 and fix clippy warnings

0.9.2:
- update to ratatui 0.21

0.9.1:
- Implement Eq for TuiWidgetEvent 
- suppport `border_type()` for TuiLoggerSmartWidget
- Disable default features of chrono to avoid import of `time` v0.1.x

0.9.0:
- support for tracing-subscriber
- add optional ratatui support as proposed by (#32)
- slog is NOT a default feature anymore. Enable with `slog-support`

0.8.3:
- Make `TuiWidgetState.set_default_display_level()` work for TuiLoggerWidget (#30)

0.8.2:
- Allow TuiLoggerWidget to be controlled with TuiWidgetState by calling state() builder function (#30)
- Extend demo for an example for this TuiLoggerWidget control

0.8.1:
- Update to tui-rs 0.19 and slog to 2.7.0

0.8.0:
- Update to tui-rs 0.18

0.7.1:
- Update to tui-rs 0.17

0.7.0:
- Update rust edition in Cargo.toml to 2021
- Fix all warnings from cargo clippy
- new function for TuiWidgetState to set the default display level - not impacting the recording
  ```rust
    set_default_display_level(self, levelfilter: LevelFilter) -> TuiWidgetState
- changed signature for TuiWidgetState function from
  ```rust
    set_level_for_target(&self, target: &str, levelfilter: LevelFilter) -> &TuiWidgetState
  ```
  to
  ```rust
    set_level_for_target(self, target: &str, levelfilter: LevelFilter) -> TuiWidgetState
  ```


0.6.6:
- Add functions to format output of log data as discussed in [issue #19](https://github.com/gin66/tui-logger/issues/19)
  The functions are with their default values:
  ```
  output_separator(':')
  output_timestamp(Some("%H:%M:%S".to_string()))
  output_level(Some(TuiLoggerLevelOutput::Long))
  output_target(true)
  output_file(true)
  output_line(true)
  ```

0.6.5:
- Use thread safe counterparts for Rc/RefCell

0.6.4:
- Bump version up for update to tui 0.16

0.6.3:
- Removed verbose timestamp info log (issue #16)

0.6.2:
- Fix by Wuelle to avoid panic on line wrapping inside a utf8 character

0.6.1:
- Changes in README

0.6.0:
- Support Scrollback in log history with TuiWidgetEvent::PrevPageKey, NextPageKey and EscapeKey
- log and target panes' title can be set via .title_log(String) and .title_target(String)

0.5.1:
- TuiWidgetEvent is now Debug, Clone, PartialEq, Hash

0.5.0:
- Remove dispatcher completely
- Get rid of dependency to termion and crossterm
- KeyCommands to be translated by the application into KeyEvents for TuiWidgetState::transition()

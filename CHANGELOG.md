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

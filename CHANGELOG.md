0.6.1:
- remove left over tui::info!() from developing 0.6.0

0.6.0:
- Support Scrollback in log history with TuiWidgetEvent::PrevPageKey, NextPageKey and EscapeKey
- log and target panes' title can be set via .title_log(String) and .title_target(String)

0.5.1:
- TuiWidgetEvent is now Debug, Clone, PartialEq, Hash

0.5.0:
- Remove dispatcher completely
- Get rid of dependency to termion and crossterm
- KeyCommands to be translated by the application into KeyEvents for TuiWidgetState::transition()

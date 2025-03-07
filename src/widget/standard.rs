use crate::widget::logformatter::LogFormatter;
use crate::widget::standard_formatter::LogStandardFormatter;
use parking_lot::Mutex;
use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

use crate::widget::inner::LinePointer;
use crate::{CircularBuffer, ExtLogRecord, TuiLoggerLevelOutput, TuiWidgetState, TUI_LOGGER};

use super::inner::TuiWidgetInnerState;

pub struct TuiLoggerWidget<'b> {
    block: Option<Block<'b>>,
    logformatter: Option<Box<dyn LogFormatter>>,
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
        //TUI_LOGGER.move_events();
        TuiLoggerWidget {
            block: None,
            logformatter: None,
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
    pub fn opt_formatter(mut self, formatter: Option<Box<dyn LogFormatter>>) -> Self {
        self.logformatter = formatter;
        self
    }
    pub fn formatter(mut self, formatter: Box<dyn LogFormatter>) -> Self {
        self.logformatter = Some(formatter);
        self
    }
    pub fn opt_style(mut self, style: Option<Style>) -> Self {
        if let Some(s) = style {
            self.style = s;
        }
        self
    }
    pub fn opt_style_error(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_error = style;
        }
        self
    }
    pub fn opt_style_warn(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_warn = style;
        }
        self
    }
    pub fn opt_style_info(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_info = style;
        }
        self
    }
    pub fn opt_style_trace(mut self, style: Option<Style>) -> Self {
        if style.is_some() {
            self.style_trace = style;
        }
        self
    }
    pub fn opt_style_debug(mut self, style: Option<Style>) -> Self {
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
    pub fn opt_output_separator(mut self, opt_sep: Option<char>) -> Self {
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
    pub fn opt_output_timestamp(mut self, opt_fmt: Option<Option<String>>) -> Self {
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
    pub fn opt_output_level(mut self, opt_fmt: Option<Option<TuiLoggerLevelOutput>>) -> Self {
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
    pub fn opt_output_target(mut self, opt_enabled: Option<bool>) -> Self {
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
    pub fn opt_output_file(mut self, opt_enabled: Option<bool>) -> Self {
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
    pub fn opt_output_line(mut self, opt_enabled: Option<bool>) -> Self {
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
    pub fn inner_state(mut self, state: Arc<Mutex<TuiWidgetInnerState>>) -> Self {
        self.state = state;
        self
    }
    pub fn state(mut self, state: &TuiWidgetState) -> Self {
        self.state = state.inner.clone();
        self
    }
    fn next_event<'a>(
        &self,
        events: &'a CircularBuffer<ExtLogRecord>,
        mut index: usize,
        ignore_current: bool,
        increment: bool,
        state: &TuiWidgetInnerState,
    ) -> Option<(Option<usize>, usize, &'a ExtLogRecord)> {
        // The result is an optional next_index, the event index and the event
        if ignore_current {
            index = if increment {
                index + 1
            } else {
                if index == 0 {
                    return None;
                }
                index - 1
            };
        }
        while let Some(evt) = events.element_at_index(index) {
            let mut skip = false;
            if let Some(level) = state
                .config
                .get(&evt.target)
                .or(state.config.default_display_level)
            {
                if level < evt.level {
                    skip = true;
                }
            }
            if !skip && state.focus_selected {
                if let Some(target) = state.opt_selected_target.as_ref() {
                    if target != &evt.target {
                        skip = true;
                    }
                }
            }
            if skip {
                index = if increment {
                    index + 1
                } else {
                    if index == 0 {
                        break;
                    }
                    index - 1
                };
            } else {
                if increment {
                    return Some((Some(index + 1), index, evt));
                } else {
                    if index == 0 {
                        return Some((None, index, evt));
                    }
                    return Some((Some(index - 1), index, evt));
                };
            }
        }
        None
    }
}
impl<'b> Widget for TuiLoggerWidget<'b> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let formatter = match self.logformatter.take() {
            Some(fmt) => fmt,
            None => {
                let fmt = LogStandardFormatter {
                    style: self.style,
                    style_error: self.style_error,
                    style_warn: self.style_warn,
                    style_debug: self.style_debug,
                    style_trace: self.style_trace,
                    style_info: self.style_info,
                    format_separator: self.format_separator,
                    format_timestamp: self.format_timestamp.clone(),
                    format_output_level: self.format_output_level,
                    format_output_target: self.format_output_target,
                    format_output_file: self.format_output_file,
                    format_output_line: self.format_output_line,
                };
                Box::new(fmt)
            }
        };

        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        if list_area.width < formatter.min_width() || list_area.height < 1 {
            return;
        }

        let mut state = self.state.lock();
        let la_height = list_area.height as usize;
        let la_left = list_area.left();
        let la_top = list_area.top();
        let la_width = list_area.width as usize;
        let mut rev_lines: Vec<(LinePointer, Line)> = vec![];
        let mut can_scroll_up = true;
        let mut can_scroll_down = state.opt_line_pointer_center.is_some();
        {
            enum Pos {
                Top,
                Bottom,
                Center(usize),
            }
            let tui_lock = TUI_LOGGER.inner.lock();
            // If scrolling, the opt_line_pointer_center is set.
            // Otherwise we are following the bottom of the events
            let opt_pos_event_index = if let Some(lp) = state.opt_line_pointer_center {
                tui_lock.events.first_index().map(|first_index| {
                    if first_index <= lp.event_index {
                        (Pos::Center(lp.subline), lp.event_index)
                    } else {
                        (Pos::Top, first_index)
                    }
                })
            } else {
                tui_lock
                    .events
                    .last_index()
                    .map(|last_index| (Pos::Bottom, last_index))
            };
            if let Some((pos, mut event_index)) = opt_pos_event_index {
                // There are events to be shown
                match pos {
                    Pos::Center(subline) => {
                        println!("CENTER {}", event_index);
                        if let Some((_, evt_index, evt)) =
                            self.next_event(&tui_lock.events, event_index, false, true, &state)
                        {
                            let mut lines: Vec<(usize, Vec<Line>, usize)> = Vec::new();
                            let evt_lines = formatter.format(la_width, evt);
                            let mut from_line: isize = (la_height / 2) as isize - subline as isize;
                            let mut to_line = la_height / 2 + (evt_lines.len() - 1) - subline;
                            let n = evt_lines.len();
                            lines.push((evt_index, evt_lines, n));
                            println!("Center is {}", evt_index);

                            let mut cont = true;
                            while cont {
                                println!("from_line {}, to_line {}", from_line, to_line);
                                cont = false;
                                if from_line > 0 {
                                    if let Some((_, evt_index, evt)) = self.next_event(
                                        &tui_lock.events,
                                        lines.first().as_ref().unwrap().0,
                                        true,
                                        false,
                                        &state,
                                    ) {
                                        let mut evt_lines = formatter.format(la_width, evt);
                                        from_line -= evt_lines.len() as isize;
                                        let n = evt_lines.len();
                                        lines.insert(0, (evt_index, evt_lines, n));
                                        cont = true;
                                    }
                                    else {
                                        // no more events, so adjust start
                                        println!("no more events adjust start");
                                        to_line = to_line - from_line as usize;
                                        from_line = 0;
                                    }
                                }
                                if to_line < la_height-1 {
                                    if let Some((_, evt_index, evt)) = self.next_event(
                                        &tui_lock.events,
                                        event_index,
                                        true,
                                        true,
                                        &state,
                                    ) {
                                        let mut evt_lines = formatter.format(la_width, evt);
                                        to_line += evt_lines.len();
                                        let n = evt_lines.len();
                                        lines.push((evt_index, evt_lines, n));
                                        cont = true;
                                    }
                                    else {
                                        println!("no more events at end");
                                        // no more events
                                        if !cont {
                                            // no more lines can be added at start
                                            break;
                                        }
                                        // no more events, so adjust end
                                        from_line = from_line + (la_height - 1 - to_line) as isize;
                                        to_line = la_height - 1;
                                    }
                                }
                            }
                            println!("finished: from_line {}, to_line {}", from_line, to_line);
                            while from_line < 0 {
                                lines[0].1.remove(0);
                                from_line += 1;
                            }
                            while to_line >= la_height {
                                let n = lines.len() - 1;
                                lines[n].1.pop();
                                to_line -= 1;
                            }
                            while let Some((evt_index, evt_lines, mut n)) = lines.pop() {
                                for line in evt_lines {
                                    n -= 1;
                                    let line_ptr = LinePointer {
                                        event_index: evt_index,
                                        subline: n,
                                    };
                                    rev_lines.push((line_ptr, line));
                                }
                            }
                        }
                    }
                    Pos::Top => {
                        can_scroll_up = false;
                    }
                    Pos::Bottom => {
                        // Fill up with lines until the top is reached aka sufficient lines in the buffer or no more events
                        'outer: while let Some((opt_next_index, evt_index, evt)) =
                            self.next_event(&tui_lock.events, event_index, false, false, &state)
                        {
                            let mut evt_lines = formatter.format(la_width, evt);
                            while let Some(line) = evt_lines.pop() {
                                let line_ptr = LinePointer {
                                    event_index: evt_index,
                                    subline: evt_lines.len(),
                                };
                                rev_lines.push((line_ptr, line));
                                if rev_lines.len() >= la_height {
                                    break 'outer;
                                }
                            }
                            if let Some(next_index) = opt_next_index {
                                event_index = next_index;
                            } else {
                                break;
                            }
                        }
                    }
                }
            } else {
                can_scroll_down = false;
                can_scroll_up = false;
            }
        }

        state.opt_line_pointer_next_page = if can_scroll_down {
            rev_lines.first().map(|l| l.0)
        } else {
            None
        };
        state.opt_line_pointer_prev_page = if can_scroll_up {
            rev_lines.last().map(|l| l.0)
        } else {
            None
        };

        if true {
            println!("Line pointers in buffer:");
            for l in rev_lines.iter().rev() {
                println!("event_index {}, subline {}", l.0.event_index, l.0.subline);
            }
            if state.opt_line_pointer_center.is_some() {
                println!(
                    "Linepointer center: {:?}",
                    state.opt_line_pointer_center.unwrap()
                );
            }
            if state.opt_line_pointer_next_page.is_some() {
                println!(
                    "Linepointer next: {:?}",
                    state.opt_line_pointer_next_page.unwrap()
                );
            }
            if state.opt_line_pointer_prev_page.is_some() {
                println!(
                    "Linepointer prev: {:?}",
                    state.opt_line_pointer_prev_page.unwrap()
                );
            }
        }

        // This apparently ensures, that the log starts at top
        let offset: u16 = if state.opt_line_pointer_center.is_none() {
            0
        } else {
            let lines_cnt = rev_lines.len();
            std::cmp::max(0, la_height - lines_cnt) as u16
        };

        for (i, line) in rev_lines.into_iter().rev().take(la_height).enumerate() {
            line.1.render(
                Rect {
                    x: la_left,
                    y: la_top + i as u16 + offset,
                    width: list_area.width,
                    height: 1,
                },
                buf,
            )
        }
    }
}

use crate::widget::logformatter::LogFormatter;
use crate::ExtLogRecord;
use crate::Style;
use crate::TuiLoggerLevelOutput;
use ratatui::text::{Line, Span};
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct LogStandardFormatter {
    /// Base style of the widget
    pub style: Style,
    /// Level based style
    pub style_error: Option<Style>,
    pub style_warn: Option<Style>,
    pub style_debug: Option<Style>,
    pub style_trace: Option<Style>,
    pub style_info: Option<Style>,
    pub format_separator: char,
    pub format_timestamp: Option<String>,
    pub format_output_level: Option<TuiLoggerLevelOutput>,
    pub format_output_target: bool,
    pub format_output_file: bool,
    pub format_output_line: bool,
}

impl LogStandardFormatter {
    fn append_line(&self, lines: &mut Vec<Line>, style: Style, need_indent: bool, space: &str, subline: &mut Vec<&str>) {
        	let mut spans: Vec<Span> = Vec::new();
                if need_indent {
                    // need indent
                    spans.push(Span {
                        style,
                        content: Cow::Owned(space.to_string()),
                    });
                }
                spans.push(Span {
                    style,
                    content: Cow::Owned(subline.drain(..).map(|x| x.to_string()).collect()),
                });
                let line = Line::from(spans);
                lines.push(line);
    }
    fn append_wrapped_line(
        &self,
        style: Style,
        indent: usize,
        lines: &mut Vec<Line>,
        line: &str,
        width: usize,
        with_indent: bool,
    ) {
        let mut wrap_len = width;
        if with_indent {
            wrap_len -= indent;
        }
        let space = " ".repeat(indent);
        let line_chars = line.graphemes(true);
        let lc_with_length = line_chars.map(|ch| (UnicodeWidthStr::width(ch), ch));
        // unicode characters may have different printable lenghts.
        // A loop is needed to fill up the current line till the limit
        let mut p = 0;
        let mut subline: Vec<&str> = vec![];
        let mut need_indent = false;
        for (w, ch) in lc_with_length {
            if p + w > wrap_len {
                self.append_line(lines, style, need_indent, &space, &mut subline);
                p = 0;
                // following lines need to be indented
                wrap_len = width - indent;
                need_indent = true;
            }
            subline.push(ch);
            p += w;
        }
        if p > 0 {
	   self.append_line(lines, style, need_indent, &space, &mut subline);
	}
    }
}

impl LogFormatter for LogStandardFormatter {
    fn min_width(&self) -> u16 {
        9 + 4
    }
    fn format(&self, width: usize, evt: &ExtLogRecord) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut output = String::new();
        let (col_style, lev_long, lev_abbr, with_loc) = match evt.level {
            log::Level::Error => (self.style_error, "ERROR", "E", true),
            log::Level::Warn => (self.style_warn, "WARN ", "W", true),
            log::Level::Info => (self.style_info, "INFO ", "I", true),
            log::Level::Debug => (self.style_debug, "DEBUG", "D", true),
            log::Level::Trace => (self.style_trace, "TRACE", "T", true),
        };
        let col_style = col_style.unwrap_or(self.style);
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
        let mut sublines: Vec<&str> = evt.msg.lines().rev().collect();

        output.push_str(sublines.pop().unwrap());
        self.append_wrapped_line(col_style, 9, &mut lines, &output, width, false);

        for subline in sublines.iter().rev() {
            self.append_wrapped_line(col_style, 9, &mut lines, subline, width, true);
        }
        lines
    }
}

use crate::widget::logformatter::LogFormatter;
use crate::ExtLogRecord;
use crate::Style;
use crate::TuiLoggerLevelOutput;
use ratatui::text::{Line, Span};
use std::borrow::Cow;

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
    fn append_wrapped_line(
        &self,
        style: Style,
        indent: usize,
        lines: &mut Vec<Line>,
        line: &str,
        width: usize,
        with_indent: bool,
    ) {
        let mut p = 0;
        let mut wrap_len = width;
        if with_indent {
            wrap_len -= indent;
        }
        let space = " ".repeat(indent);
        while p < line.len() {
            let linelen = std::cmp::min(wrap_len, line.len() - p);
            let subline = line
                .chars()
                .skip(p)
                .take(linelen)
                .map(|char| {
                    let mut buf = [0; 4];
                    char.encode_utf8(&mut buf).to_string()
                })
                .collect::<String>();

            let mut spans: Vec<Span> = Vec::new();
            if wrap_len < width {
                // need indent
                spans.push(Span {
                    style,
                    content: Cow::Owned(space.to_string()),
                });
            }
            spans.push(Span {
                style,
                content: Cow::Owned(subline),
            });
            let line = Line::from(spans);
            lines.push(line);

            p += linelen;
            // following lines need to be indented
            wrap_len = width - indent;
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

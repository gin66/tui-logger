use crate::ExtLogRecord;
use crate::Style;
use crate::TuiLoggerLevelOutput;

pub trait LogFormatter: Send + Sync {
    fn format(&self, evt: &ExtLogRecord) -> (String, Option<Style>);
}

pub struct LogStandardFormatter {
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

impl LogFormatter for LogStandardFormatter {
    fn format(&self, evt: &ExtLogRecord) -> (String, Option<Style>) {
        let mut output = String::new();
        let (col_style, lev_long, lev_abbr, with_loc) = match evt.level {
            log::Level::Error => (self.style_error, "ERROR", "E", true),
            log::Level::Warn => (self.style_warn, "WARN ", "W", true),
            log::Level::Info => (self.style_info, "INFO ", "I", true),
            log::Level::Debug => (self.style_debug, "DEBUG", "D", true),
            log::Level::Trace => (self.style_trace, "TRACE", "T", true),
        };
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
        (output, col_style)
    }
}

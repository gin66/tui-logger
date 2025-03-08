use std::fs::{File, OpenOptions};

use crate::logger::logger::TuiLoggerLevelOutput;

/// This closely follows the options of [``TuiLoggerSmartWidget``] but is used of logging to a file.
pub struct TuiLoggerFile {
    pub(crate) dump: File,
    pub(crate) format_separator: char,
    pub(crate) timestamp_fmt: Option<String>,
    pub(crate) format_output_target: bool,
    pub(crate) format_output_file: bool,
    pub(crate) format_output_line: bool,
    pub(crate) format_output_level: Option<TuiLoggerLevelOutput>,
}

impl TuiLoggerFile {
    pub fn new(fname: &str) -> Self {
        TuiLoggerFile {
            dump: OpenOptions::new()
                .create(true)
                .append(true)
                .open(fname)
                .expect("Failed to open dump File"),
            format_separator: ':',
            timestamp_fmt: Some("[%Y:%m:%d %H:%M:%S]".to_string()),
            format_output_file: true,
            format_output_line: true,
            format_output_target: true,
            format_output_level: Some(TuiLoggerLevelOutput::Long),
        }
    }
    pub fn output_target(mut self, enabled: bool) -> Self {
        self.format_output_target = enabled;
        self
    }
    pub fn output_file(mut self, enabled: bool) -> Self {
        self.format_output_file = enabled;
        self
    }
    pub fn output_line(mut self, enabled: bool) -> Self {
        self.format_output_line = enabled;
        self
    }
    pub fn output_timestamp(mut self, fmt: Option<String>) -> Self {
        self.timestamp_fmt = fmt;
        self
    }
    pub fn output_separator(mut self, sep: char) -> Self {
        self.format_separator = sep;
        self
    }
    pub fn output_level(mut self, level: Option<TuiLoggerLevelOutput>) -> Self {
        self.format_output_level = level;
        self
    }
}

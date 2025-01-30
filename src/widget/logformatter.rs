use crate::ExtLogRecord;
use ratatui::text::Line;

pub trait LogFormatter: Send + Sync {
    fn min_width(&self) -> u16;

    /// This must format any event in one or more lines.
    /// Correct wrapping in next line with/without indenting must be performed here.
    /// The parameter width is the available line width
    fn format(&self, width: usize, evt: &ExtLogRecord) -> Vec<Line>;
}

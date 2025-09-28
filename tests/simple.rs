use log::*;
use ratatui::text::{Line, Span};
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    Terminal,
};
use std::borrow::Cow;
use tui_logger::*;

pub struct TestFormatter {}
impl LogFormatter for TestFormatter {
    fn min_width(&self) -> u16 {
        1
    }
    fn format(&self, _width: usize, _evt: &ExtLogRecord) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let mut spans: Vec<Span> = Vec::new();
        let style = Style::new().reversed();
        spans.push(Span {
            style,
            content: Cow::Owned("Hello".to_string()),
        });
        let line = Line::from(spans);
        lines.push(line);
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the functions from the parent module

    #[test]
    fn test_simple() {
        init_logger(LevelFilter::Trace).unwrap();
        set_default_level(LevelFilter::Trace);

        info!("1");
        move_events();

        let backend = TestBackend::new(10, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let tui_logger_widget =
                    TuiLoggerWidget::default().formatter(Box::new(TestFormatter {}));
                f.render_widget(
                    tui_logger_widget,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 10,
                        height: 1,
                    },
                );
            })
            .unwrap();
        let mut expected = Buffer::with_lines(["Hello     "]);
        expected.set_style(Rect::new(0, 0, 5, 1), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);
    }
}

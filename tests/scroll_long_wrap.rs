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
use std::{thread, time};
use tui_logger::*;

pub struct TestFormatter {}
impl LogFormatter for TestFormatter {
    fn min_width(&self) -> u16 {
        1
    }
    fn format(&self, _width: usize, evt: &ExtLogRecord) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let style = Style::new().reversed();
        let msg = evt.msg().lines().rev().collect::<Vec<&str>>().join(" ");

        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span {
            style,
            content: Cow::Owned(format!("Hello {}", msg)),
        });
        let line = Line::from(spans);
        lines.push(line);

        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span {
            style,
            content: Cow::Owned(format!(" wrap {} 1", msg)),
        });
        let line = Line::from(spans);
        lines.push(line);

        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span {
            style,
            content: Cow::Owned(format!(" wrap {} 2", msg)),
        });
        let line = Line::from(spans);
        lines.push(line);

        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span {
            style,
            content: Cow::Owned(format!(" wrap {} 3", msg)),
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
    fn test_scroll() {
        init_logger(LevelFilter::Trace).unwrap();
        set_default_level(LevelFilter::Trace);

        let state = TuiWidgetState::new();

        info!("0");
        thread::sleep(time::Duration::from_millis(10));
        info!("1");
        thread::sleep(time::Duration::from_millis(10));
        info!("2");
        thread::sleep(time::Duration::from_millis(10));
        move_events();

        println!("Initial draw");
        let backend = TestBackend::new(10, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let tui_logger_widget = TuiLoggerWidget::default()
                    .formatter(Box::new(TestFormatter {}))
                    .state(&state);
                f.render_widget(
                    tui_logger_widget,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 10,
                        height: 3,
                    },
                );
            })
            .unwrap();
        let mut expected = Buffer::with_lines([" wrap 2 1 ", " wrap 2 2 ", " wrap 2 3 "]);
        expected.set_style(Rect::new(0, 0, 9, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);

        println!("Scroll up");
        state.transition(TuiWidgetEvent::PrevPageKey);

        terminal
            .draw(|f| {
                let tui_logger_widget = TuiLoggerWidget::default()
                    .formatter(Box::new(TestFormatter {}))
                    .state(&state);
                f.render_widget(
                    tui_logger_widget,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 10,
                        height: 3,
                    },
                );
            })
            .unwrap();
        expected = Buffer::with_lines(["Hello 2   ", " wrap 2 1 ", " wrap 2 2 "]);
        expected.set_style(Rect::new(0, 0, 7, 1), Style::new().reversed());
        expected.set_style(Rect::new(0, 1, 9, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);

        println!("Scroll up");
        state.transition(TuiWidgetEvent::PrevPageKey);

        terminal
            .draw(|f| {
                let tui_logger_widget = TuiLoggerWidget::default()
                    .formatter(Box::new(TestFormatter {}))
                    .state(&state);
                f.render_widget(
                    tui_logger_widget,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 10,
                        height: 3,
                    },
                );
            })
            .unwrap();
        expected = Buffer::with_lines([" wrap 1 3 ", "Hello 2   ", " wrap 2 1 "]);
        expected.set_style(Rect::new(0, 0, 9, 1), Style::new().reversed());
        expected.set_style(Rect::new(0, 1, 7, 2), Style::new().reversed());
        expected.set_style(Rect::new(0, 2, 9, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);
    }
}

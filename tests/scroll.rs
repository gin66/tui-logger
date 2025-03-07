// src/lib.rs
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
    fn format(&self, _width: usize, evt: &ExtLogRecord) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut spans: Vec<Span> = Vec::new();
        let style = Style::new().reversed();
        let msg = evt.msg.lines().rev().collect::<Vec<&str>>().join(" ");
        spans.push(Span {
            style,
            content: Cow::Owned(format!("Hello {}", msg)),
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
        info!("3");
        thread::sleep(time::Duration::from_millis(10));
        info!("4");
        thread::sleep(time::Duration::from_millis(10));
        info!("5");
        thread::sleep(time::Duration::from_millis(10));
        info!("6");
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
        let mut expected = Buffer::with_lines(["Hello 4   ", "Hello 5   ", "Hello 6   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
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
        expected = Buffer::with_lines(["Hello 3   ", "Hello 4   ", "Hello 5   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
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
        expected = Buffer::with_lines(["Hello 2   ", "Hello 3   ", "Hello 4   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
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
        expected = Buffer::with_lines(["Hello 1   ", "Hello 2   ", "Hello 3   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);

        println!("Scroll down");
        state.transition(TuiWidgetEvent::NextPageKey);

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
        expected = Buffer::with_lines(["Hello 2   ", "Hello 3   ", "Hello 4   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);

        println!("Scroll down");
        state.transition(TuiWidgetEvent::NextPageKey);

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
        expected = Buffer::with_lines(["Hello 3   ", "Hello 4   ", "Hello 5   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);

        println!("Scroll down");
        state.transition(TuiWidgetEvent::NextPageKey);

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
        expected = Buffer::with_lines(["Hello 4   ", "Hello 5   ", "Hello 6   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);
 
        println!("Scroll down at bottom");
        state.transition(TuiWidgetEvent::NextPageKey);

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
        expected = Buffer::with_lines(["Hello 4   ", "Hello 5   ", "Hello 6   "]);
        expected.set_style(Rect::new(0, 0, 7, 3), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);
    }
}

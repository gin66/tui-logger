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
use tui_logger::*;

#[cfg(test)]
mod tests {
    use super::*; // Import the functions from the parent module

    #[test]
    fn test_simple() {
        init_logger(LevelFilter::Trace).unwrap();
        set_default_level(LevelFilter::Trace);

        info!("Message");
        move_events();

        let backend = TestBackend::new(40, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let tui_logger_widget =
                    TuiLoggerWidget::default()
                    .output_timestamp(None);
                f.render_widget(
                    tui_logger_widget,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 40,
                        height: 3,
                    },
                );
            })
            .unwrap();
        let mut expected = Buffer::with_lines([
            "INFO :formatter_wrap::tests:tests/format",
            "         ter_wrap.rs:23:Message         ",
            "                                        ",
        ]);
        //expected.set_style(Rect::new(0, 0, 40, 2), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);
    }
}

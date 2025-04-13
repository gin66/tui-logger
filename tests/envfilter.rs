use log::*;
use ratatui::{backend::TestBackend, buffer::Buffer, layout::Rect, Terminal};
use tui_logger::*;

#[cfg(test)]
mod tests {
    use super::*; // Import the functions from the parent module

    #[test]
    fn test_formatter() {
        init_logger(LevelFilter::Trace).unwrap();
        set_default_level(LevelFilter::Off);

        warn!("Message"); // This is suppressed due to LevelFilter::Off
        move_events();
        set_env_filter_from_string("envfilter=info");
        warn!("Message");
        move_events();
        info!("Message");
        move_events();
        remove_env_filter(); // Ensure the level has been stored in the hot_select hashtable
        info!("Message"); // Default filter would be Off
        move_events();

        let backend = TestBackend::new(40, 8);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let tui_logger_widget = TuiLoggerWidget::default().output_timestamp(None);
                f.render_widget(
                    tui_logger_widget,
                    Rect {
                        x: 0,
                        y: 0,
                        width: 40,
                        height: 8,
                    },
                );
            })
            .unwrap();
        let expected = Buffer::with_lines([
            "WARN :envfilter::tests:tests/envfilter.r",
            "         s:17:Message                   ",
            "INFO :envfilter::tests:tests/envfilter.r",
            "         s:19:Message                   ",
            "INFO :envfilter::tests:tests/envfilter.r",
            "         s:22:Message                   ",
            "                                        ",
            "                                        ",
        ]);
        //expected.set_style(Rect::new(0, 0, 40, 2), Style::new().reversed());
        terminal.backend().assert_buffer(&expected);
    }
}

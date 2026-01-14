use log::*;
use ratatui::{backend::TestBackend, layout::Rect, Terminal};
use tui_logger::*;

#[test]
fn test_panic_on_empty_log() {
    let _ = init_logger(LevelFilter::Trace);
    set_default_level(LevelFilter::Trace);

    info!("");
    move_events();

    let backend = TestBackend::new(40, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let res = terminal.draw(|f| {
        let tui_logger_widget = TuiLoggerWidget::default().output_timestamp(None);
        f.render_widget(
            tui_logger_widget,
            Rect {
                x: 0,
                y: 0,
                width: 40,
                height: 3,
            },
        );
    });
    assert!(res.is_ok());
}

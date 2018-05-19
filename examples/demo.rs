extern crate termion;
extern crate tui;
extern crate tui_logger;
#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::mpsc;
use std::{thread, time};

use log::LevelFilter;
use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;
use tui_logger::*;

fn main() {
    init_logger(LevelFilter::Trace).unwrap();
    set_default_level(LevelFilter::Trace);
    info!(target:"DEMO", "Start demo");

    let mut terminal = Terminal::new(MouseBackend::new().unwrap()).unwrap();
    let stdin = io::stdin();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let (tx, rx) = mpsc::channel();
    let tx_event = tx.clone();
    thread::spawn(move || {
        for c in stdin.events() {
            trace!(target:"DEMO", "Stdin event received {:?}", c);
            tx_event.send(c.unwrap()).unwrap();
        }
    });
    thread::spawn(move || {
        let one_second = time::Duration::from_millis(1_000);
        loop {
            trace!(target:"DEMO", "Sleep one second");
            thread::sleep(one_second);
            trace!(target:"DEMO", "Issue log entry for each level");
            error!(target:"error", "an error");
            warn!(target:"warn", "a warning");
            trace!(target:"trace", "a trace");
            debug!(target:"debug", "a debug");
            info!(target:"info", "an info");
            tx.send(termion::event::Event::Unsupported(vec![])).unwrap();
        }
    });

    let mut term_size = terminal.size().unwrap();
    let dispatcher = Rc::new(RefCell::new(Dispatcher::<event::Event>::new()));
    let state = TuiWidgetState::new();
    draw(&mut terminal, &term_size, dispatcher.clone(), &state);

    for evt in rx {
        trace!(target: "New event", "{:?}",evt);
        if !dispatcher.borrow_mut().dispatch(&evt) {
            if evt == termion::event::Event::Key(event::Key::Char('q')) {
                break;
            }
        }
        let size = terminal.size().unwrap();
        if term_size != size {
            terminal.resize(size).unwrap();
            term_size = size;
        }
        dispatcher.borrow_mut().clear();
        draw(&mut terminal, &term_size, dispatcher.clone(), &state);
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw(
    t: &mut Terminal<MouseBackend>,
    size: &Rect,
    dispatcher: Rc<RefCell<Dispatcher<event::Event>>>,
    state: &TuiWidgetState,
) {
    Block::default().borders(Borders::ALL).render(t, size);
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, size, |t, chunks| {
            TuiLoggerSmartWidget::default()
                .block(
                    Block::default()
                        .title("Tui Logger")
                        .border_style(Style::default().fg(Color::Black))
                        .borders(Borders::ALL),
                )
                .style_error(Style::default().fg(Color::Red))
                .style_debug(Style::default().fg(Color::Green))
                .style_warn(Style::default().fg(Color::Yellow))
                .style_trace(Style::default().fg(Color::Magenta))
                .style_info(Style::default().fg(Color::Cyan))
                .state(state)
                .dispatcher(dispatcher)
                .render(t, &chunks[0]);
            TuiLoggerWidget::default()
                .block(
                    Block::default()
                        .title("Independent Tui Logger View with colors")
                        .border_style(Style::default().fg(Color::Black))
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White).bg(Color::Blue))
                .render(t, &chunks[1]);
        });

    t.draw().unwrap();
}

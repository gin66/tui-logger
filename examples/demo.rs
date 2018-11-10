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
use termion::event::{self, Key};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::Frame;
use tui::Terminal;
use tui_logger::*;

struct App {
    states: Vec<TuiWidgetState>,
    dispatcher: Rc<RefCell<Dispatcher<event::Event>>>,
    selected_tab: Rc<RefCell<usize>>,
}

fn main() {
    init_logger(LevelFilter::Trace).unwrap();
    set_default_level(LevelFilter::Trace);
    info!(target:"DEMO", "Start demo");

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
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
    let mut app = App {
        states: vec![],
        dispatcher: Rc::new(RefCell::new(Dispatcher::<event::Event>::new())),
        selected_tab: Rc::new(RefCell::new(0)),
    };
    draw(&mut terminal, term_size, &mut app);

    // Here is the main loop
    for evt in rx {
        trace!(target: "New event", "{:?}",evt);
        if !app.dispatcher.borrow_mut().dispatch(&evt) {
            if evt == termion::event::Event::Key(event::Key::Char('q')) {
                break;
            }
        }
        let size = terminal.size().unwrap();
        if term_size != size {
            terminal.resize(size).unwrap();
            term_size = size;
        }
        app.dispatcher.borrow_mut().clear();
        draw(&mut terminal, term_size, &mut app);
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw<B: Backend>(t: &mut Terminal<B>, size: Rect, app: &mut App) {
    t.draw(|mut f| {
        draw_frame(&mut f, size, app);
    }).unwrap();
}

fn draw_frame<B: Backend>(t: &mut Frame<B>, size: Rect, app: &mut App) {
    let tabs = vec!["V1", "V2", "V3", "V4"];
    let sel = *app.selected_tab.borrow();
    let sel_tab = if sel + 1 < tabs.len() { sel + 1 } else { 0 };
    let sel_stab = if sel > 0 { sel - 1 } else { tabs.len() - 1 };
    let v_sel = app.selected_tab.clone();

    // Switch between tabs via Tab and Shift-Tab
    // At least on my computer the 27/91/90 equals a Shift-Tab
    app.dispatcher.borrow_mut().add_listener(move |evt| {
        if &event::Event::Unsupported(vec![27, 91, 90]) == evt {
            *v_sel.borrow_mut() = sel_stab;
            true
        } else if &event::Event::Key(Key::Char('\t')) == evt {
            *v_sel.borrow_mut() = sel_tab;
            true
        } else {
            false
        }
    });
    if app.states.len() <= sel {
        app.states.push(TuiWidgetState::new());
    }

    Block::default().borders(Borders::ALL).render(t, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(size);

    Tabs::default()
        .block(Block::default().borders(Borders::ALL))
        .titles(&tabs)
        .highlight_style(Style::default().modifier(Modifier::Invert))
        .select(sel)
        .render(t, chunks[0]);
    TuiLoggerSmartWidget::default()
        .border_style(Style::default().fg(Color::Black))
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .state(&mut app.states[sel])
        .dispatcher(app.dispatcher.clone())
        .render(t, chunks[1]);
    TuiLoggerWidget::default()
        .block(
            Block::default()
                .title("Independent Tui Logger View")
                .title_style(Style::default().fg(Color::White).bg(Color::Black))
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .render(t, chunks[2]);
}

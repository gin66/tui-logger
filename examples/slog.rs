//! Demo of slog-based logging
//!

use slog::{debug, error, info, o, trace, warn, Drain, Logger};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::mpsc;
use std::{thread, time};

use termion::event::{self, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, Tabs};
use tui::Frame;
use tui::Terminal;
use tui::text::Spans;
use tui_logger::*;

struct App {
    states: Vec<TuiWidgetState>,
    dispatcher: Rc<RefCell<Dispatcher<event::Event>>>,
    selected_tab: Rc<RefCell<usize>>,
    opt_info_cnt: Option<u16>,
}

#[derive(Debug)]
enum AppEvent {
    Termion(termion::event::Event),
    LoopCnt(Option<u16>),
}

fn demo_application(log: Logger, tx: mpsc::Sender<AppEvent>) {
    let one_second = time::Duration::from_millis(1_000);
    let mut lp_cnt = (1..=100).into_iter();
    loop {
        trace!(log, "Sleep one second");
        thread::sleep(one_second);
        trace!(log, "Issue log entry for each level");
        error!(log, "an error");
        warn!(log, "a warning");
        trace!(log, "a trace");
        debug!(log, "a debug");
        info!(log, "an info");
        tx.send(AppEvent::LoopCnt(lp_cnt.next())).unwrap();
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let drain = tui_logger::slog_drain().fuse();
    let log = slog::Logger::root(drain, o!());
    info!(log, "Start demo");

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let stdin = io::stdin();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    // Use an mpsc::channel to combine stdin events with app events
    let (tx, rx) = mpsc::channel();
    let tx_event = tx.clone();
    let log_thread = log.clone();
    thread::spawn(move || {
        for c in stdin.events() {
            trace!(log_thread, "Stdin event received {:?}", c);
            tx_event.send(AppEvent::Termion(c.unwrap())).unwrap();
        }
    });
    let log_thread = log.clone();
    thread::spawn(move || {
        demo_application(log_thread, tx);
    });

    let mut app = App {
        states: vec![],
        dispatcher: Rc::new(RefCell::new(Dispatcher::<event::Event>::new())),
        selected_tab: Rc::new(RefCell::new(0)),
        opt_info_cnt: None,
    };

    let log_thread = log.clone();
    // Here is the main loop
    for evt in rx {
        trace!(log_thread, "{:?}", evt);
        match evt {
            AppEvent::Termion(evt) => {
                if !app.dispatcher.borrow_mut().dispatch(&evt) {
                    if evt == termion::event::Event::Key(event::Key::Char('q')) {
                        break;
                    }
                }
            }
            AppEvent::LoopCnt(opt_cnt) => {
                app.opt_info_cnt = opt_cnt;
            }
        }
        terminal.draw(|mut f| {
            let size = f.size();
            draw_frame(&mut f, size, &mut app);
        })?;
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();

    Ok(())
}

fn draw_frame<B: Backend>(t: &mut Frame<B>, size: Rect, app: &mut App) {
    let tabs = vec!["V1", "V2", "V3", "V4"];
    let tabs = tabs.into_iter().map(|t| Spans::from(t)).collect::<Vec<_>>();
    let sel = *app.selected_tab.borrow();
    let sel_tab = if sel + 1 < tabs.len() { sel + 1 } else { 0 };
    let sel_stab = if sel > 0 { sel - 1 } else { tabs.len() - 1 };
    let v_sel = app.selected_tab.clone();

    // Switch between tabs via Tab and Shift-Tab
    // At least on my computer the 27/91/90 equals a Shift-Tab
    app.dispatcher.borrow_mut().clear();
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

    let block = Block::default().borders(Borders::ALL);
    t.render_widget(block, size);

    let mut constraints = vec![
        Constraint::Length(3),
        Constraint::Percentage(50),
        Constraint::Min(3),
    ];
    if app.opt_info_cnt.is_some() {
        constraints.push(Constraint::Length(3));
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(size);

    let tabs = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .select(sel);
    t.render_widget(tabs, chunks[0]);

    let tui_sm = TuiLoggerSmartWidget::default()
        .border_style(Style::default().fg(Color::Black))
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .state(&mut app.states[sel])
        .dispatcher(app.dispatcher.clone());
    t.render_widget(tui_sm, chunks[1]);
    let tui_w: TuiLoggerWidget = TuiLoggerWidget::default()
        .block(
            Block::default()
                .title("Independent Tui Logger View")
                //.title_style(Style::default().fg(Color::White).bg(Color::Black))
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black));
    t.render_widget(tui_w, chunks[2]);
    if let Some(percent) = app.opt_info_cnt {
        let guage = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(percent);
        t.render_widget(guage, chunks[3]);
    }
}

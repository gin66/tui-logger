use std::io;
use std::sync::mpsc;
use std::{thread, time};

use log::LevelFilter;
use log::*;

#[cfg(feature = "crossterm")]
use crossterm::event::KeyCode as Key;
#[cfg(feature = "crossterm")]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[cfg(feature = "termion")]
use termion::{
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};

#[cfg(feature = "examples-ratatui-crossterm")]
use ratatui::backend::CrosstermBackend as SelectedBackend;
#[cfg(feature = "examples-ratatui-termion")]
use ratatui::backend::TermionBackend as SelectedBackend;
use ratatui::prelude::*;
use ratatui::widgets::*;

use tui_logger::*;

struct App {
    states: Vec<TuiWidgetState>,
    tabs: Vec<String>,
    selected_tab: usize,
    opt_info_cnt: Option<u16>,
}

#[derive(Debug)]
enum AppEvent {
    UiEvent(Event),
    LoopCnt(Option<u16>),
}

fn demo_application(tx: mpsc::Sender<AppEvent>) {
    let one_second = time::Duration::from_millis(1_000);
    let mut lp_cnt = (1..=100).into_iter();
    loop {
        trace!(target:"DEMO", "Sleep one second");
        thread::sleep(one_second);
        trace!(target:"DEMO", "Issue log entry for each level");
        error!(target:"error", "an error");
        warn!(target:"warn", "a warning");
        trace!(target:"trace", "a trace");
        debug!(target:"debug", "a debug");
        info!(target:"info", "an info");
        tx.send(AppEvent::LoopCnt(lp_cnt.next())).unwrap();
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    init_logger(LevelFilter::Trace).unwrap();
    set_default_level(LevelFilter::Trace);
    info!(target:"DEMO", "Start demo");

    #[cfg(feature = "termion")]
    let backend = {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        SelectedBackend::new(stdout)
    };

    #[cfg(feature = "crossterm")]
    let backend = {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        SelectedBackend::new(stdout)
    };

    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    // Use an mpsc::channel to combine stdin events with app events
    let (tx, rx) = mpsc::channel();
    let tx_event = tx.clone();
    thread::spawn({
        let f = {
            move || {
                #[cfg(feature = "termion")]
                for c in io::stdin().events() {
                    trace!(target:"DEMO", "Stdin event received {:?}", c);
                    tx_event.send(AppEvent::UiEvent(c.unwrap())).unwrap();
                }
                #[cfg(feature = "crossterm")]
                while let Ok(c) = event::read() {
                    trace!(target:"DEMO", "Stdin event received {:?}", c);
                    tx_event.send(AppEvent::UiEvent(c)).unwrap();
                }
            }
        };
        f
    });
    thread::spawn(move || {
        demo_application(tx);
    });

    let mut app = App {
        states: vec![],
        tabs: vec!["V1".into(), "V2".into(), "V3".into(), "V4".into()],
        selected_tab: 0,
        opt_info_cnt: None,
    };

    // Here is the main loop
    for evt in rx {
        let opt_state = if app.selected_tab < app.states.len() {
            Some(&mut app.states[app.selected_tab])
        } else {
            None
        };
        match evt {
            AppEvent::UiEvent(evt) => {
                debug!(target: "New event", "{:?}",evt);
                if let Event::Key(key) = evt {
                    if let Some(state) = opt_state {
                        #[cfg(feature = "crossterm")]
                        let code = key.code;
                        #[cfg(feature = "termion")]
                        let code = key;

                        match code {
                            Key::Char('q') => break,
                            Key::Char('\t') => {
                                // tab
                                let sel = app.selected_tab;
                                let sel_tab = if sel + 1 < app.tabs.len() { sel + 1 } else { 0 };
                                app.selected_tab = sel_tab;
                            }
                            Key::Char(' ') => {
                                state.transition(&TuiWidgetEvent::SpaceKey);
                            }
                            Key::Esc => {
                                state.transition(&TuiWidgetEvent::EscapeKey);
                            }
                            Key::PageUp => {
                                state.transition(&TuiWidgetEvent::PrevPageKey);
                            }
                            Key::PageDown => {
                                state.transition(&TuiWidgetEvent::NextPageKey);
                            }
                            Key::Up => {
                                state.transition(&TuiWidgetEvent::UpKey);
                            }
                            Key::Down => {
                                state.transition(&TuiWidgetEvent::DownKey);
                            }
                            Key::Left => {
                                state.transition(&TuiWidgetEvent::LeftKey);
                            }
                            Key::Right => {
                                state.transition(&TuiWidgetEvent::RightKey);
                            }
                            Key::Char('+') => {
                                state.transition(&TuiWidgetEvent::PlusKey);
                            }
                            Key::Char('-') => {
                                state.transition(&TuiWidgetEvent::MinusKey);
                            }
                            Key::Char('h') => {
                                state.transition(&TuiWidgetEvent::HideKey);
                            }
                            Key::Char('f') => {
                                state.transition(&TuiWidgetEvent::FocusKey);
                            }
                            _ => (),
                        }
                    }
                }
            }
            AppEvent::LoopCnt(opt_cnt) => {
                trace!(target: "New event", "{:?}",evt);
                app.opt_info_cnt = opt_cnt;
            }
        }
        terminal.draw(|mut f| {
            let size = f.size();
            draw_frame(&mut f, size, &mut app);
        })?;
    }
    #[cfg(feature = "crossterm")]
    {
        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();

    Ok(())
}

fn draw_frame<B: Backend>(t: &mut Frame<B>, size: Rect, app: &mut App) {
    let tabs: Vec<Line> = vec!["V1".into(), "V2".into(), "V3".into(), "V4".into()];
    let sel = app.selected_tab;

    if app.states.len() <= sel {
        let tws = TuiWidgetState::new().set_default_display_level(log::LevelFilter::Info);
        app.states.push(tws);
    }

    let block = Block::default().borders(Borders::ALL);
    let inner_area = block.inner(size);
    t.render_widget(block, size);

    let mut constraints = vec![
        Constraint::Length(3),
        Constraint::Percentage(50),
        Constraint::Percentage(30),
        Constraint::Min(10),
    ];
    if app.opt_info_cnt.is_some() {
        constraints.push(Constraint::Length(3));
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    let tabs = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .select(sel);
    t.render_widget(tabs, chunks[0]);

    let tui_sm = TuiLoggerSmartWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .output_separator(':')
        .output_timestamp(Some("%H:%M:%S".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_target(true)
        .output_file(true)
        .output_line(true)
        .state(&mut app.states[sel]);
    t.render_widget(tui_sm, chunks[1]);

    // show two TuiWidgetState side-by-side
    constraints = vec![Constraint::Percentage(50), Constraint::Percentage(30)];
    let hchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(chunks[2]);

    // Example to filter out log entries below Info for targets "trace" and "DEMO"
    // Best to store TuiWidgetState on application level,
    // but this temporary usage as shown here works, too.
    let filter_state = TuiWidgetState::new()
        .set_default_display_level(log::LevelFilter::Off)
        .set_level_for_target("New event", log::LevelFilter::Debug)
        .set_level_for_target("info", log::LevelFilter::Info);
    let tui_w: TuiLoggerWidget = TuiLoggerWidget::default()
        .block(
            Block::default()
                .title("Independent Tui Logger View")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .output_separator('|')
        .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Long))
        .output_target(false)
        .output_file(false)
        .output_line(false)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .state(&filter_state);
    t.render_widget(tui_w, chunks[2]);

    let tui_w: TuiLoggerWidget = TuiLoggerWidget::default()
        .block(
            Block::default()
                .title("Independent Tui Logger View")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .output_separator('|')
        .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Long))
        .output_target(false)
        .output_file(false)
        .output_line(false)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    t.render_widget(tui_w, hchunks[1]);

    if let Some(percent) = app.opt_info_cnt {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(percent);
        t.render_widget(gauge, chunks[3]);
    }
}

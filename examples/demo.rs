use std::{io, sync::mpsc, thread, time};

use log::*;
use ratatui::{prelude::*, widgets::*};
use tui_logger::*;

/// Choose the backend depending on the selected feature (crossterm or termion). This is a mutually
/// exclusive feature, so only one of them can be enabled at a time.
#[cfg(all(feature = "crossterm", not(feature = "termion")))]
use self::crossterm_backend::*;
#[cfg(all(feature = "termion", not(feature = "crossterm")))]
use self::termion_backend::*;
#[cfg(not(any(feature = "crossterm", feature = "termion")))]
compile_error!("One of the features 'crossterm' or 'termion' must be enabled.");
#[cfg(all(feature = "crossterm", feature = "termion"))]
compile_error!("Only one of the features 'crossterm' and 'termion' can be enabled.");

struct App {
    mode: AppMode,
    states: Vec<TuiWidgetState>,
    tab_names: Vec<&'static str>,
    selected_tab: usize,
    progress_counter: Option<u16>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    #[default]
    Run,
    Quit,
}

#[derive(Debug)]
enum AppEvent {
    UiEvent(Event),
    CounterChanged(Option<u16>),
}

fn main() -> anyhow::Result<()> {
    init_logger(LevelFilter::Trace)?;
    set_default_level(LevelFilter::Trace);
    debug!(target:"App", "Logging initialized");

    let mut terminal = init_terminal()?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    App::new().start(&mut terminal)?;

    restore_terminal()?;
    terminal.clear()?;

    Ok(())
}

impl App {
    pub fn new() -> App {
        let states = vec![
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
        ];
        let tab_names = vec!["State 1", "State 2", "State 3", "State 4"];
        App {
            mode: AppMode::Run,
            states,
            tab_names,
            selected_tab: 0,
            progress_counter: None,
        }
    }

    pub fn start(mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        // Use an mpsc::channel to combine stdin events with app events
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        let progress_tx = tx.clone();

        thread::spawn(move || input_thread(event_tx));
        thread::spawn(move || progress_task(progress_tx).unwrap());
        thread::spawn(move || background_task());

        self.run(terminal, rx)
    }

    /// Main application loop
    fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        rx: mpsc::Receiver<AppEvent>,
    ) -> anyhow::Result<()> {
        for event in rx {
            match event {
                AppEvent::UiEvent(event) => self.handle_ui_event(event),
                AppEvent::CounterChanged(value) => self.update_progress_bar(event, value),
            }
            if self.mode == AppMode::Quit {
                break;
            }
            self.draw(terminal)?;
        }
        Ok(())
    }

    fn update_progress_bar(&mut self, event: AppEvent, value: Option<u16>) {
        trace!(target: "App", "Updating progress bar {:?}",event);
        self.progress_counter = value;
        if value.is_none() {
            info!(target: "App", "Background task finished");
        }
    }

    fn handle_ui_event(&mut self, event: Event) {
        debug!(target: "App", "Handling UI event: {:?}",event);
        let state = self.selected_state();

        if let Event::Key(key) = event {
            #[cfg(feature = "crossterm")]
            let code = key.code;

            #[cfg(feature = "termion")]
            let code = key;

            match code.into() {
                Key::Char('q') => self.mode = AppMode::Quit,
                Key::Char('\t') => self.next_tab(),
                #[cfg(feature = "crossterm")]
                Key::Tab => self.next_tab(),
                Key::Char(' ') => state.transition(TuiWidgetEvent::SpaceKey),
                Key::Esc => state.transition(TuiWidgetEvent::EscapeKey),
                Key::PageUp => state.transition(TuiWidgetEvent::PrevPageKey),
                Key::PageDown => state.transition(TuiWidgetEvent::NextPageKey),
                Key::Up => state.transition(TuiWidgetEvent::UpKey),
                Key::Down => state.transition(TuiWidgetEvent::DownKey),
                Key::Left => state.transition(TuiWidgetEvent::LeftKey),
                Key::Right => state.transition(TuiWidgetEvent::RightKey),
                Key::Char('+') => state.transition(TuiWidgetEvent::PlusKey),
                Key::Char('-') => state.transition(TuiWidgetEvent::MinusKey),
                Key::Char('h') => state.transition(TuiWidgetEvent::HideKey),
                Key::Char('f') => state.transition(TuiWidgetEvent::FocusKey),
                _ => (),
            }
        }
    }

    fn selected_state(&mut self) -> &mut TuiWidgetState {
        &mut self.states[self.selected_tab]
    }

    fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.tab_names.len();
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;
        Ok(())
    }
}

/// A simulated task that sends a counter value to the UI ranging from 0 to 100 every second.
fn progress_task(tx: mpsc::Sender<AppEvent>) -> anyhow::Result<()> {
    for progress in 0..100 {
        debug!(target:"progress-task", "Send progress to UI thread. Value: {:?}", progress);
        tx.send(AppEvent::CounterChanged(Some(progress)))?;

        trace!(target:"progress-task", "Sleep one second");
        thread::sleep(time::Duration::from_millis(1000));
    }
    info!(target:"progress-task", "Progress task finished");
    tx.send(AppEvent::CounterChanged(None))?;
    Ok(())
}

/// A background task that logs a log entry for each log level every second.
fn background_task() {
    loop {
        error!(target:"background-task", "an error");
        warn!(target:"background-task", "a warning");
        info!(target:"background-task", "an info");
        debug!(target:"background-task", "a debug");
        trace!(target:"background-task", "a trace");
        thread::sleep(time::Duration::from_millis(1000));
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let progress_height = if self.progress_counter.is_some() {
            3
        } else {
            0
        };
        let [tabs_area, smart_area, main_area, progress_area, help_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(50),
            Constraint::Fill(30),
            Constraint::Length(progress_height),
            Constraint::Length(3),
        ])
        .areas(area);
        // show two TuiWidgetState side-by-side
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(main_area);

        Tabs::new(self.tab_names.iter().cloned())
            .block(Block::default().title("States").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .select(self.selected_tab)
            .render(tabs_area, buf);

        TuiLoggerSmartWidget::default()
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
            .state(self.selected_state())
            .render(smart_area, buf);

        // An example of filtering the log output. The left TuiLoggerWidget is filtered to only show
        // log entries for the "App" target. The right TuiLoggerWidget shows all log entries.
        let filter_state = TuiWidgetState::new()
            .set_default_display_level(LevelFilter::Off)
            .set_level_for_target("App", LevelFilter::Debug)
            .set_level_for_target("background-task", LevelFilter::Info);
        TuiLoggerWidget::default()
            .block(Block::bordered().title("Filtered TuiLoggerWidget"))
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style(Style::default().fg(Color::White))
            .state(&filter_state)
            .render(left, buf);

        TuiLoggerWidget::default()
            .block(Block::bordered().title("Unfiltered TuiLoggerWidget"))
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style(Style::default().fg(Color::White))
            .render(right, buf);

        if let Some(percent) = self.progress_counter {
            Gauge::default()
                .block(Block::bordered().title("progress-task"))
                .gauge_style((Color::White, Modifier::ITALIC))
                .percent(percent)
                .render(progress_area, buf);
        }
        if area.width > 40 {
            Text::from(vec![
                "Q: Quit | Tab: Switch state | ↑/↓: Select target | f: Focus target".into(),
                "←/→: Display level | +/-: Filter level | Space: Toggle hidden targets".into(),
                "h: Hide target selector | PageUp/Down: Scroll | Esc: Cancel scroll".into(),
            ])
            .style(Color::Gray)
            .centered()
            .render(help_area, buf);
        }
    }
}

/// A module for crossterm specific code
#[cfg(feature = "crossterm")]
mod crossterm_backend {
    use super::*;

    pub use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode as Key},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };

    pub fn init_terminal() -> io::Result<Terminal<impl Backend>> {
        trace!(target:"crossterm", "Initializing terminal");
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(io::stdout());
        Terminal::new(backend)
    }

    pub fn restore_terminal() -> io::Result<()> {
        trace!(target:"crossterm", "Restoring terminal");
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
    }

    pub fn input_thread(tx_event: mpsc::Sender<AppEvent>) -> anyhow::Result<()> {
        trace!(target:"crossterm", "Starting input thread");
        while let Ok(event) = event::read() {
            trace!(target:"crossterm", "Stdin event received {:?}", event);
            tx_event.send(AppEvent::UiEvent(event))?;
        }
        Ok(())
    }
}

/// A module for termion specific code
#[cfg(feature = "termion")]
mod termion_backend {
    use super::*;
    use termion::screen::IntoAlternateScreen;
    pub use termion::{
        event::{Event, Key},
        input::{MouseTerminal, TermRead},
        raw::IntoRawMode,
    };

    pub fn init_terminal() -> io::Result<Terminal<impl Backend>> {
        trace!(target:"termion", "Initializing terminal");
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = stdout.into_alternate_screen()?;
        let backend = TermionBackend::new(stdout);
        Terminal::new(backend)
    }

    pub fn restore_terminal() -> io::Result<()> {
        trace!(target:"termion", "Restoring terminal");
        Ok(())
    }

    pub fn input_thread(tx_event: mpsc::Sender<AppEvent>) -> anyhow::Result<()> {
        trace!(target:"termion", "Starting input thread");
        for event in io::stdin().events() {
            let event = event?;
            trace!(target:"termion", "Stdin event received {:?}", event);
            tx_event.send(AppEvent::UiEvent(event))?;
        }
        Ok(())
    }
}

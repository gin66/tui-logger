use crate::{CircularBuffer, LevelConfig, TuiLoggerFile};
use chrono::{DateTime, Local};
use log::{Level, LevelFilter, Log, Metadata, Record};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::io::Write;
use std::mem;
use std::thread;

/// The TuiLoggerWidget shows the logging messages in an endless scrolling view.
/// It is controlled by a TuiWidgetState for selected events.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TuiLoggerLevelOutput {
    Abbreviated,
    Long,
}
/// These are the sub-structs for the static TUI_LOGGER struct.
pub struct HotSelect {
    pub hashtable: HashMap<u64, LevelFilter>,
    pub default: LevelFilter,
}
pub struct HotLog {
    pub events: CircularBuffer<ExtLogRecord>,
    pub mover_thread: Option<thread::JoinHandle<()>>,
}

pub struct ExtLogRecord {
    pub timestamp: DateTime<Local>,
    pub level: Level,
    target: String,
    file: String,
    pub line: u32,
    msg: String,
}
impl ExtLogRecord {
    #[inline]
    pub fn target(&self) -> &str {
       &self.target
    } 
    #[inline]
    pub fn file(&self) -> &str {
       &self.file
    } 
    #[inline]
    pub fn msg(&self) -> &str {
       &self.msg
    } 
}
pub struct TuiLoggerInner {
    pub hot_depth: usize,
    pub events: CircularBuffer<ExtLogRecord>,
    pub dump: Option<TuiLoggerFile>,
    pub total_events: usize,
    pub default: LevelFilter,
    pub targets: LevelConfig,
}
pub struct TuiLogger {
    pub hot_select: Mutex<HotSelect>,
    pub hot_log: Mutex<HotLog>,
    pub inner: Mutex<TuiLoggerInner>,
}
impl TuiLogger {
    pub fn move_events(&self) {
        // If there are no new events, then just return
        if self.hot_log.lock().events.total_elements() == 0 {
            return;
        }
        // Exchange new event buffer with the hot buffer
        let mut received_events = {
            let hot_depth = self.inner.lock().hot_depth;
            let new_circular = CircularBuffer::new(hot_depth);
            let mut hl = self.hot_log.lock();
            mem::replace(&mut hl.events, new_circular)
        };
        let mut tli = self.inner.lock();
        let total = received_events.total_elements();
        let elements = received_events.len();
        tli.total_events += total;
        let mut consumed = received_events.take();
        let mut reversed = Vec::with_capacity(consumed.len() + 1);
        while let Some(log_entry) = consumed.pop() {
            reversed.push(log_entry);
        }
        if total > elements {
            // Too many events received, so some have been lost
            let new_log_entry = ExtLogRecord {
                timestamp: reversed[reversed.len() - 1].timestamp,
                level: Level::Warn,
                target: "TuiLogger".to_string(),
                file: "?".to_string(),
                line: 0,
                msg: format!(
                    "There have been {} events lost, {} recorded out of {}",
                    total - elements,
                    elements,
                    total
                ),
            };
            reversed.push(new_log_entry);
        }
        let default_level = tli.default;
        while let Some(log_entry) = reversed.pop() {
            if tli.targets.get(&log_entry.target).is_none() {
                tli.targets.set(&log_entry.target, default_level);
            }
            if let Some(ref mut file_options) = tli.dump {
                let mut output = String::new();
                let (lev_long, lev_abbr, with_loc) = match log_entry.level {
                    log::Level::Error => ("ERROR", "E", true),
                    log::Level::Warn => ("WARN ", "W", true),
                    log::Level::Info => ("INFO ", "I", false),
                    log::Level::Debug => ("DEBUG", "D", true),
                    log::Level::Trace => ("TRACE", "T", true),
                };
                if let Some(fmt) = file_options.timestamp_fmt.as_ref() {
                    output.push_str(&format!("{}", log_entry.timestamp.format(fmt)));
                    output.push(file_options.format_separator);
                }
                match file_options.format_output_level {
                    None => {}
                    Some(TuiLoggerLevelOutput::Abbreviated) => {
                        output.push_str(lev_abbr);
                        output.push(file_options.format_separator);
                    }
                    Some(TuiLoggerLevelOutput::Long) => {
                        output.push_str(lev_long);
                        output.push(file_options.format_separator);
                    }
                }
                if file_options.format_output_target {
                    output.push_str(&log_entry.target);
                    output.push(file_options.format_separator);
                }
                if with_loc {
                    if file_options.format_output_file {
                        output.push_str(&log_entry.file);
                        output.push(file_options.format_separator);
                    }
                    if file_options.format_output_line {
                        output.push_str(&format!("{}", log_entry.line));
                        output.push(file_options.format_separator);
                    }
                }
                output.push_str(&log_entry.msg);
                if let Err(_e) = writeln!(file_options.dump, "{}", output) {
                    // TODO: What to do in case of write error ?
                }
            }
            tli.events.push(log_entry);
        }
    }
}
lazy_static! {
    pub static ref TUI_LOGGER: TuiLogger = {
        let hs = HotSelect {
            hashtable: HashMap::with_capacity(1000),
            default: LevelFilter::Info,
        };
        let hl = HotLog {
            events: CircularBuffer::new(1000),
            mover_thread: None,
        };
        let tli = TuiLoggerInner {
            hot_depth: 1000,
            events: CircularBuffer::new(10000),
            total_events: 0,
            dump: None,
            default: LevelFilter::Info,
            targets: LevelConfig::new(),
        };
        TuiLogger {
            hot_select: Mutex::new(hs),
            hot_log: Mutex::new(hl),
            inner: Mutex::new(tli),
        }
    };
}

impl Log for TuiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let h = fxhash::hash64(metadata.target());
        let hs = self.hot_select.lock();
        if let Some(&levelfilter) = hs.hashtable.get(&h) {
            metadata.level() <= levelfilter
        } else {
            metadata.level() <= hs.default
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.raw_log(record)
        }
    }

    fn flush(&self) {}
}

impl TuiLogger {
    pub fn raw_log(&self, record: &Record) {
        let log_entry = ExtLogRecord {
            timestamp: chrono::Local::now(),
            level: record.level(),
            target: record.target().to_string(),
            file: record.file().unwrap_or("?").to_string(),
            line: record.line().unwrap_or(0),
            msg: format!("{}", record.args()),
        };
        let mut events_lock = self.hot_log.lock();
        events_lock.events.push(log_entry);
        let need_signal =
            (events_lock.events.total_elements() % (events_lock.events.capacity() / 2)) == 0;
        if need_signal {
            events_lock
                .mover_thread
                .as_ref()
                .map(|jh| thread::Thread::unpark(jh.thread()));
        }
    }
}

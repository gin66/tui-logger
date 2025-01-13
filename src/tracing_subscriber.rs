//! `tracing-subscriber` support for `tui-logger`

use super::TUI_LOGGER;
use log::{self, Log, Record};
use std::collections::BTreeMap;
use std::fmt;
use tracing_subscriber::Layer;

#[derive(Default)]
struct ToStringVisitor<'a>(BTreeMap<&'a str, String>);

impl fmt::Display for ToStringVisitor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|(k, v)| -> fmt::Result {
            if *k == "message" {
                write!(f, " {}", v)
            } else {
                write!(f, " {}: {}", k, v)
            }
        })
    }
}

impl<'a> tracing::field::Visit for ToStringVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0
            .insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0
            .insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0
            .insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0
            .insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0
            .insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0
            .insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name(), format_args!("{:?}", value).to_string());
    }
}

#[allow(clippy::needless_doctest_main)]
///  tracing-subscriber-compatible layer that feeds messages to `tui-logger`.
///
///  ## How it works
///  Under the hood, tui_logger still uses `log`. `tracing` events are mapped to
///  `log` events internally (which are then fed to `tui-logger`).
///
///  ## Usage note
///  As per the example below, [init_logger()] must be called prior to logging events.
///
///  [init_logger()]: crate::init_logger()
///  ## Basic usage
///  ```
///  use tracing_subscriber::prelude::*;
///
///  fn main() {
///     tracing_subscriber::registry()
///          .with(tui_logger::tracing_subscriber_layer())
///          .init();
///     tui_logger::init_logger(tui_logger::LevelFilter::Trace).unwrap();
///     tracing::info!("Logging via tracing works!");
///  }
///  ```
pub struct TuiTracingSubscriberLayer;

impl<S> Layer<S> for TuiTracingSubscriberLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = ToStringVisitor::default();
        event.record(&mut visitor);

        let level = match *event.metadata().level() {
            tracing::Level::ERROR => log::Level::Error,
            tracing::Level::WARN => log::Level::Warn,
            tracing::Level::INFO => log::Level::Info,
            tracing::Level::DEBUG => log::Level::Debug,
            tracing::Level::TRACE => log::Level::Trace,
        };

        TUI_LOGGER.log(
            &Record::builder()
                .args(format_args!("{}", visitor))
                .level(level)
                .target(event.metadata().target())
                .file(event.metadata().file())
                .line(event.metadata().line())
                .module_path(event.metadata().module_path())
                .build(),
        );
    }
}

//! `tracing-subscriber` support for `tui-logger`

use std::collections::HashMap;
use super::TUI_LOGGER;
use log::{self, Log, Record};
use tracing_subscriber::Layer;

#[derive(Default)]
struct ToStringVisitor<'a>(HashMap<&'a str, String>);

impl ToStringVisitor<'_> {
    fn to_string<'a>(&self) -> String {
        self.0
            .iter()
            .fold("".to_string(), |mut str: String, (k, v)| {
                str += &*format!(" {}: {}", k, v);
                str
            })
    }
}

impl<'a> tracing::field::Visit for ToStringVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0.insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0.insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0.insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0.insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0.insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0.insert(field.name(), format_args!("{}", value).to_string());
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(field.name(), format_args!("{:?}", value).to_string());
    }
}

pub struct TuiTracingSubscriber;

impl<S> Layer<S> for TuiTracingSubscriber
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
                .args(format_args!("{}", visitor.to_string()))
                .level(level)
                .target(event.metadata().target())
                .file(event.metadata().file())
                .line(event.metadata().line())
                .module_path(event.metadata().module_path())
                .build(),
        );
    }
}
use std::sync::Arc;

use log::LevelFilter;
use parking_lot::Mutex;

use crate::{set_level_for_target, LevelConfig, TuiWidgetEvent};

#[derive(Clone, Copy, Debug)]
pub(crate) struct LinePointer {
    pub event_index: usize, // into event buffer
    pub subline: usize,
}

/// This struct contains the shared state of a TuiLoggerWidget and a TuiLoggerTargetWidget.
#[derive(Default)]
pub struct TuiWidgetState {
    inner: Arc<Mutex<TuiWidgetInnerState>>,
}
impl TuiWidgetState {
    /// Create a new TuiWidgetState
    pub fn new() -> TuiWidgetState {
        TuiWidgetState {
            inner: Arc::new(Mutex::new(TuiWidgetInnerState::new())),
        }
    }
    pub fn set_default_display_level(self, levelfilter: LevelFilter) -> TuiWidgetState {
        self.inner
            .lock()
            .config
            .set_default_display_level(levelfilter);
        self
    }
    pub fn set_level_for_target(self, target: &str, levelfilter: LevelFilter) -> TuiWidgetState {
        self.inner.lock().config.set(target, levelfilter);
        self
    }
    pub fn transition(&self, event: TuiWidgetEvent) {
        self.inner.lock().transition(event);
    }
    pub fn clone_state(&self) -> Arc<Mutex<TuiWidgetInnerState>> {
        self.inner.clone()
    }
}

#[derive(Default)]
pub struct TuiWidgetInnerState {
    pub(crate) config: LevelConfig,
    pub(crate) nr_items: usize,
    pub(crate) selected: usize,
    pub(crate) opt_line_pointer_center: Option<LinePointer>,
    pub(crate) opt_line_pointer_next_page: Option<LinePointer>,
    pub(crate) opt_line_pointer_prev_page: Option<LinePointer>,
    pub(crate) opt_selected_target: Option<String>,
    pub(crate) opt_selected_visibility_more: Option<LevelFilter>,
    pub(crate) opt_selected_visibility_less: Option<LevelFilter>,
    pub(crate) opt_selected_recording_more: Option<LevelFilter>,
    pub(crate) opt_selected_recording_less: Option<LevelFilter>,
    pub(crate) offset: usize,
    pub(crate) hide_off: bool,
    pub(crate) hide_target: bool,
    pub(crate) focus_selected: bool,
}
impl TuiWidgetInnerState {
    pub fn new() -> TuiWidgetInnerState {
        TuiWidgetInnerState::default()
    }
    fn transition(&mut self, event: TuiWidgetEvent) {
        use TuiWidgetEvent::*;
        match event {
            SpaceKey => {
                self.hide_off ^= true;
            }
            HideKey => {
                self.hide_target ^= true;
            }
            FocusKey => {
                self.focus_selected ^= true;
            }
            UpKey => {
                if !self.hide_target && self.selected > 0 {
                    self.selected -= 1;
                }
            }
            DownKey => {
                if !self.hide_target && self.selected + 1 < self.nr_items {
                    self.selected += 1;
                }
            }
            LeftKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_visibility_less) = self.opt_selected_visibility_less.take()
                    {
                        self.config.set(&selected_target, selected_visibility_less);
                    }
                }
            }
            RightKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_visibility_more) = self.opt_selected_visibility_more.take()
                    {
                        self.config.set(&selected_target, selected_visibility_more);
                    }
                }
            }
            PlusKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_recording_more) = self.opt_selected_recording_more.take() {
                        set_level_for_target(&selected_target, selected_recording_more);
                    }
                }
            }
            MinusKey => {
                if let Some(selected_target) = self.opt_selected_target.take() {
                    if let Some(selected_recording_less) = self.opt_selected_recording_less.take() {
                        set_level_for_target(&selected_target, selected_recording_less);
                    }
                }
            }
            PrevPageKey => self.opt_line_pointer_center = self.opt_line_pointer_prev_page,
            NextPageKey => self.opt_line_pointer_center = self.opt_line_pointer_next_page,
            EscapeKey => self.opt_line_pointer_center = None,
        }
    }
}

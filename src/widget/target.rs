use std::sync::Arc;

use crate::advance_levelfilter;
use crate::logger::TUI_LOGGER;
use crate::widget::inner::TuiWidgetInnerState;
use crate::Block;
use crate::Buffer;
use crate::Level;
use crate::LevelFilter;
use crate::Modifier;
use crate::Rect;
use crate::Style;
use crate::TuiWidgetState;
use crate::Widget;
use parking_lot::Mutex;

/// This is the definition for the TuiLoggerTargetWidget,
/// which allows configuration of the logger system and selection of log messages.
pub struct TuiLoggerTargetWidget<'b> {
    block: Option<Block<'b>>,
    /// Base style of the widget
    style: Style,
    style_show: Style,
    style_hide: Style,
    style_off: Option<Style>,
    highlight_style: Style,
    state: Arc<Mutex<TuiWidgetInnerState>>,
    targets: Vec<String>,
}
impl<'b> Default for TuiLoggerTargetWidget<'b> {
    fn default() -> TuiLoggerTargetWidget<'b> {
        TuiLoggerTargetWidget {
            block: None,
            style: Default::default(),
            style_off: None,
            style_hide: Style::default(),
            style_show: Style::default().add_modifier(Modifier::REVERSED),
            highlight_style: Style::default().add_modifier(Modifier::REVERSED),
            state: Arc::new(Mutex::new(TuiWidgetInnerState::new())),
            targets: vec![],
        }
    }
}
impl<'b> TuiLoggerTargetWidget<'b> {
    pub fn block(mut self, block: Block<'b>) -> TuiLoggerTargetWidget<'b> {
        self.block = Some(block);
        self
    }
    pub fn opt_style(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.style = s;
        }
        self
    }
    pub fn opt_style_off(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if style.is_some() {
            self.style_off = style;
        }
        self
    }
    pub fn opt_style_hide(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.style_hide = s;
        }
        self
    }
    pub fn opt_style_show(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.style_show = s;
        }
        self
    }
    pub fn opt_highlight_style(mut self, style: Option<Style>) -> TuiLoggerTargetWidget<'b> {
        if let Some(s) = style {
            self.highlight_style = s;
        }
        self
    }
    pub fn style(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style = style;
        self
    }
    pub fn style_off(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style_off = Some(style);
        self
    }
    pub fn style_hide(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style_hide = style;
        self
    }
    pub fn style_show(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.style_show = style;
        self
    }
    pub fn highlight_style(mut self, style: Style) -> TuiLoggerTargetWidget<'b> {
        self.highlight_style = style;
        self
    }
    pub(crate) fn inner_state(
        mut self,
        state: Arc<Mutex<TuiWidgetInnerState>>,
    ) -> TuiLoggerTargetWidget<'b> {
        self.state = state;
        self
    }
    pub fn state(mut self, state: &TuiWidgetState) -> TuiLoggerTargetWidget<'b> {
        self.state = state.clone_state();
        self
    }
}
impl<'b> Widget for TuiLoggerTargetWidget<'b> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        if list_area.width < 8 || list_area.height < 1 {
            return;
        }

        let la_left = list_area.left();
        let la_top = list_area.top();
        let la_width = list_area.width as usize;

        {
            let inner = &TUI_LOGGER.inner.lock();
            let hot_targets = &inner.targets;
            let mut state = self.state.lock();
            let hide_off = state.hide_off;
            let offset = state.offset;
            let focus_selected = state.focus_selected;
            {
                let targets = &mut state.config;
                targets.merge(hot_targets);
                self.targets.clear();
                for (t, levelfilter) in targets.iter() {
                    if hide_off && levelfilter == &LevelFilter::Off {
                        continue;
                    }
                    self.targets.push(t.clone());
                }
                self.targets.sort();
            }
            state.nr_items = self.targets.len();
            if state.selected >= state.nr_items {
                state.selected = state.nr_items.max(1) - 1;
            }
            if state.selected < state.nr_items {
                state.opt_selected_target = Some(self.targets[state.selected].clone());
                let t = &self.targets[state.selected];
                let (more, less) = if let Some(levelfilter) = state.config.get(t) {
                    advance_levelfilter(levelfilter)
                } else {
                    (None, None)
                };
                state.opt_selected_visibility_less = less;
                state.opt_selected_visibility_more = more;
                let (more, less) = if let Some(levelfilter) = hot_targets.get(t) {
                    advance_levelfilter(levelfilter)
                } else {
                    (None, None)
                };
                state.opt_selected_recording_less = less;
                state.opt_selected_recording_more = more;
            }
            let list_height = (list_area.height as usize).min(self.targets.len());
            let offset = if list_height > self.targets.len() {
                0
            } else if state.selected < state.nr_items {
                let sel = state.selected;
                if sel >= offset + list_height {
                    // selected is below visible list range => make it the bottom
                    sel - list_height + 1
                } else if sel.min(offset) + list_height > self.targets.len() {
                    self.targets.len() - list_height
                } else {
                    sel.min(offset)
                }
            } else {
                0
            };
            state.offset = offset;

            let targets = &(&state.config);
            let default_level = inner.default;
            for i in 0..list_height {
                let t = &self.targets[i + offset];
                // Comment in relation to issue #69:
                // Widgets maintain their own list of level filters per target.
                // These lists are not forwarded to the TUI_LOGGER, but kept widget private.
                // Example: This widget's private list contains a target named "not_yet",
                // and the application hasn't logged an entry with target "not_yet".
                // If displaying the target list, then "not_yet" will be only present in target,
                // but not in hot_targets. In issue #69 the problem has been, that
                // `hot_targets.get(t).unwrap()` has caused a panic. Which is to be expected.
                // The remedy is to use unwrap_or with default_level.
                let hot_level_filter = hot_targets.get(t).unwrap_or(default_level);
                let level_filter = targets.get(t).unwrap_or(default_level);
                for (j, sym, lev) in &[
                    (0, "E", Level::Error),
                    (1, "W", Level::Warn),
                    (2, "I", Level::Info),
                    (3, "D", Level::Debug),
                    (4, "T", Level::Trace),
                ] {
                    if let Some(cell) = buf.cell_mut((la_left + j, la_top + i as u16)) {
                        let cell_style = if hot_level_filter >= *lev {
                            if level_filter >= *lev {
                                if !focus_selected || i + offset == state.selected {
                                    self.style_show
                                } else {
                                    self.style_hide
                                }
                            } else {
                                self.style_hide
                            }
                        } else if let Some(style_off) = self.style_off {
                            style_off
                        } else {
                            cell.set_symbol(" ");
                            continue;
                        };
                        cell.set_style(cell_style);
                        cell.set_symbol(sym);
                    }
                }
                buf.set_stringn(la_left + 5, la_top + i as u16, ":", la_width, self.style);
                buf.set_stringn(
                    la_left + 6,
                    la_top + i as u16,
                    t,
                    la_width,
                    if i + offset == state.selected {
                        self.highlight_style
                    } else {
                        self.style
                    },
                );
            }
        }
    }
}

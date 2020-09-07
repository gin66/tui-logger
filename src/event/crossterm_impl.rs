use crossterm::event::{KeyCode, KeyEvent};

pub use crossterm::event::Event;

pub(crate) fn is_space_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: _,
        })
    )
}

pub(crate) fn is_up_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: _,
        })
    )
}

pub(crate) fn is_down_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: _,
        })
    )
}

pub(crate) fn is_left_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: _,
        })
    )
}

pub(crate) fn is_right_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: _,
        })
    )
}

pub(crate) fn is_plus_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Char('+'),
            modifiers: _,
        })
    )
}

pub(crate) fn is_minus_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Char('-'),
            modifiers: _,
        })
    )
}

pub(crate) fn is_h_key(evt: &Event) -> bool {
    matches!(
        evt,
        &Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: _,
        })
    )
}

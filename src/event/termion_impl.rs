use termion::event::Key;

pub use termion::event::Event;

pub(crate) fn is_space_key(evt: &Event) -> bool {
    &Event::Key(Key::Char(' ')) == evt
}

pub(crate) fn is_up_key(evt: &Event) -> bool {
    &Event::Key(Key::Up) == evt
}

pub(crate) fn is_down_key(evt: &Event) -> bool {
    &Event::Key(Key::Down) == evt
}

pub(crate) fn is_left_key(evt: &Event) -> bool {
    &Event::Key(Key::Left) == evt
}

pub(crate) fn is_right_key(evt: &Event) -> bool {
    &Event::Key(Key::Right) == evt
}

pub(crate) fn is_plus_key(evt: &Event) -> bool {
    &Event::Key(Key::Char('+')) == evt
}

pub(crate) fn is_minus_key(evt: &Event) -> bool {
    &Event::Key(Key::Char('-')) == evt
}

pub(crate) fn is_h_key(evt: &Event) -> bool {
    &Event::Key(Key::Char('h')) == evt
}

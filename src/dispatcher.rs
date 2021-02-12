use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

/// Dispatcher is used to dispatch any event to a dynamically built chain of handlers.
/// The dispatch is a one-shot event. After an event is successfully processed, the dispatch chain is emptied.
/// ```
/// extern crate tui_logger;
/// extern crate termion;
///
/// use tui_logger::Dispatcher;
/// use termion::event::{Event,Key};
///
/// let mut dispatcher = Dispatcher::new();
/// dispatcher.add_listener(|ev| { println!("called"); true });
/// dispatcher.dispatch(&Event::Key(Key::Up));
/// ```
#[derive(Default)]
pub struct Dispatcher<E: Debug> {
    map: Vec<Box<dyn Fn(&E) -> bool>>,
}
#[allow(dead_code)]
impl<E> Dispatcher<E>
where
    E: Debug,
{
    /// Create a new dispatcher
    pub fn new() -> Dispatcher<E> {
        trace!("New dispatcher is created.");
        Dispatcher::<E> { map: vec![] }
    }
    /// Add a listener at the end of the dispatch chain.
    /// Every Listener has to be a closure receiving a termion event as parameter and shall return a boolean.
    pub fn add_listener<F: 'static + Fn(&E) -> bool>(&mut self, f: F) {
        trace!("Add listener to this dispatcher.");
        self.map.push(Box::new(f));
    }
    /// Dispatches an event to the queue.
    /// The event is sent to the event handlers in the queue in FIFO order.
    /// If an event handler returns true, then the following event handlers will not be processed anymore,
    /// the queue will be emptied and the return value of dispatch() is true.
    /// If no event handler has returned true, or the event queue is empty, then the function returns false.
    pub fn dispatch(&mut self, ev: &E) -> bool {
        let mut processed = false;
        trace!(
            "Dispatcher with {} handlers shall dispatch event {:?}",
            self.map.len(),
            ev
        );
        for f in &self.map {
            if f(ev) {
                processed = true;
                break;
            }
        }
        if processed {
            self.map.clear();
        }
        trace!("Event dispatching result for {:?} is {}", ev, processed);
        processed
    }
    /// Clear the dispatcher queue
    pub fn clear(&mut self) {
        trace!("Dispatcher clear called.");
        self.map.clear();
    }
}

/// The EventListener Trait is only a standard way to implement a tui widget, which can listen to events.
pub trait EventListener<E: Debug> {
    /// Hand over a Dispatcher to the widget.
    fn dispatcher(self, dispatcher: Rc<RefCell<Dispatcher<E>>>) -> Self;
}

#[cfg(test)]
mod tests {
    use crate::Dispatcher;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[cfg(feature = "tui-crossterm")]
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    #[cfg(feature = "tui-termion")]
    use termion::event::{Event, Key};

    fn make_queue(dispatcher: &mut Dispatcher<Event>, v: Rc<RefCell<u64>>) {
        macro_rules! match_key {
            ($ev:expr, $name:ident) => {{
                let ev = $ev;
                #[cfg(feature = "tui-crossterm")]
                let m = matches!(
                    ev,
                    &Event::Key(KeyEvent {
                        code: KeyCode::$name,
                        modifiers: _,
                    })
                );
                #[cfg(feature = "tui-termion")]
                let m = ev == &Event::Key(Key::$name);
                m
            }};
        }

        let vx = v.clone();
        dispatcher.add_listener(move |ev| {
            if match_key!(ev, Left) {
                *vx.borrow_mut() += 1;
                true
            } else {
                false
            }
        });
        let vx = v.clone();
        dispatcher.add_listener(move |ev| {
            if match_key!(ev, Left) {
                *vx.borrow_mut() += 2;
                true
            } else {
                false
            }
        });
        let vx = v.clone();
        dispatcher.add_listener(move |ev| {
            if match_key!(ev, Down) {
                *vx.borrow_mut() += 4;
                true
            } else {
                false
            }
        });
    }

    #[test]
    fn test_dispatch() {
        macro_rules! gen_key {
            ($name:ident) => {{
                #[cfg(feature = "tui-crossterm")]
                let ev = Event::Key(KeyEvent {
                    code: KeyCode::$name,
                    modifiers: KeyModifiers::NONE,
                });
                #[cfg(feature = "tui-termion")]
                let ev = Event::Key(Key::$name);
                ev
            }};
        }

        let v = Rc::new(RefCell::new(0));

        let mut dispatcher = crate::Dispatcher::<Event>::new();
        make_queue(&mut dispatcher, v.clone());
        assert_eq!(*v.borrow(), 0);
        let processed = dispatcher.dispatch(&gen_key!(Left));
        assert_eq!(processed, true);
        assert_eq!(*v.borrow(), 1);

        make_queue(&mut dispatcher, v.clone());
        assert_eq!(*v.borrow(), 1);
        let processed = dispatcher.dispatch(&gen_key!(Down));
        assert_eq!(processed, true);
        assert_eq!(*v.borrow(), 5);

        make_queue(&mut dispatcher, v.clone());
        assert_eq!(*v.borrow(), 5);
        let processed = dispatcher.dispatch(&gen_key!(Up));
        assert_eq!(processed, false);
        assert_eq!(*v.borrow(), 5);
        let processed = dispatcher.dispatch(&gen_key!(Down));
        assert_eq!(processed, true);
        assert_eq!(*v.borrow(), 9);
    }
}

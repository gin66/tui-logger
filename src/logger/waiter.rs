use parking_lot::{Condvar, Mutex};
use std::time::{Duration, Instant};

pub(crate) struct Waiter {
    state: Mutex<WaiterState>,
    cvar: Condvar,
}

#[derive(Default)]
struct WaiterState {
    moved_index: u64,
}

impl Default for Waiter {
    fn default() -> Self {
        Waiter {
            state: Mutex::new(WaiterState::default()),
            cvar: Condvar::new(),
        }
    }
}

impl Waiter {
    pub(crate) fn notify_new_events(&self) {
        let mut state = self.state.lock();
        state.moved_index = state.moved_index.wrapping_add(1);
        self.cvar.notify_all();
    }

    pub(crate) fn wait(&self) -> u64 {
        let mut state = self.state.lock();
        let initial = state.moved_index;
        loop {
            if state.moved_index != initial {
                return state.moved_index;
            }
            self.cvar.wait(&mut state);
        }
    }

    pub(crate) fn wait_timeout(&self, timeout: Duration) -> Option<u64> {
        let mut state = self.state.lock();
        let initial = state.moved_index;
        let deadline = Instant::now() + timeout;
        loop {
            if state.moved_index != initial {
                return Some(state.moved_index);
            }
            let now = Instant::now();
            if now >= deadline {
                return None;
            }
            let timed_out = self.cvar.wait_for(&mut state, deadline - now).timed_out();
            if timed_out {
                // A notify could have raced with the timeout.
                if state.moved_index != initial {
                    return Some(state.moved_index);
                }
                return None;
            }
        }
    }
}
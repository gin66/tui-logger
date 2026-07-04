#![cfg(feature = "waiter")]

use std::sync::{mpsc, Arc, Barrier, Mutex, Once};
use std::thread;
use std::time::{Duration};

use log::*;
use tui_logger::*;

static INIT_LOGGER: Once = Once::new();

// Serialize tests within this file so they don't race each other on the
// shared global TUI_LOGGER state. The background move_events thread
// spawned by init_logger keeps running independently, so assertions
// must be robust against it.
static SERIAL: Mutex<()> = Mutex::new(());

fn ensure_logger() {
    INIT_LOGGER.call_once(|| {
        init_logger(LevelFilter::Trace).unwrap();
        set_default_level(LevelFilter::Trace);
    });
}

fn lock_serial() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn wait_wakes_when_new_events_are_moved() {
    let _g = lock_serial();
    ensure_logger();

    let (tx, rx) = mpsc::channel::<u64>();
    let barrier = Arc::new(Barrier::new(2));

    let b = Arc::clone(&barrier);
    let waiter = thread::spawn(move || {
        b.wait();
        tx.send(wait()).expect("send wait result");
    });

    barrier.wait();
    // Give the waiter a chance to actually block on the condvar.
    thread::sleep(Duration::from_millis(50));

    info!("wake-up message");
    move_events();

    let result = rx
        .recv_timeout(Duration::from_secs(2))
        .expect("wait() did not return in time");
    waiter.join().expect("waiter thread panicked");

    assert!(
        result > 0,
        "wait() should return a moved index > 0, got {}",
        result
    );
}

#[test]
fn wait_timeout_returns_some_after_move_events() {
    let _g = lock_serial();
    ensure_logger();

    info!("timeout test message");
    // The background move_events thread should pick up the message
    // within ~10ms and bump the moved-index, causing wait_timeout to
    // return Some.
    let result = wait_timeout(Duration::from_secs(2));
    assert!(
        result.is_some(),
        "expected Some after move_events, got None"
    );
}

#[test]
fn wait_timeout_returns_none_quickly_when_idle() {
    let _g = lock_serial();
    ensure_logger();

    // Capture the current moved-index, then call wait_timeout with a
    // very short timeout. If no new events arrive in that window
    // (likely, since we haven't logged anything), the call must
    // return None.
    //
    // This is inherently racy with the background move_events thread,
    // so we retry a few times to make the test reliable.
    for _ in 0..20 {
        let initial = wait_timeout(Duration::from_millis(1));
        // Skip iterations where the background thread happened to bump
        // the index between the previous call and now.
        if initial.is_none() {
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
    panic!("wait_timeout never returned None in 20 attempts");
}

#[test]
fn wait_returns_increasing_indices_across_two_bursts() {
    let _g = lock_serial();
    ensure_logger();

    // Call wait_timeout BEFORE move_events so it captures the
    // pre-bump index and waits for the upcoming bump.
    info!("burst one");
    let first = wait_timeout(Duration::from_millis(500))
        .expect("expected Some after first burst");
    move_events();

    info!("burst two");
    let second = wait_timeout(Duration::from_millis(500))
        .expect("expected Some after second burst");
    move_events();

    assert!(
        second > first,
        "moved index should advance across bursts: first={} second={}",
        first,
        second
    );
}
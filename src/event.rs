//! An event in the main interactive app. Based on the tui-rs event struct.
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::{Key, Event as TEvent};
use termion::input::TermRead;

/// An event; either an Input or a Tick
pub enum Event<I> {
    Input(I),
    Tick,
}

/// An small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

/// The config for the event class - decides what the exit key is and what the tick rate is
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    /// create an event object with a default config
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    /// Create an event with the given config
    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.events() {
                    match evt {
                        Ok(TEvent::Key(key)) => {
                            if let Err(_) = tx.send(Event::Input(key)) {
                                return;
                            }
                            if key == config.exit_key {
                                return;
                            }
                        },
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    tx.send(Event::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    /// Find the next event
    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

use std::thread;
use std::sync::mpsc;

use crate::input_listeners::KeyboardListener;
use crate::input_listeners::key::keyboard_event::KeyboardEvent;
use std::sync::mpsc::TryRecvError;

pub struct KeyboardListenerAggregator {
    keyboard_listeners: Vec<KeyboardListener>,
}

impl KeyboardListenerAggregator {
    pub fn new() -> KeyboardListenerAggregator {
        KeyboardListenerAggregator {
            keyboard_listeners: Vec::new()
        }
    }

    pub fn add_keyboard_listener(&mut self, keyboard_listener: KeyboardListener) {
        self.keyboard_listeners.push(keyboard_listener);
    }

    pub fn start_listening(&self, sender: mpsc::Sender<KeyboardEvent>) -> mpsc::Sender<()> {
        let (tx, rx) = mpsc::channel::<KeyboardEvent>();
        let (tx2, rx2) = mpsc::channel();
        let mut stoppers = Vec::new();

        for keyboard_listener in &self.keyboard_listeners {
            let tx = tx.clone();

            let stopper = keyboard_listener.start_listening(tx);
            stoppers.push(stopper);
        }

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(keyboard_event) => {
                        match sender.send(keyboard_event) {
                            Ok(_) => {},
                            Err(_) => {
                                break;
                            }
                        }
                    },
                    Err(_) => {
                        break;
                    },
                };

                match rx2.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => {
                        break;
                    },
                    Err(TryRecvError::Empty) => {}
                }
            }

            for stopper in stoppers {
                stopper.send(());
            }
        });

        tx2
    }
}
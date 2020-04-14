use std::thread;
use std::sync::mpsc;

use crate::key::keyboard_listener::KeyboardListener;
use crate::key::keyboard_event::KeyboardEvent;

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

    pub fn start_listening(&self, sender: mpsc::Sender<KeyboardEvent>) {
        let (tx, rx) = mpsc::channel::<KeyboardEvent>();

        for keyboard_listener in &self.keyboard_listeners {
            let tx = tx.clone();

            keyboard_listener.start_listening(tx);
        }

        thread::spawn(move || {
            loop {
                let keyboard_event = match rx.recv() {
                    Ok(event) => event,
                    Err(error) => {
                        println!("{:?}", error);
                        panic!("");
                    }
                };

                sender.send(keyboard_event)
                    .expect("Failed to send keyboard event to key chord producer.");
            }
        });
    }
}
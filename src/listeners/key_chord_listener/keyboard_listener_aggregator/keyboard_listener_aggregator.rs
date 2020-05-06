use std::sync::mpsc;
use std::thread;

use crate::listeners::KeyboardListener;
use crate::KeyboardEvent;
use crate::KeyboardListenerAggregatorHandle;
use std::time::Duration;

pub struct KeyboardListenerAggregator {
    keyboard_listeners: Vec<KeyboardListener>,
}

impl KeyboardListenerAggregator {
    pub fn new() -> KeyboardListenerAggregator {
        KeyboardListenerAggregator {
            keyboard_listeners: Vec::new(),
        }
    }

    pub fn add_keyboard_listener(&mut self, keyboard_listener: KeyboardListener) {
        self.keyboard_listeners.push(keyboard_listener);
    }

    pub fn start_listening(
        self,
        keyboard_event_sender: mpsc::Sender<KeyboardEvent>,
    ) -> KeyboardListenerAggregatorHandle {
        let (keyboard_event_sender_for_children, keyboard_event_listener) =
            mpsc::channel::<KeyboardEvent>();
        let (stop_sender, stop_receiver) = mpsc::channel();

        let mut keyboard_listener_handles = Vec::new();

        for keyboard_listener in &self.keyboard_listeners {
            let keyboard_event_sender_for_children = keyboard_event_sender_for_children.clone();

            let stopper = keyboard_listener.start_listening(keyboard_event_sender_for_children);
            keyboard_listener_handles.push(stopper);
        }

        thread::spawn(move || {
            keyboard_listener_aggregator_log!("Keyboard Listener Aggregator spawned.");

            loop {
                match keyboard_event_listener.try_recv() {
                    Ok(keyboard_event) => match keyboard_event_sender.send(keyboard_event) {
                        Ok(_) => {}
                        Err(_) => {
                            keyboard_listener_aggregator_elog!("Channel [Keyboard Event Aggregator] -> [Key Chord Producer] is destructed. Exiting...");
                            break;
                        }
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        keyboard_listener_aggregator_elog!("Channels [Keyboard Listener #] -> [Keyboard Listener Aggregator] are destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                };

                match stop_receiver.try_recv() {
                    Ok(()) => {
                        keyboard_listener_aggregator_log!("Got exit signal. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        keyboard_listener_aggregator_elog!("Channel [Key Chord Producer] -> [Keyboard Listener Aggregator] is destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            keyboard_listener_aggregator_log!("Execution is ended. Cleanup...");
            keyboard_listener_aggregator_log!("Stopping keyboard listeners...");

            for (index, keyboard_listener_handle) in
                keyboard_listener_handles.into_iter().enumerate()
            {
                keyboard_listener_aggregator_log!("Stopping keyboard listener #{}...", index);

                match keyboard_listener_handle.stop() {
                    Ok(()) => {
                        keyboard_listener_aggregator_log!("Stopped channel [Keyboard Listener Aggregator] -> [Keyboard Listener #{}].", index);
                        keyboard_listener_aggregator_log!("Stopped channel [Keyboard Listener #{}] -> [Keyboard Listener Aggregator].", index);
                    }
                    Err(()) => {
                        keyboard_listener_aggregator_elog!("Channel [Keyboard Listener Aggregator] -> [Keyboard Listener #{}] is destructed.", index);
                    }
                }
            }

            keyboard_listener_aggregator_log!("Keyboard listener aggregator is ended.");
        });

        let keyboard_listener_aggregator_handle =
            KeyboardListenerAggregatorHandle::new(stop_sender);

        keyboard_listener_aggregator_handle
    }
}

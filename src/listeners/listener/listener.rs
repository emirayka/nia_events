use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;

use crate::listeners::Event;
use crate::listeners::ListenerSettings;
use crate::listeners::KeyChordProducer;
use crate::ListenerHandle;

pub struct Listener {
    key_chord_producer: KeyChordProducer,
}

impl Listener {
    pub fn new(settings: ListenerSettings) -> Listener {
        Listener {
            key_chord_producer: KeyChordProducer::new(settings.get_key_chord_producer_settings()),
        }
    }

    pub fn start_listening(self) -> ListenerHandle {
        let key_chord_producer = self.key_chord_producer;

        let (event_sender, event_receiver) = mpsc::channel();
        let (stop_sender, rx) = mpsc::channel();

        thread::spawn(move || {
            let event_sender = event_sender.clone();
            let (key_chord_event_receiver, stopper) = key_chord_producer.start_listening();

            loop {
                match key_chord_event_receiver.recv() {
                    Ok(key_chord_event) => {
                        let key_chord = key_chord_event.into_key_chord();
                        let event = Event::KeyChordEvent(key_chord);

                        match event_sender.send(event) {
                            Err(_) => break,
                            _ => {}
                        };
                    }
                    Err(_) => {
                        break;
                    }
                }

                match rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }

            stopper.send(())
        });

        let event_listener_handle = ListenerHandle::new(event_receiver, stop_sender);

        event_listener_handle
    }
}

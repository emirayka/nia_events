use std::thread;
use std::sync::mpsc;

use crate::input_listeners::{KeyChordProducer, EventListenerSettings, Event};
use std::sync::mpsc::TryRecvError;

pub struct EventListener {
    key_chord_producer: KeyChordProducer,
}

impl EventListener {
    pub fn new(settings: EventListenerSettings) -> EventListener {
        EventListener {
            key_chord_producer: KeyChordProducer::new(settings.get_key_chord_producer_settings())
        }
    }

    pub fn start_listening(self) -> (mpsc::Receiver<Event>, mpsc::Sender<()>) {
        let key_chord_producer = self.key_chord_producer;

        let (event_sender, event_receiver) = mpsc::channel();
        let (tx, rx) = mpsc::channel();

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
                            _ => {},
                        };
                    },
                    Err(_) => {
                        break;
                    }
                }

                match rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => {
                        break;
                    },
                    Err(TryRecvError::Empty) => {}
                }
            }

            stopper.send(())
        });

        (event_receiver, tx)
    }
}
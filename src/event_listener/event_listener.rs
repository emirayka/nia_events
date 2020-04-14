use std::thread;
use std::sync::mpsc;

use crate::key::KeyChordProducer;
use crate::event_listener::event_listener_settings::EventListenerSettings;
use crate::event_listener::event::Event;

pub struct EventListener {
    key_chord_producer: KeyChordProducer,
}

impl EventListener {
    pub fn new(settings: EventListenerSettings) -> EventListener {
        EventListener {
            key_chord_producer: KeyChordProducer::new(settings.get_key_chord_producer_settings())
        }
    }

    pub fn start_listening(self) -> mpsc::Receiver<Event> {
        let key_chord_producer = self.key_chord_producer;
        let (event_sender, event_receiver) = mpsc::channel();

        thread::spawn(move || {
            let event_sender = event_sender.clone();
            let key_chord_event_receiver = key_chord_producer.start_listening();

            loop {
                let key_chord_event = key_chord_event_receiver.recv()
                    .expect("Failure while receiving key chord event from key chord producer.");

                let key_chord = key_chord_event.into_key_chord();
                let event = Event::KeyChordEvent(key_chord);

                event_sender.send(event)
                    .expect("Failure while sending event.")
            }
        });

        event_receiver
    }
}
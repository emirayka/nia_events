use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;

use crate::listeners::Event;
use crate::listeners::KeyChordProducer;
use crate::listeners::ListenerSettings;
use crate::ListenerHandle;
use std::time::Duration;

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
        let (stop_sender, stop_receiver) = mpsc::channel();

        thread::spawn(move || {
            listener_log!("Listener spawned.");

            listener_log!("Starting main listener...");
            let event_sender = event_sender.clone();
            listener_log!("Started channel: [Listener] -> [World].");

            listener_log!("Starting Key Chord Producer.");
            let key_chord_producer_handle = key_chord_producer.start_listening();
            listener_log!("Started channel: [Key Chord Producer] -> [Listener].");
            listener_log!("Started channel: [Listener] -> [Key Chord Producer].");

            loop {
                match key_chord_producer_handle.try_receive_event() {
                    Ok(key_chord_event) => {
                        let key_chord = key_chord_event.into_key_chord();
                        let event = Event::KeyChordEvent(key_chord);

                        match event_sender.send(event) {
                            Err(_) => {
                                listener_elog!(
                                    "Channel [Listener] -> [World] is destructed. Exiting..."
                                );
                                break;
                            }
                            Ok(_) => {}
                        };
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        listener_elog!(
                            "Channel [Key Chord Producer] -> [Listener] is destructed. Exiting..."
                        );
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                match stop_receiver.try_recv() {
                    Ok(()) => {
                        listener_log!("Got exit signal. Exiting...");
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        listener_elog!("Channel [World] -> [Listener] is destructed. Exiting...");
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            listener_log!("Execution is ended. Cleanup...");
            listener_log!("Stopping Key Chord Producer...");

            match key_chord_producer_handle.stop() {
                Ok(_) => {
                    listener_log!("Stopped channel: [Key Chord Producer] -> [Listener].");
                    listener_log!("Stopped channel: [Listener] -> [Key Chord Producer].");
                }
                Err(_) => {
                    listener_elog!("Channel [Listener] -> [Key Chord Producer] is destructed.");
                }
            }

            listener_log!("Listener is ended.");
        });

        let event_listener_handle = ListenerHandle::new(event_receiver, stop_sender);

        event_listener_handle
    }
}

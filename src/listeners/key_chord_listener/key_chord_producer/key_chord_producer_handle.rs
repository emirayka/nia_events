use std::sync::mpsc;

use crate::KeyChordEvent;

pub struct KeyChordProducerHandle {
    key_chord_event_receiver: mpsc::Receiver<KeyChordEvent>,
    stop_sender: mpsc::Sender<()>,
}

impl KeyChordProducerHandle {
    pub fn new(
        key_chord_event_receiver: mpsc::Receiver<KeyChordEvent>,
        stop_sender: mpsc::Sender<()>,
    ) -> KeyChordProducerHandle {
        KeyChordProducerHandle {
            key_chord_event_receiver,
            stop_sender,
        }
    }

    pub fn receive_event(&self) -> Result<KeyChordEvent, ()> {
        match self.key_chord_event_receiver.recv() {
            Ok(key_chord_event) => Ok(key_chord_event),
            Err(_) => Err(()),
        }
    }

    pub fn try_receive_event(&self) -> Result<KeyChordEvent, mpsc::TryRecvError> {
        match self.key_chord_event_receiver.try_recv() {
            Ok(key_chord_event) => Ok(key_chord_event),
            Err(error) => Err(error),
        }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

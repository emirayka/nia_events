use std::sync::mpsc;

pub struct KeyChordProducerHandle {
    stop_sender: mpsc::Sender<()>,
}

impl KeyChordProducerHandle {
    pub fn new(stop_sender: mpsc::Sender<()>) -> KeyChordProducerHandle {
        KeyChordProducerHandle {
            stop_sender,
        }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}

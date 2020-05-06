use std::sync::mpsc;

pub struct KeyboardListenerHandle {
    stop_sender: mpsc::Sender<()>,
}

impl KeyboardListenerHandle {
    pub fn new(stop_sender: mpsc::Sender<()>) -> KeyboardListenerHandle {
        KeyboardListenerHandle {
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
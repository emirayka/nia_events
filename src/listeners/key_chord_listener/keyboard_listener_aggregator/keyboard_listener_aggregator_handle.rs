use std::sync::mpsc;

pub struct KeyboardListenerAggregatorHandle {
    stop_sender: mpsc::Sender<()>,
}

impl KeyboardListenerAggregatorHandle {
    pub fn new(stop_sender: mpsc::Sender<()>) -> KeyboardListenerAggregatorHandle {
        KeyboardListenerAggregatorHandle { stop_sender }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

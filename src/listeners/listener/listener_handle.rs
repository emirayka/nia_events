use std::sync::mpsc;

use crate::Event;

pub struct ListenerHandle {
    event_receiver: mpsc::Receiver<Event>,
    stop_sender: mpsc::Sender<()>
}

impl ListenerHandle {
    pub fn new(
        event_receiver: mpsc::Receiver<Event>,
        stop_sender: mpsc::Sender<()>
    ) -> ListenerHandle {
        ListenerHandle {
            event_receiver,
            stop_sender,
        }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn read_event(&self) -> Result<Event, ()> {
        match self.event_receiver.recv() {
            Ok(event) => Ok(event),
            Err(_) => Err(())
        }
    }
}

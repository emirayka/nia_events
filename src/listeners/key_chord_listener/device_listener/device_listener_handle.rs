use std::sync::mpsc;

pub struct DeviceListenerHandle {
    stop_sender: mpsc::Sender<()>,
}

impl DeviceListenerHandle {
    pub fn new(stop_sender: mpsc::Sender<()>) -> DeviceListenerHandle {
        DeviceListenerHandle { stop_sender }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

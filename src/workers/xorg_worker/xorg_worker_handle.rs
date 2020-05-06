use std::sync::mpsc;

use crate::XorgWorkerCommand;

#[derive(Clone)]
pub struct XorgWorkerHandle {
    command_sender: mpsc::Sender<XorgWorkerCommand>,
    stop_sender: mpsc::Sender<()>,
}

impl XorgWorkerHandle {
    pub fn new(command_sender: mpsc::Sender<XorgWorkerCommand>, stop_sender: mpsc::Sender<()>) -> XorgWorkerHandle {
        XorgWorkerHandle {
            command_sender,
            stop_sender
        }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn send_command(&self, command: XorgWorkerCommand) -> Result<(), ()> {
        match self.command_sender.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}

use std::sync::mpsc;

use crate::Command;

#[derive(Clone)]
pub struct WorkerHandle {
    command_sender: mpsc::Sender<Command>,
    stop_sender: mpsc::Sender<()>,
}

impl WorkerHandle {
    pub fn new(command_sender: mpsc::Sender<Command>, stop_sender: mpsc::Sender<()>) -> WorkerHandle {
        WorkerHandle {
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

    pub fn send_command(&self, command: Command) -> Result<(), ()> {
        match self.command_sender.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}

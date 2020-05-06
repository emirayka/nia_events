use std::sync::mpsc;

use crate::UInputWorkerCommand;

#[derive(Clone)]
pub struct UInputWorkerHandle {
    command_sender: mpsc::Sender<UInputWorkerCommand>,
    stop_sender: mpsc::Sender<()>,
}

impl UInputWorkerHandle {
    pub fn new(command_sender: mpsc::Sender<UInputWorkerCommand>, stopper: mpsc::Sender<()>) -> UInputWorkerHandle {
        UInputWorkerHandle {
            command_sender,
            stop_sender: stopper
        }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn send_command(&self, command: UInputWorkerCommand) -> Result<(), ()> {
        match self.command_sender.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}

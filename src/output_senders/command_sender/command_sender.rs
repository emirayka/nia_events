use std::thread;
use std::sync::mpsc;

use crate::output_senders::Command;
use crate::output_senders::KeyWorker;
use std::sync::mpsc::TryRecvError;

pub struct CommandSender {
}

impl CommandSender {
    pub fn new() -> CommandSender {
        CommandSender {

        }
    }

    pub fn start_sending(&self) -> (mpsc::Sender<Command>, mpsc::Sender<()>) {
        let (command_sender, command_receiver) = mpsc::channel();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let key_worker = KeyWorker::new();
            let (key_command_sender, stopper) = key_worker.start_working();

            loop {
                match command_receiver.recv() {
                    Ok(command) => {
                        match command {
                            Command::KeyCommand(key_command) => {
                                key_command_sender.send(key_command).unwrap();
                            }
                        }
                    },
                    Err(_) => {
                        break;
                    }
                }

                match rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => {
                        break;
                    },
                    Err(TryRecvError::Empty) => {}
                }
            }

            stopper.send(())
        });

        (command_sender, tx)
    }
}
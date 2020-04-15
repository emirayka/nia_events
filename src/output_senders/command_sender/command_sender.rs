use std::thread;
use std::sync::mpsc;

use crate::output_senders::Command;
use crate::output_senders::KeyWorker;

pub struct CommandSender {
}

impl CommandSender {
    pub fn new() -> CommandSender {
        CommandSender {

        }
    }

    pub fn start_sending(&self) -> mpsc::Sender<Command> {
        let (command_sender, command_receiver) = mpsc::channel();

        thread::spawn(move || {
            let key_worker = KeyWorker::new();
            let key_command_sender = key_worker.start_working();

            loop {
                let command = command_receiver.recv()
                    .expect("Failure reading command to send");

                match command {
                    Command::KeyCommand(key_command) => {
                        key_command_sender.send(key_command).unwrap();
                    }
                }
            }
        });

        command_sender
    }
}
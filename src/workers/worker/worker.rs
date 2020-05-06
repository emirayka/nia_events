use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::WorkerHandle;
use crate::XorgWorker;
use crate::UInputWorker;
use crate::Command;
use crate::Error;

pub struct Worker {}

impl Worker {
    pub fn new() -> Worker {
        Worker {}
    }

    pub fn start_working(&self) -> Result<WorkerHandle, Error> {
        let (command_sender, command_receiver) = mpsc::channel();
        let (stopper_sender, stopper_receiver) = mpsc::channel();

        thread::spawn(move || {
            let uinput_worker = UInputWorker::new();
            let xorg_worker = XorgWorker::new();

            let uinput_worker_handle = match uinput_worker.start_working() {
                Ok(uinput_worker_handle) => uinput_worker_handle,
                Err(error) => {
                    worker_elog!("Cannot spawn uinput worker:");
                    worker_elog!("{:?}", error);
                    worker_elog!("Exiting...");
                    return;
                }
            };

            let xorg_worker_handle = match xorg_worker.start_working() {
                Ok(uinput_worker_handle) => uinput_worker_handle,
                Err(error) => {
                    worker_elog!("Cannot spawn xorrg worker:");
                    worker_elog!("{:?}", error);
                    worker_elog!("Exiting...");
                    return;
                }
            };

            loop {
                match command_receiver.try_recv() {
                    Ok(command) => match command {
                        Command::UInput(uinput_worker_command) => {
                            match uinput_worker_handle.send_command(uinput_worker_command) {
                                Err(_) => {
                                    worker_elog!("UInput worker channel disconnected. Exiting...");
                                    break;
                                },
                                _ => {}
                            }
                        }
                        Command::Xorg(xorg_worker_command) => {
                            match xorg_worker_handle.send_command(xorg_worker_command) {
                                Err(_) => {
                                    worker_elog!("Xorg worker channel disconnected. Exiting...");
                                    break;
                                },
                                _ => {}
                            }
                        }
                        Command::Spawn(command) => {
                            match std::process::Command::new(&command).spawn() {
                                Ok(_) => {}
                                Err(error) => {
                                    worker_elog!("Cannot execute shell command \"{}\":", command);
                                    worker_elog!("{:?}", error);
                                }
                            }
                        }
                        Command::Wait(milliseconds) => {
                            thread::sleep(Duration::from_millis(milliseconds));
                        }
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        worker_elog!("Worker channel destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => { }
                }

                match stopper_receiver.try_recv() {
                    Ok(()) => {
                        worker_log!("Got exit signal. Exiting...");
                        break;
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        worker_elog!("Worker channel destructed. Exiting...");
                        break;
                    },
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            match uinput_worker_handle.stop() {
                Ok(()) => {
                    worker_log!("Successfully stopped uinput worker.");
                }
                Err(()) => {
                    worker_elog!("Error while stopping uinput worker.");
                }
            };
            match xorg_worker_handle.stop() {
                Ok(()) => {
                    worker_log!("Successfully stopped xorg worker.");
                }
                Err(()) => {
                    worker_elog!("Error while stopping xorg worker.");
                }
            };
        });

        let worker_handle = WorkerHandle::new(command_sender, stopper_sender);

        Ok(worker_handle)
    }
}
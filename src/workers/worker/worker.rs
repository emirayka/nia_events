use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::Command;
use crate::Error;
use crate::UInputWorker;
use crate::WorkerHandle;
use crate::XorgWorker;

pub struct Worker {}

impl Worker {
    pub fn new() -> Worker {
        Worker {}
    }

    pub fn start_working(&self) -> Result<WorkerHandle, Error> {
        let (command_sender, command_receiver) = mpsc::channel();
        let (stopper_sender, stopper_receiver) = mpsc::channel();

        thread::spawn(move || {
            worker_log!("Worker spawned.");

            let uinput_worker = UInputWorker::new();
            let xorg_worker = XorgWorker::new();

            worker_log!("Starting UInput Worker.");

            let uinput_worker_handle = match uinput_worker.start_working() {
                Ok(uinput_worker_handle) => uinput_worker_handle,
                Err(error) => {
                    worker_elog!("Cannot spawn uinput worker:");
                    worker_elog!("{:?}", error);
                    worker_elog!("Exiting...");
                    return;
                }
            };
            worker_log!("Started channel [UInput Worker] -> [Worker].");
            worker_log!("Started channel [Worker] -> [UInput Worker].");

            worker_log!("Starting Xorg Worker...");

            let xorg_worker_handle = match xorg_worker.start_working() {
                Ok(uinput_worker_handle) => uinput_worker_handle,
                Err(error) => {
                    worker_elog!("Cannot spawn xorrg worker:");
                    worker_elog!("{:?}", error);
                    worker_elog!("Exiting...");
                    return;
                }
            };
            worker_log!("Started channel [Xorg Worker] -> [Worker].");
            worker_log!("Started channel [Worker] -> [Xorg Worker].");

            loop {
                match command_receiver.try_recv() {
                    Ok(command) => match command {
                        Command::UInput(uinput_worker_command) => {
                            match uinput_worker_handle.send_command(uinput_worker_command) {
                                Err(_) => {
                                    worker_elog!("Channel [Worker] -> [UInput Worker] is destructed. Exiting...");
                                    break;
                                }
                                _ => {}
                            }
                        }
                        Command::Xorg(xorg_worker_command) => {
                            match xorg_worker_handle.send_command(xorg_worker_command) {
                                Err(_) => {
                                    worker_elog!("Channel [Worker] -> [Xorg Worker] is destructed. Exiting...");
                                    break;
                                }
                                _ => {}
                            }
                        }
                        Command::Spawn(command) => {
                            let exec = subprocess::Exec::shell(&command);

                            match exec.popen() {
                                Ok(mut popen) => popen.detach(),
                                Err(error) => {
                                    worker_elog!("Cannot spawn process: \"{}\"", command);
                                    worker_elog!("Error: {}", error.to_string());
                                }
                            }
                        }
                        Command::Wait(milliseconds) => {
                            thread::sleep(Duration::from_millis(milliseconds));
                        }
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        worker_elog!("Channel [World] -> [Worker] is destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                match stopper_receiver.try_recv() {
                    Ok(()) => {
                        worker_log!("Got exit signal. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        worker_elog!("Channel [World] -> [Worker] is destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            worker_log!("Execution is ended. Cleanup...");

            worker_log!("Stopping UInput Worker.");
            match uinput_worker_handle.stop() {
                Ok(()) => {
                    worker_log!("Stopped channel [Worker] -> [UInput Worker].");
                }
                Err(()) => {
                    worker_elog!("Channel [Worker] -> [UInput Worker] is destructed.");
                }
            };

            worker_log!("Stopping Xorg Worker.");
            match xorg_worker_handle.stop() {
                Ok(()) => {
                    worker_log!("Stopped channel [Worker] -> [Xorg Worker].");
                }
                Err(()) => {
                    worker_elog!("Channel [Worker] -> [Xorg Worker] is destructed.");
                }
            };

            worker_log!("Worker is ended.");
        });

        let worker_handle = WorkerHandle::new(command_sender, stopper_sender);

        Ok(worker_handle)
    }
}

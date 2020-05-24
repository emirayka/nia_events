use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use crate::workers::xorg_worker::xorg_worker_handle::XorgWorkerHandle;
use crate::Error;
use crate::XorgDeviceBuilder;
use crate::XorgWorkerCommand;

pub struct XorgWorker {}

impl XorgWorker {
    pub fn new() -> XorgWorker {
        XorgWorker {}
    }

    pub fn start_working(self) -> Result<XorgWorkerHandle, Error> {
        let (command_sender, command_receiver) = mpsc::channel();

        let (loop_stopper, stop_receiver) = mpsc::channel();

        thread::spawn(move || {
            xorg_worker_log!("Xorg worker spawned.");

            let mut xorg_device = match XorgDeviceBuilder::build_default() {
                Ok(device) => device,
                Err(error) => {
                    xorg_worker_elog!("Cannot create xorg device:");
                    xorg_worker_elog!("{:?}", error);
                    return;
                }
            };

            xorg_worker_log!("Successfully created xorg device.");

            loop {
                let result = match command_receiver.try_recv() {
                    Ok(xorg_worker_command) => {
                        println!("{:?}", xorg_worker_command);

                        match xorg_worker_command {
                            XorgWorkerCommand::MouseMoveBy(x, y) => xorg_device.mouse_move_by(x, y),
                            XorgWorkerCommand::MouseMoveTo(x, y) => xorg_device.mouse_move_to(x, y),
                            XorgWorkerCommand::TextType(string) => xorg_device.type_text(&string),
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        xorg_worker_elog!(
                            "Channel [Worker] -> [Xorg Worker] is destructed. Exiting..."
                        );
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => Ok(()),
                };

                if let Err(error) = result {
                    xorg_worker_elog!("Command execution failed:");
                    xorg_worker_elog!("{}", error.get_message());
                }

                match stop_receiver.try_recv() {
                    Ok(()) => {
                        xorg_worker_log!("Got exit signal. Exiting...");
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        xorg_worker_elog!(
                            "Channel [Worker] -> [Xorg Worker] is destructed. Exiting..."
                        );
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            xorg_worker_log!("Xorg Worker is ended.");
        });

        let handle = XorgWorkerHandle::new(command_sender, loop_stopper);

        Ok(handle)
    }
}

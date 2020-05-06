use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::Error;
use crate::UInputDeviceBuilder;
use crate::UInputWorkerCommand;
use crate::workers::uinput_worker::uinput_worker_handle::UInputWorkerHandle;

pub struct UInputWorker {}

impl UInputWorker {
    pub fn new() -> UInputWorker {
        UInputWorker {}
    }

    pub fn start_working(self) -> Result<UInputWorkerHandle, Error> {
        let (command_sender, command_receiver) = mpsc::channel();
        let (stopper_sender, stopper_receiver) = mpsc::channel();

        thread::spawn(move || {
            uinput_worker_log!("Uinput worker spawned.");

            let mut uinput_device = match UInputDeviceBuilder::build_default() {
                Ok(device) => device,
                Err(error) => {
                    uinput_worker_elog!("Cannot create uinput device:");
                    uinput_worker_elog!("{:?}", error);
                    return;
                }
            };

            uinput_worker_log!("Created uinput device.");

            loop {
                let result = match command_receiver.try_recv() {
                    Ok(uinput_worker_command) => {
                        let result = match uinput_worker_command {
                            UInputWorkerCommand::ForwardKeyChord(key_chord) => {
                                uinput_device.key_chord_press(key_chord)
                            }
                            UInputWorkerCommand::KeyDown(key_id) => {
                                uinput_device.key_down(key_id)
                            }
                            UInputWorkerCommand::KeyPress(key_id) => {
                                uinput_device.key_press(key_id)
                            }
                            UInputWorkerCommand::KeyUp(key_id) => {
                                uinput_device.key_up(key_id)
                            }
                            UInputWorkerCommand::MouseButtonDown(button_id) => {
                                uinput_device.mouse_button_down(button_id)
                            }
                            UInputWorkerCommand::MouseButtonPress(button_id) => {
                                uinput_device.mouse_button_press(button_id)
                            }
                            UInputWorkerCommand::MouseButtonUp(button_id) => {
                                uinput_device.mouse_button_up(button_id)
                            }
                        };

                        thread::sleep(Duration::from_millis(1));

                        result
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        uinput_worker_elog!("UInput worker channel destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        Ok(())
                    }
                };

                if let Err(error) = result {
                    uinput_worker_elog!("Command execution failed:");
                    uinput_worker_elog!("{}", error.get_message());
                }

                match stopper_receiver.try_recv() {
                    Ok(()) => {
                        uinput_worker_log!("Got exit signal. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        uinput_worker_elog!("UInput worker channel destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }
        });

        let uinput_worker_handle = UInputWorkerHandle::new(
            command_sender, stopper_sender
        );

        Ok(uinput_worker_handle)
    }
}
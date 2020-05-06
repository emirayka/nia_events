use std::fs::File;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use evdev_rs::enums::EventCode;

use crate::KeyboardId;
use crate::KeyId;
use crate::KeyboardEvent;
use crate::KeyboardEventType;
use crate::KeyboardListenerHandle;

pub struct KeyboardListener {
    keyboard_id: KeyboardId,
    device_path: String,
}

impl KeyboardListener {
    pub fn new(keyboard_id: KeyboardId, device_path: String) -> KeyboardListener {
        KeyboardListener {
            keyboard_id,
            device_path,
        }
    }

    pub fn start_listening(&self, keyboard_event_sender: mpsc::Sender<KeyboardEvent>) -> KeyboardListenerHandle {
        let device_path = self.device_path.clone();
        let keyboard_id = self.keyboard_id;

        let (stop_sender, stop_receiver) = mpsc::channel();

        thread::spawn(move || {
            let fd = match File::open(&device_path) {
                Ok(fd) => fd,
                Err(error) => {
                    keyboard_listener_elog!(keyboard_id, "Cannot open file. Error:");
                    keyboard_listener_elog!(keyboard_id, "{:?}", error);
                    return;
                }
            };

            let mut device = match evdev_rs::device::Device::new() {
                Some(device) => device,
                None => {
                    keyboard_listener_elog!(keyboard_id, "Cannot create device for {}", device_path);
                    return;
                }
            };

            match device.set_fd(fd) {
                Err(errno) => {
                    keyboard_listener_elog!(keyboard_id, "Failed while setting file descriptor to keyboard event listener. Errno: {}.", errno);
                    return;
                }
                Ok(_) => {}
            }

            match device.grab(evdev_rs::GrabMode::Grab) {
                Ok(_) => {}
                Err(errno) => {
                    keyboard_listener_elog!(keyboard_id, "Cannot grab \"{}\". Errno: {}.", device_path, errno);
                    return;
                }
            }

            let flags = evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING;

            loop {
                // if device.has_event_pending() {
                while device.has_event_pending() {
                    match device.next_event(flags) {
                        Ok((read_status, event)) => match read_status {
                            evdev_rs::ReadStatus::Sync => {}
                            evdev_rs::ReadStatus::Success => match event.event_type {
                                evdev_rs::enums::EventType::EV_KEY => {
                                    let keyboard_id = keyboard_id;

                                    let key_id = if let EventCode::EV_KEY(ev_key) = event.event_code {
                                        KeyId::from_ev_key(ev_key)
                                    } else {
                                        continue;
                                    };

                                    let event_type = KeyboardEventType::from_value(event.value);

                                    let keyboard_event =
                                        KeyboardEvent::new(keyboard_id, key_id, event_type);

                                    match keyboard_event_sender.send(keyboard_event) {
                                        Ok(_) => { }
                                        Err(_) => {
                                            keyboard_listener_elog!(keyboard_id, "Keyboard listener channel destructed. Exiting...");
                                            break;
                                        },
                                    }
                                }
                                _ => {}
                            },
                        },
                        Err(errno) => {
                            keyboard_listener_elog!(keyboard_id, "Keyboard listener got an error: {}. Exiting...", errno);
                            break;
                        }
                    }
                }

                match stop_receiver.try_recv() {
                    Ok(()) => {
                        keyboard_listener_log!(keyboard_id, "Got interruption command. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        keyboard_listener_elog!(keyboard_id, "Keyboard listener channel destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        // todo: try to implement listening without that
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });

        KeyboardListenerHandle::new(stop_sender)
    }
}

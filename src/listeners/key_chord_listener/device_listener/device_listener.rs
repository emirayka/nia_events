use std::fs::File;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use evdev_rs::enums::EventCode;

use crate::DeviceEvent;
use crate::DeviceEventType;
use crate::DeviceInfo;
use crate::DeviceListenerHandle;
use crate::KeyId;

pub struct DeviceListener {
    device_info: DeviceInfo,
}

impl DeviceListener {
    pub fn new(device_info: DeviceInfo) -> DeviceListener {
        DeviceListener { device_info }
    }

    pub fn start_listening(
        &self,
        device_event_sender: mpsc::Sender<DeviceEvent>,
    ) -> DeviceListenerHandle {
        let device_path = self.device_info.get_device_path().clone();
        let device_id = self.device_info.get_device_id();

        let (stop_sender, stop_receiver) = mpsc::channel();

        thread::spawn(move || {
            device_listener_log!(device_id, "Keyboard Listener #{} spawned.", device_id);

            let fd = match File::open(&device_path) {
                Ok(fd) => fd,
                Err(error) => {
                    device_listener_elog!(device_id, "Cannot open file. Error:");
                    device_listener_elog!(device_id, "{:?}", error);
                    device_listener_elog!(device_id, "Keyboard Listener #{} is ended.", device_id);
                    return;
                }
            };

            let mut device = match evdev_rs::device::Device::new() {
                Some(device) => device,
                None => {
                    device_listener_elog!(device_id, "Cannot create device for {}.", device_path);
                    device_listener_elog!(device_id, "Keyboard Listener #{} is ended.", device_id);
                    return;
                }
            };

            match device.set_fd(fd) {
                Err(errno) => {
                    device_listener_elog!(
                        device_id,
                        "Failed while setting file descriptor to device event listener. Errno: {}.",
                        errno
                    );
                    device_listener_elog!(device_id, "Keyboard Listener #{} is ended.", device_id);
                    return;
                }
                Ok(_) => {}
            }

            match device.grab(evdev_rs::GrabMode::Grab) {
                Ok(_) => {}
                Err(errno) => {
                    device_listener_elog!(
                        device_id,
                        "Cannot grab \"{}\". Errno: {}.",
                        device_path,
                        errno
                    );
                    device_listener_elog!(device_id, "Keyboard Listener #{} is ended.", device_id);
                    return;
                }
            }

            let flags = evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING;

            loop {
                while device.has_event_pending() {
                    match device.next_event(flags) {
                        Ok((read_status, event)) => match read_status {
                            evdev_rs::ReadStatus::Sync => {}
                            evdev_rs::ReadStatus::Success => match event.event_type {
                                evdev_rs::enums::EventType::EV_KEY => {
                                    let device_id = device_id;

                                    let key_id = if let EventCode::EV_KEY(ev_key) = event.event_code
                                    {
                                        KeyId::from_ev_key(ev_key)
                                    } else {
                                        continue;
                                    };

                                    let event_type = DeviceEventType::from_value(event.value);

                                    let device_event =
                                        DeviceEvent::new(device_id, key_id, event_type);

                                    match device_event_sender.send(device_event) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            device_listener_elog!(device_id, "Channel [Keyboard Listener #{}] -> [Keyboard Event Aggregator] is destructed. Exiting...", device_id);
                                            break;
                                        }
                                    }
                                }
                                _ => {}
                            },
                        },
                        Err(errno) => {
                            device_listener_elog!(
                                device_id,
                                "Error reading device events: \"Errno {}\". Exiting...",
                                errno
                            );
                            break;
                        }
                    }
                }

                match stop_receiver.try_recv() {
                    Ok(()) => {
                        device_listener_log!(device_id, "Got interruption command. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        device_listener_elog!(device_id, "Channel [Keyboard Listener Aggregator] -> [Keyboard Listener #{}] is destructed. Exiting...", device_id);
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            device_listener_log!(device_id, "Keyboard Listener #{} is ended.", device_id);
        });

        DeviceListenerHandle::new(stop_sender)
    }
}

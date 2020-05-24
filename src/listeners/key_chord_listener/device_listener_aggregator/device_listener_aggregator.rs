use std::sync::mpsc;
use std::thread;

use crate::listeners::DeviceListener;
use crate::DeviceEvent;
use crate::KeyboardListenerAggregatorHandle;
use std::time::Duration;

pub struct DeviceListenerAggregator {
    device_listeners: Vec<DeviceListener>,
}

impl DeviceListenerAggregator {
    pub fn new() -> DeviceListenerAggregator {
        DeviceListenerAggregator {
            device_listeners: Vec::new(),
        }
    }

    pub fn add_device_listener(&mut self, device_listener: DeviceListener) {
        self.device_listeners.push(device_listener);
    }

    pub fn start_listening(
        self,
        device_event_sender: mpsc::Sender<DeviceEvent>,
    ) -> KeyboardListenerAggregatorHandle {
        let (device_event_sender_for_children, device_event_listener) =
            mpsc::channel::<DeviceEvent>();
        let (stop_sender, stop_receiver) = mpsc::channel();

        let mut device_listener_handles = Vec::new();

        for device_listener in &self.device_listeners {
            let device_event_sender_for_children = device_event_sender_for_children.clone();

            let device_listener_handle =
                device_listener.start_listening(device_event_sender_for_children);
            device_listener_handles.push(device_listener_handle);
        }

        thread::spawn(move || {
            device_listener_aggregator_log!("Keyboard Listener Aggregator spawned.");

            loop {
                match device_event_listener.try_recv() {
                    Ok(device_event) => match device_event_sender.send(device_event) {
                        Ok(_) => {}
                        Err(_) => {
                            device_listener_aggregator_elog!("Channel [Keyboard Event Aggregator] -> [Key Chord Producer] is destructed. Exiting...");
                            break;
                        }
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        device_listener_aggregator_elog!("Channels [Keyboard Listener #] -> [Keyboard Listener Aggregator] are destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                };

                match stop_receiver.try_recv() {
                    Ok(()) => {
                        device_listener_aggregator_log!("Got exit signal. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        device_listener_aggregator_elog!("Channel [Key Chord Producer] -> [Keyboard Listener Aggregator] is destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            device_listener_aggregator_log!("Execution is ended. Cleanup...");
            device_listener_aggregator_log!("Stopping device listeners...");

            for (index, device_listener_handle) in device_listener_handles.into_iter().enumerate() {
                device_listener_aggregator_log!("Stopping device listener #{}...", index);

                match device_listener_handle.stop() {
                    Ok(()) => {
                        device_listener_aggregator_log!("Stopped channel [Keyboard Listener Aggregator] -> [Keyboard Listener #{}].", index);
                        device_listener_aggregator_log!("Stopped channel [Keyboard Listener #{}] -> [Keyboard Listener Aggregator].", index);
                    }
                    Err(()) => {
                        device_listener_aggregator_elog!("Channel [Keyboard Listener Aggregator] -> [Keyboard Listener #{}] is destructed.", index);
                    }
                }
            }

            device_listener_aggregator_log!("Keyboard listener aggregator is ended.");
        });

        let device_listener_aggregator_handle = KeyboardListenerAggregatorHandle::new(stop_sender);

        device_listener_aggregator_handle
    }
}

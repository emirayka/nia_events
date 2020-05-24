use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use crate::DeviceEventType;
use crate::DeviceId;
use crate::DeviceListener;
use crate::DeviceListenerAggregator;
use crate::KeyChord;
use crate::KeyChordEvent;
use crate::KeyChordProducerSettings;
use crate::{DeviceEvent, DeviceInfo};
use crate::{Key, KeyChordProducerHandle};
use std::time::Duration;

fn is_modifier_event(modifier_map: &HashMap<Key, bool>, key_chord_part: Key) -> bool {
    modifier_map.contains_key(&key_chord_part)
}

fn set_modifier_state(modifier_map: &mut HashMap<Key, bool>, key_chord_part: Key, state: bool) {
    let reference = modifier_map.get_mut(&key_chord_part).unwrap();

    *reference = state;
}

fn construct_key_chord(modifier_map: &HashMap<Key, bool>, event: DeviceEvent) -> KeyChord {
    let modifier_keys = modifier_map
        .iter()
        .filter(|(_, pressed)| **pressed)
        .map(|(key, _)| *key)
        .collect();

    let ordinary_key = Key::Key2(event.get_device_id(), event.get_key_id());

    KeyChord::new(modifier_keys, ordinary_key)
}

pub struct KeyChordProducer {
    device_listener_aggregator: DeviceListenerAggregator,
    modifier_keys: Vec<Key>,
}

impl KeyChordProducer {
    pub fn new(settings: &KeyChordProducerSettings) -> KeyChordProducer {
        let devices = settings.get_devices();
        let modifiers = settings.get_modifiers();

        let mut device_listener_aggregator = DeviceListenerAggregator::new();

        for device_info in devices {
            let listener = DeviceListener::new(device_info.clone());

            device_listener_aggregator.add_device_listener(listener);
        }

        let mut key_chord_producer = KeyChordProducer {
            device_listener_aggregator,
            modifier_keys: Vec::new(),
        };

        for modifier in modifiers {
            key_chord_producer.modifier_keys.push(*modifier)
        }

        key_chord_producer
    }

    pub fn start_listening(self) -> KeyChordProducerHandle {
        let modifier_keys = self.modifier_keys.clone();

        let (device_event_sender, device_event_receiver) = mpsc::channel();
        let (key_chord_event_sender, key_chord_event_receiver) = mpsc::channel();
        let (stop_sender, stop_receiver) = mpsc::channel();

        let device_listener_aggregator = self.device_listener_aggregator;

        thread::spawn(move || {
            key_chord_producer_log!("Key chord producer spawned.");

            key_chord_producer_log!("Starting Device Listener Aggregator...");

            let device_listener_aggregator_handle =
                device_listener_aggregator.start_listening(device_event_sender);

            key_chord_producer_log!(
                "Started channel: [Device Listener Aggregator] -> [Key Chord Producer]."
            );
            key_chord_producer_log!(
                "Started channel: [Key Chord Producer] -> [Device Listener Aggregator]."
            );

            let modifier_keys = modifier_keys;
            let mut modifier_map = HashMap::new();

            for modifier_key in modifier_keys {
                modifier_map.insert(modifier_key, false);
            }

            loop {
                match device_event_receiver.try_recv() {
                    Ok(device_event) => {
                        let key_chord_part =
                            Key::Key2(device_event.get_device_id(), device_event.get_key_id());

                        if is_modifier_event(&mut modifier_map, key_chord_part) {
                            match device_event.get_event_type() {
                                DeviceEventType::PRESSED => {
                                    set_modifier_state(&mut modifier_map, key_chord_part, true);
                                }
                                DeviceEventType::RELEASED => {
                                    set_modifier_state(&mut modifier_map, key_chord_part, false);
                                }
                                DeviceEventType::UNKNOWN => {}
                            }
                        } else {
                            if device_event.get_event_type() == DeviceEventType::PRESSED {
                                continue;
                            }

                            let key_chord = construct_key_chord(&modifier_map, device_event);
                            let key_chord_event = KeyChordEvent::from(key_chord);

                            match key_chord_event_sender.send(key_chord_event) {
                                Ok(()) => {}
                                Err(_) => {
                                    key_chord_producer_elog!("[Key Chord Producer] -> [Listener] channel is destructed. Exiting...");
                                    break;
                                }
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        key_chord_producer_elog!("[Device Event Aggregator] -> [Key Chord Producer] channel is destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                match stop_receiver.try_recv() {
                    Ok(()) => {
                        key_chord_producer_log!("Got exit signal. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        key_chord_producer_elog!(
                            "[Listener] -> [Key Chord Producer] channel is destructed. Exiting..."
                        );
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            key_chord_producer_log!("Execution is ended. Cleanup...");
            key_chord_producer_log!("Stopping device listener aggregator...");

            match device_listener_aggregator_handle.stop() {
                Ok(_) => {
                    key_chord_producer_log!(
                        "Stopped channel: [Device Listener Aggregator] -> [Key Chord Producer]."
                    );
                    key_chord_producer_log!(
                        "Stopped channel: [Key Chord Producer] -> [Device Listener Aggregator]."
                    );
                }
                Err(_) => {
                    key_chord_producer_elog!("Channel [Key Chord Producer] -> [Device Listener Aggregator] is destructed.");
                }
            };

            key_chord_producer_log!("Key Chord Producer is ended.");
        });

        KeyChordProducerHandle::new(key_chord_event_receiver, stop_sender)
    }
}

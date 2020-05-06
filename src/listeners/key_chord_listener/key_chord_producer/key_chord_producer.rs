use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use crate::KeyChordPart;
use crate::KeyChord;
use crate::KeyboardEvent;
use crate::KeyboardListenerAggregator;
use crate::KeyChordProducerSettings;
use crate::KeyboardId;
use crate::KeyboardListener;
use crate::KeyChordEvent;
use crate::KeyboardEventType;
use std::time::Duration;

fn is_modifier_event(
    modifier_map: &HashMap<KeyChordPart, bool>,
    key_chord_part: KeyChordPart,
) -> bool {
    modifier_map.contains_key(&key_chord_part)
}

fn set_modifier_state(
    modifier_map: &mut HashMap<KeyChordPart, bool>,
    key_chord_part: KeyChordPart,
    state: bool,
) {
    let reference = modifier_map.get_mut(&key_chord_part).unwrap();

    *reference = state;
}

fn construct_key_chord(
    map: &HashMap<KeyChordPart, bool>,
    event: KeyboardEvent,
) -> KeyChord {
    let modifier_keys = map
        .iter()
        .filter(|(_, pressed)| **pressed)
        .map(|(key, _)| *key)
        .collect();

    let ordinary_key = KeyChordPart::Key2(event.get_keyboard_id(), event.get_key_id());

    KeyChord::new(modifier_keys, ordinary_key)
}

pub struct KeyChordProducer {
    keyboard_listener_aggregator: KeyboardListenerAggregator,
    modifier_keys: Vec<KeyChordPart>,
}

impl KeyChordProducer {
    pub fn new(settings: &KeyChordProducerSettings) -> KeyChordProducer {
        let keyboards = settings.get_keyboards();
        let modifiers = settings.get_modifiers();

        let mut keyboard_listener_aggregator = KeyboardListenerAggregator::new();

        for (id, path) in keyboards.iter().enumerate() {
            let listener = KeyboardListener::new(KeyboardId::new(id as u16), path.clone());

            keyboard_listener_aggregator.add_keyboard_listener(listener);
        }

        let mut key_chord_producer = KeyChordProducer {
            keyboard_listener_aggregator,
            modifier_keys: Vec::new(),
        };

        for modifier in modifiers {
            key_chord_producer.modifier_keys.push(*modifier)
        }

        key_chord_producer
    }

    pub fn start_listening(self) -> (mpsc::Receiver<KeyChordEvent>, mpsc::Sender<()>) {
        let modifier_keys = self.modifier_keys.clone();

        let (
            keyboard_event_sender,
            keyboard_event_receiver
        ) = mpsc::channel();

        let (
            key_chord_event_sender,
            key_chord_event_receiver
        ) = mpsc::channel();

        let (
            stop_sender,
            stop_receiver
        ) = mpsc::channel();

        let keyboard_listener_aggregator_handle =
            self.keyboard_listener_aggregator.start_listening(keyboard_event_sender);

        thread::spawn(move || {
            let modifier_keys = modifier_keys;
            let mut modifier_map = HashMap::new();

            for modifier_key in modifier_keys {
                modifier_map.insert(modifier_key, false);
            }

            loop {
                match keyboard_event_receiver.try_recv() {
                    Ok(keyboard_event) => {
                        let key_chord_part = KeyChordPart::Key2(
                            keyboard_event.get_keyboard_id(),
                            keyboard_event.get_key_id(),
                        );

                        if is_modifier_event(&mut modifier_map, key_chord_part) {
                            match keyboard_event.get_event_type() {
                                KeyboardEventType::PRESSED => {
                                    set_modifier_state(&mut modifier_map, key_chord_part, true);
                                }
                                KeyboardEventType::RELEASED => {
                                    set_modifier_state(&mut modifier_map, key_chord_part, false);
                                }
                                KeyboardEventType::UNKNOWN => { }
                            }
                        } else {
                            if keyboard_event.get_event_type() == KeyboardEventType::PRESSED {
                                continue;
                            }

                            let key_chord = construct_key_chord(&modifier_map, keyboard_event);
                            let key_chord_event = KeyChordEvent::from(key_chord);

                            match key_chord_event_sender.send(key_chord_event) {
                                Ok(()) => {}
                                Err(_) => {
                                    key_chord_producer_elog!("Key chord producer channel is destructed. Exiting...");
                                    break;
                                },
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        key_chord_producer_elog!("Key chord producer channel is destructed. Exiting...");
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
                        key_chord_producer_elog!("Key chord producer channel is destructed. Exiting...");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                thread::sleep(Duration::from_millis(10));
            }

            keyboard_listener_aggregator_handle.stop()
        });

        (key_chord_event_receiver, stop_sender)
    }
}
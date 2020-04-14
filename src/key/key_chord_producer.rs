use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

use crate::key::{KeyChordProducerSettings, KeyboardId, KeyChord, KeyboardListenerAggregator, KeyboardListener, KeyboardEventType};
use crate::key::keyboard_event::KeyboardEvent;
use crate::key::key_chord_event::KeyChordEvent;
use crate::key::key_chord::KeyChordPart;

fn is_modifier_event(
    map: &HashMap<KeyChordPart, bool>,
    key_chord_part: KeyChordPart
) -> bool {
    map.contains_key(&key_chord_part)
}

fn set_modifier_state(
    map: &mut HashMap<KeyChordPart, bool>,
    key_chord_part: KeyChordPart,
    state: bool
) {
    let reference = map.get_mut(&key_chord_part).unwrap();

    *reference = state;
}

fn construct_key_chord(map: &HashMap<KeyChordPart, bool>, event: KeyboardEvent) -> KeyChord {
    let modifier_keys = map.iter()
        .filter(|(_, pressed)| **pressed)
        .map(|(key, _)| *key)
        .collect();

    let ordinary_key = KeyChordPart::Key2(
        event.get_keyboard_id(),
        event.get_key_id(),
    );

    KeyChord::new(
        modifier_keys,
        ordinary_key
    )
}

pub struct KeyChordProducer {
    listener: KeyboardListenerAggregator,
    modifier_keys: Vec<KeyChordPart>,
}

impl KeyChordProducer {
    pub fn new(settings: &KeyChordProducerSettings) -> KeyChordProducer {
        let keyboards = settings.get_keyboards();
        let modifiers = settings.get_modifiers();

        let mut keyboard_listener_aggregator = KeyboardListenerAggregator::new();

        for (id, path) in keyboards.iter().enumerate() {
            let listener = KeyboardListener::new(
                KeyboardId::new(id),
                path.clone()
            );

            keyboard_listener_aggregator.add_keyboard_listener(listener);
        }

        let mut key_chord_producer = KeyChordProducer {
            listener: keyboard_listener_aggregator,
            modifier_keys: Vec::new()
        };

        for modifier in modifiers {
            key_chord_producer.modifier_keys.push(*modifier)
        }

        key_chord_producer
    }

    pub fn start_listening(&self) -> mpsc::Receiver<KeyChordEvent> {
        let modifier_keys = self.modifier_keys.clone();

        let (
            tx,
            rx
        ) = mpsc::channel();

        let (
            tx2,
            rx2
        ) = mpsc::channel();

        self.listener.start_listening(tx);

        thread::spawn(move || {
            let modifier_keys = modifier_keys;

            let mut modifier_map = HashMap::new();

            for modifier_key in modifier_keys {
                modifier_map.insert(modifier_key, false);
            }

            loop {
                let keyboard_event = rx.recv().unwrap();
                let key_chord_part = KeyChordPart::Key2(
                    keyboard_event.get_keyboard_id(),
                    keyboard_event.get_key_id()
                );

                if is_modifier_event(&mut modifier_map, key_chord_part) {
                    match keyboard_event.get_event_type() {
                        KeyboardEventType::PRESSED => {
                            set_modifier_state(
                                &mut modifier_map,
                                key_chord_part,
                                true
                            )
                        },
                        KeyboardEventType::RELEASED => {
                            set_modifier_state(
                                &mut modifier_map,
                                key_chord_part,
                                false
                            )
                        },
                        _ => {}
                    }
                } else {
                    if keyboard_event.get_event_type() == KeyboardEventType::PRESSED {
                        continue;
                    }

                    let key_chord = construct_key_chord(
                        &modifier_map,
                        keyboard_event
                    );

                    let key_chord_event = key_chord.into_event();

                    tx2.send(key_chord_event)
                        .expect("Failed to send key chord to overall listener.");
                }
            }
        });

        rx2
    }
}
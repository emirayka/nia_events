use std::hash::{Hash, Hasher};

use crate::input_listeners::{KeyboardId, KeyId, KeyChordProducerSettings};
use crate::input_listeners::key::key_chord_event::KeyChordEvent;

#[derive(Clone, Copy, Debug, Eq)]
pub enum KeyChordPart {
    Key1(KeyId),
    Key2(KeyboardId, KeyId),
}

impl KeyChordPart {
    pub fn get_key_id(&self) -> KeyId {
        match self {
            KeyChordPart::Key1(key_id) => *key_id,
            KeyChordPart::Key2(_, key_id) => *key_id,
        }
    }
}

impl Hash for KeyChordPart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            KeyChordPart::Key1(key_id) => key_id.hash(state),
            KeyChordPart::Key2(_, key_id) => key_id.hash(state),
        }
    }
}

impl PartialEq for KeyChordPart {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KeyChordPart::Key1(key_id_1), KeyChordPart::Key1(key_id_2)) => {
                key_id_1 == key_id_2
            },
            (KeyChordPart::Key1(key_id_1), KeyChordPart::Key2(_, key_id_2)) => {
                key_id_1 == key_id_2
            },
            (KeyChordPart::Key2(_, key_id_1), KeyChordPart::Key1(key_id_2)) => {
                key_id_1 == key_id_2
            },
            (KeyChordPart::Key2(keyboard_id_1, key_id_1), KeyChordPart::Key2(keyboard_id_2, key_id_2)) => {
                keyboard_id_1 == keyboard_id_2 && key_id_1 == key_id_2
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct KeyChord {
    modifiers: Vec<KeyChordPart>,
    key: KeyChordPart,
}

impl PartialEq for KeyChord {
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key {
            return false;
        }

        if self.modifiers.len() != other.modifiers.len() {
            return false;
        }

        for key_chord_part in &self.modifiers {
            if !other.modifiers.contains(key_chord_part) {
                return false;
            }
        }

        true
    }
}

impl KeyChord {
    pub fn new(modifiers: Vec<KeyChordPart>, key: KeyChordPart) -> KeyChord {
        KeyChord {
            modifiers,
            key,
        }
    }

    pub fn get_modifiers(&self) -> &Vec<KeyChordPart> {
        &self.modifiers
    }

    pub fn get_key(&self) -> &KeyChordPart {
        &self.key
    }

    pub fn into_event(self) -> KeyChordEvent {
        KeyChordEvent::new(self)
    }
}
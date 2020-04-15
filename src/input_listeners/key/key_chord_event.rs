use crate::input_listeners::KeyChord;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyChordEvent {
    key_chord: KeyChord,
}

impl KeyChordEvent {
    pub fn new(key_chord: KeyChord) -> KeyChordEvent {
        KeyChordEvent {
            key_chord,
        }
    }

    pub fn into_key_chord(self) -> KeyChord {
        self.key_chord
    }
}


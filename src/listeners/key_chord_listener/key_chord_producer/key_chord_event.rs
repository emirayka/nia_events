use crate::enums::KeyChord;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyChordEvent {
    key_chord: KeyChord,
}

impl KeyChordEvent {
    pub fn new(key_chord: KeyChord) -> KeyChordEvent {
        KeyChordEvent { key_chord }
    }

    pub fn into_key_chord(self) -> KeyChord {
        self.key_chord
    }
}

impl From<KeyChord> for KeyChordEvent {
    fn from(key_chord: KeyChord) -> Self {
        KeyChordEvent::new(key_chord)
    }
}

impl From<KeyChordEvent> for KeyChord {
    fn from(key_chord_event: KeyChordEvent) -> Self {
        key_chord_event.key_chord
    }
}

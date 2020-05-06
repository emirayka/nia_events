use crate::enums::KeyChord;

#[derive(Clone, Debug)]
pub enum Event {
    KeyChordEvent(KeyChord),
}

use crate::key::KeyChord;

#[derive(Clone, Debug)]
pub enum Event {
    KeyChordEvent(KeyChord),
}
use crate::input_listeners::KeyChord;

#[derive(Clone, Debug)]
pub enum Event {
    KeyChordEvent(KeyChord),
}
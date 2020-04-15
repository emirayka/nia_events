use crate::input_listeners::KeyChord;

#[derive(Clone, Debug)]
pub enum KeyCommand {
    ForwardKeyChord(KeyChord),
}

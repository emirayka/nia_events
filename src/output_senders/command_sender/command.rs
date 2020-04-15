use crate::input_listeners::KeyChord;
use crate::output_senders::KeyCommand;

#[derive(Clone, Debug)]
pub enum Command {
    KeyCommand(KeyCommand),
}

impl Command {
    pub fn forward_key_chord(key_chord: KeyChord) -> Command {
        Command::KeyCommand(KeyCommand::ForwardKeyChord(key_chord))
    }
}
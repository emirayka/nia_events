use crate::input_listeners::KeyChord;
use crate::output_senders::UinputCommand;

#[derive(Clone, Debug)]
pub enum Command {
    UinputCommand(UinputCommand),
    Spawn(String),
    Wait(u64),
}

impl Command {
    pub fn forward_key_chord(key_chord: KeyChord) -> Command {
        Command::UinputCommand(UinputCommand::ForwardKeyChord(key_chord))
    }
}
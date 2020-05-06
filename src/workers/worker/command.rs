use crate::enums::KeyChord;

use crate::UInputWorkerCommand;
use crate::XorgWorkerCommand;

#[derive(Clone, Debug)]
pub enum Command {
    UInput(UInputWorkerCommand),
    Xorg(XorgWorkerCommand),
    Spawn(String),
    Wait(u64),
}

impl Command {
    pub fn forward_key_chord(key_chord: KeyChord) -> Command {
        Command::UInput(UInputWorkerCommand::ForwardKeyChord(key_chord))
    }
}

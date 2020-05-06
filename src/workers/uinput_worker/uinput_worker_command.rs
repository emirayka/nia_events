use crate::enums::ButtonId;
use crate::enums::KeyChord;
use crate::enums::KeyId;

#[derive(Clone, Debug)]
pub enum UInputWorkerCommand {
    ForwardKeyChord(KeyChord),
    KeyDown(KeyId),
    KeyPress(KeyId),
    KeyUp(KeyId),
    MouseButtonDown(ButtonId),
    MouseButtonPress(ButtonId),
    MouseButtonUp(ButtonId),
}

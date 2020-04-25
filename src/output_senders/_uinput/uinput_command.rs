use crate::input_listeners::KeyChord;
use crate::KeyId;
use crate::ButtonId;

#[derive(Clone, Debug)]
pub enum UinputCommand {
    ForwardKeyChord(KeyChord),
    KeyDown(KeyId),
    KeyPress(KeyId),
    KeyUp(KeyId),
    TextType(String),
    MouseButtonDown(ButtonId),
    MouseButtonPress(ButtonId),
    MouseButtonUp(ButtonId),
    MouseMoveBy(i16, i16),
    MouseMoveTo(i16, i16),
}

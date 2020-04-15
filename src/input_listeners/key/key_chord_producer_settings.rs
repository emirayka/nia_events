use crate::input_listeners::KeyChordPart;

pub struct KeyChordProducerSettings {
    keyboards: Vec<String>,
    modifiers: Vec<KeyChordPart>,
}

impl KeyChordProducerSettings {
    pub fn new(keyboards: Vec<String>, modifiers: Vec<KeyChordPart>) -> KeyChordProducerSettings {
        KeyChordProducerSettings {
            keyboards,
            modifiers,
        }
    }

    pub fn get_keyboards(&self) -> &Vec<String> {
        &self.keyboards
    }

    pub fn get_modifiers(&self) -> &Vec<KeyChordPart> {
        &self.modifiers
    }
}


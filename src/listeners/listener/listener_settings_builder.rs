use crate::enums::KeyChordPart;

use crate::ListenerSettings;
use crate::KeyChordProducerSettings;

pub struct ListenerSettingsBuilder {
    keyboards: Vec<String>,
    modifiers: Vec<KeyChordPart>,
}

impl ListenerSettingsBuilder {
    pub fn new() -> ListenerSettingsBuilder {
        ListenerSettingsBuilder {
            keyboards: Vec::new(),
            modifiers: Vec::new(),
        }
    }

    pub fn add_keyboard(mut self, keyboard: String) -> ListenerSettingsBuilder {
        self.keyboards.push(keyboard);

        self
    }

    pub fn add_modifier(mut self, modifier: KeyChordPart) -> ListenerSettingsBuilder {
        self.modifiers.push(modifier);

        self
    }

    pub fn build(self) -> ListenerSettings {
        let key_chord_producer_settings =
            KeyChordProducerSettings::new(self.keyboards, self.modifiers);

        ListenerSettings::new(key_chord_producer_settings)
    }
}
use crate::key::{KeyChordPart, KeyChordProducerSettings};

pub struct EventListenerSettings {
    key_chord_producer_settings: KeyChordProducerSettings,
}

impl EventListenerSettings {
    pub fn new(key_chord_producer_settings: KeyChordProducerSettings) -> EventListenerSettings {
        EventListenerSettings {
            key_chord_producer_settings
        }
    }

    pub fn get_key_chord_producer_settings(&self) -> &KeyChordProducerSettings {
        &self.key_chord_producer_settings
    }
}

pub struct EventListenerSettingsBuilder {
    keyboards: Vec<String>,
    modifiers: Vec<KeyChordPart>,
}

impl EventListenerSettingsBuilder {
    pub fn new() -> EventListenerSettingsBuilder {
        EventListenerSettingsBuilder {
            keyboards: Vec::new(),
            modifiers: Vec::new(),
        }
    }

    pub fn add_keyboard(mut self, keyboard: String) -> EventListenerSettingsBuilder {
        self.keyboards.push(keyboard);

        self
    }

    pub fn add_modifier(mut self, modifier: KeyChordPart) -> EventListenerSettingsBuilder {
        self.modifiers.push(modifier);

        self
    }

    pub fn build(self) -> EventListenerSettings {
        let key_chord_producer_settings = KeyChordProducerSettings::new(
            self.keyboards,
            self.modifiers
        );

        EventListenerSettings::new(
            key_chord_producer_settings
        )
    }
}

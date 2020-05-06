use crate::listeners::KeyChordProducerSettings;

pub struct ListenerSettings {
    key_chord_producer_settings: KeyChordProducerSettings,
}

impl ListenerSettings {
    pub fn new(key_chord_producer_settings: KeyChordProducerSettings) -> ListenerSettings {
        ListenerSettings {
            key_chord_producer_settings,
        }
    }

    pub fn get_key_chord_producer_settings(&self) -> &KeyChordProducerSettings {
        &self.key_chord_producer_settings
    }
}


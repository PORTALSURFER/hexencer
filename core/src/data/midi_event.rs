use super::midi_message::MidiMessage;

#[derive(Debug)]
pub struct MidiRequest {
    message: MidiMessage,
    port: u8,
}

impl MidiRequest {
    pub fn get_message(&self) -> &MidiMessage {
        &self.message
    }
    pub fn get_port(&self) -> u8 {
        self.port
    }
}

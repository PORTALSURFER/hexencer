use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct Instrument {
    pub name: String,
    pub midi_port: u8,
    pub midi_channel: u8,
}

impl Instrument {
    pub fn new(name: &str, midi_port: u8, midi_channel: u8) -> Self {
        Self {
            name: String::from(name),
            midi_port,
            midi_channel,
        }
    }
}

impl Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "[instrument|name:{}, midi_port:{}]",
            self.name, self.midi_port
        ))
    }
}

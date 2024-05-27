use std::fmt::Display;

/// Represents a device to use on 'Track's
/// Think of it as a kind of voice
#[derive(Debug, Clone, Default)]
pub struct Instrument {
    /// the name of the instrument
    pub name: String,
    /// midi port used for the instrument
    pub port: u8,
    /// midi channel used for the instrument
    pub channel: u8,
}

impl Instrument {
    /// creates a new 'Instrument' with the given name, port and channel
    pub fn new(name: &str, port: u8, channel: u8) -> Self {
        Self {
            name: String::from(name),
            port,
            channel,
        }
    }
}

impl Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("name:{}, port:{}", self.name, self.port))
    }
}

use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct Instrument {
    pub name: String,
    pub port: u8,
    pub channel: u8,
}

impl Instrument {
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

use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub index: u8,
    pub channel: u8,
    pub velocity: u8,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            index: 48,
            channel: 0,
            velocity: 96,
        }
    }
}

impl Note {
    pub fn new(index: u8, channel: u8, velocity: u8) -> Self {
        Self {
            index,
            channel,
            velocity,
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "[note|index: {}, channel: {}, velocity: {}",
                self.index, self.channel, self.velocity
            )
            .as_str(),
        )
    }
}

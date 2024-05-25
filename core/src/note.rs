use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct NoteEvent {
    pub key: u8,
    pub length: u32,
    pub velocity: u8,
}

impl Default for NoteEvent {
    fn default() -> Self {
        Self {
            key: 48,
            length: 120,
            velocity: 96,
        }
    }
}

impl NoteEvent {
    pub fn new(on: bool, index: u8, length: u32, velocity: u8) -> Self {
        Self {
            key: index,
            length,
            velocity,
        }
    }
}

impl Display for NoteEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "key: {}, length: {}, velocity: {}",
                self.key, self.length, self.velocity
            )
            .as_str(),
        )
    }
}

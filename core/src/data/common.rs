use std::fmt::Display;

/// bits for midi note on message
pub const NOTE_ON_MSG: u8 = 0x90;

/// bits for midi all notes off message
pub const ALL_NOTE_ON_MSG: u8 = 0xB0;
/// bits for midi note off message
pub const NOTE_OFF_MSG: u8 = 0x80;

/// id used to identify persistant objects like those stored in a project
#[derive(Default, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct DataId(uuid::Uuid);
impl DataId {
    /// creates a new 'Id'
    pub fn new() -> DataId {
        DataId(uuid::Uuid::new_v4())
    }

    /// get as a slice of bytes
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
}

impl Display for DataId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl PartialEq for DataId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

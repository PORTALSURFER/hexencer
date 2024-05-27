use super::{common::DataId, event_list::EventList};

/// A clip is a collection of events
/// They house things like notes and automation data
#[derive(Default)]
pub struct Clip {
    id: DataId,
    /// visual name of the clip
    pub name: String,
    events: EventList,
}

impl Clip {
    pub fn new(name: &str) -> Self {
        Self {
            id: DataId::new(),
            name: String::from(name),
            events: EventList::new(),
        }
    }

    pub fn id_as_string(&self) -> String {
        self.id.to_string()
    }

    pub fn get_id(&self) -> DataId {
        self.id.clone()
    }
}

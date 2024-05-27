use super::{common::Id, event_list::EventList};

/// A clip is a collection of events
/// They house things like notes and automation data
#[derive(Default)]
pub struct Clip {
    _id: Id,
    /// visual name of the clip
    pub name: String,
    _events: EventList,
}

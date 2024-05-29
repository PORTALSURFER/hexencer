use crate::{
    event::{Event, EventType},
    Tick,
};

use super::{common::DataId, event_list::EventList, MidiMessage};

/// A clip is a collection of events
/// They house things like notes and automation data
#[derive(Default)]
pub struct Clip {
    id: DataId,
    /// visual name of the clip
    pub name: String,
    /// notes in this clip
    pub events: EventList,
}

impl Clip {
    /// Create a new clip
    pub fn new(name: &str) -> Self {
        let mut test_events = EventList::new();

        let event1 = Event::new(
            DataId::new(),
            EventType::Midi(MidiMessage::NoteOn {
                key: 48,
                velocity: 64,
            }),
            true,
        );
        let event2 = Event::new(
            DataId::new(),
            EventType::Midi(MidiMessage::NoteOn {
                key: 47,
                velocity: 64,
            }),
            true,
        );
        test_events.add_event(Tick::from(20), event1);
        test_events.add_event(Tick::from(28), event2);

        let test_clip = Self {
            id: DataId::new(),
            name: String::from(name),
            events: test_events,
        };

        return test_clip;

        Self {
            id: DataId::new(),
            name: String::from(name),
            events: EventList::new(),
        }
    }
    /// get this clip's id as a string
    pub fn id_as_string(&self) -> String {
        self.id.to_string()
    }

    /// get a clone of this clips id
    pub fn get_id(&self) -> DataId {
        self.id.clone()
    }
}

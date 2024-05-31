use super::{
    common::DataId,
    event_list::{EventList, EventSegment},
    MidiMessage,
};
use crate::{
    event::{Event, EventType},
    Tick,
};

/// A clip is a collection of events
/// They house things like notes and automation data
#[derive(Default, Debug)]
pub struct Clip {
    /// id used to identify data objects
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

        let event1 = EventSegment::new(
            DataId::new(),
            64,
            Tick::from(0),
            Tick::from(480),
            EventType::Midi(MidiMessage::NoteOn {
                key: 46,
                velocity: 64,
            }),
            true,
        );
        let event2 = EventSegment::new(
            DataId::new(),
            64,
            Tick::from(240),
            Tick::from(700),
            EventType::Midi(MidiMessage::NoteOn {
                key: 47,
                velocity: 64,
            }),
            true,
        );
        let event3 = EventSegment::new(
            DataId::new(),
            64,
            Tick::from(960),
            Tick::from(1440),
            EventType::Midi(MidiMessage::NoteOn {
                key: 50,
                velocity: 64,
            }),
            true,
        );
        test_events.add_event(Tick::from(0), event1);
        test_events.add_event(Tick::from(480), event2);
        test_events.add_event(Tick::from(960), event3);

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
        self.id
    }
}

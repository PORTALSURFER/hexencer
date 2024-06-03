use std::{collections::BTreeMap, ops::Deref};

use super::{
    common::DataId,
    event_list::{EventList, EventSegment},
    MidiMessage,
};
use crate::{event::EventType, Tick};

/// a collection of clips, used on tracks
#[derive(Default, Debug)]
pub struct ClipCollection {
    /// inner object housing the clips
    inner: BTreeMap<Tick, Clip>,
}

impl ClipCollection {
    /// interst a new clip to the collection
    pub fn insert(&mut self, tick: Tick, clip: Clip) {
        self.inner.insert(tick, clip);
    }

    /// returns an iterator over the clips in this collection
    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, Tick, Clip> {
        self.inner.iter()
    }

    /// returns an iterator over the clips in this collection
    pub fn into_iter(self) -> std::collections::btree_map::IntoIter<Tick, Clip> {
        self.inner.into_iter()
    }

    /// creates a new, empty, 'ClipCollection'
    pub fn new() -> ClipCollection {
        ClipCollection {
            inner: BTreeMap::new(),
        }
    }
}

impl Deref for ClipCollection {
    type Target = BTreeMap<Tick, Clip>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> IntoIterator for &'a ClipCollection {
    type Item = (&'a Tick, &'a Clip);
    type IntoIter = std::collections::btree_map::Iter<'a, Tick, Clip>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for ClipCollection {
    type Item = (Tick, Clip);

    type IntoIter = std::collections::btree_map::IntoIter<Tick, Clip>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

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
    /// end of the clip
    pub end: u64,
}

impl Clip {
    /// Create a new clip
    pub fn new(name: &str, end: u64) -> Self {
        let mut test_events = EventList::new();

        let event1 = EventSegment::new(
            DataId::new(),
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
            end,
        };

        return test_clip;

        Self {
            id: DataId::new(),
            name: String::from(name),
            events: EventList::new(),
            end: 1920,
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

use std::{collections::BTreeMap, fmt::Display, ops::Deref};

use super::{
    common::DataId,
    event_list::{EventCollection, EventSegment},
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

    /// returns the clip at the given tick, if any
    pub fn find_take(&mut self, clip_id: &ClipId) -> Option<Clip> {
        let key_to_remove = self.inner.iter().find_map(|(key, clip)| {
            if clip.get_id() == clip_id {
                Some(*key)
            } else {
                None
            }
        });

        key_to_remove.and_then(|key| self.inner.remove(&key))
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
    id: ClipId,
    /// visual name of the clip
    pub name: String,
    /// notes in this clip
    pub events: EventCollection,
    /// end of the clip
    pub end: u64,
}

impl Clip {
    /// Create a new clip
    pub fn new(name: &str, end: u64) -> Self {
        let mut test_events = EventCollection::new();

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

        Self {
            id: ClipId::new(),
            name: String::from(name),
            events: test_events,
            end,
        }

        // Self {
        //     id: ClipId::new(),
        //     name: String::from(name),
        //     events: EventList::new(),
        //     end: 1920,
        // }
    }
    /// get this clip's id as a string
    pub fn id_as_string(&self) -> String {
        self.id.to_string()
    }

    /// get a clone of this clips id
    pub fn get_id(&self) -> &ClipId {
        &self.id
    }
}

/// data identifier of a clip
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClipId(DataId);

impl Deref for ClipId {
    type Target = DataId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ClipId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ClipId {
    /// creates a new clip id
    pub(crate) fn new() -> Self {
        Self(DataId::new())
    }
}

impl PartialEq<ClipId> for &ClipId {
    fn eq(&self, other: &ClipId) -> bool {
        self.0 == other.0
    }
}

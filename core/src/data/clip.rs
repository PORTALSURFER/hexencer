use std::{
    collections::{
        hash_map::{IntoIter, Iter},
        HashMap,
    },
    fmt::Display,
    ops::Deref,
};

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
    inner: HashMap<ClipId, Clip>,
}

impl ClipCollection {
    /// interst a new clip to the collection
    pub fn insert(&mut self, clip: Clip) {
        self.inner.insert(clip.id(), clip);
    }

    /// returns an iterator over the clips in this collection
    pub fn iter(&self) -> Iter<'_, ClipId, Clip> {
        self.inner.iter()
    }

    /// returns an iterator over the clips in this collection
    pub fn into_iter(self) -> IntoIter<ClipId, Clip> {
        self.inner.into_iter()
    }

    /// creates a new, empty, 'ClipCollection'
    pub fn new() -> ClipCollection {
        ClipCollection {
            inner: HashMap::new(),
        }
    }

    /// returns the clip at the given tick, if any
    pub fn find_take(&mut self, clip_id: ClipId) -> Option<Clip> {
        let key_to_remove = self.inner.iter().find_map(|(key, clip)| {
            if clip.id() == clip_id {
                Some(*key)
            } else {
                None
            }
        });

        key_to_remove.and_then(|key| self.inner.remove(&key))
    }
}

impl Deref for ClipCollection {
    type Target = HashMap<ClipId, Clip>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> IntoIterator for &'a ClipCollection {
    type Item = (&'a ClipId, &'a Clip);
    type IntoIter = Iter<'a, ClipId, Clip>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for ClipCollection {
    type Item = (ClipId, Clip);

    type IntoIter = IntoIter<ClipId, Clip>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

/// A clip is a collection of events
/// They house things like notes and automation data
#[derive(Default, Debug)]
pub struct Clip {
    /// tick at which the clip starts
    pub start: Tick,
    /// id used to identify data objects
    id: ClipId,
    /// visual name of the clip
    pub name: String,
    /// notes in this clip
    pub events: EventCollection,
    /// end of the clip
    pub length: Tick,
}

impl Clip {
    /// Create a new clip
    pub fn new(tick: Tick, name: &str, length: Tick) -> Self {
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
            start: tick,
            id: ClipId::new(),
            name: String::from(name),
            events: test_events,
            length,
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
    pub fn id(&self) -> ClipId {
        self.id
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
    pub fn new() -> Self {
        Self(DataId::new())
    }
}

impl PartialEq<ClipId> for &ClipId {
    fn eq(&self, other: &ClipId) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
}

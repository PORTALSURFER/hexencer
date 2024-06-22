use std::{
    collections::{
        btree_map::{IntoIter, Iter},
        BTreeMap,
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

/// key type used in clip collections
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ClipKey {
    /// start tick of the clip
    pub start: Tick,
    /// id of the clip
    pub id: ClipId,
}

impl From<&Clip> for ClipKey {
    fn from(value: &Clip) -> Self {
        Self {
            start: value.start,
            id: value.id,
        }
    }
}

/// a collection of clips, used on tracks
#[derive(Default, Debug)]
pub struct ClipCollection {
    /// inner object housing the clips
    inner: BTreeMap<ClipKey, Clip>,
}

impl ClipCollection {
    /// interst a new clip to the collection
    pub fn insert(&mut self, new_clip: Clip) {
        //TODO figure out what is happening here

        // calculate end tick of the new clip
        let new_end = new_clip.start + new_clip.duration;

        // create a new clip key from the new clip
        let overlapped_clip_key = ClipKey::from(&new_clip);

        let new_clip_key = ClipKey::from(&new_clip);
        let end_clip_key = ClipKey {
            start: new_clip_key.start + new_clip.duration,
            id: new_clip.id(),
        };

        //get inner clip
        let inner_clip: Vec<_> = self
            .inner
            .range(new_clip_key..=end_clip_key)
            .map(|(&k, v)| (k, v.clone()))
            .collect();
        println!("inner clip {:?}", inner_clip);
        for (key, clip) in inner_clip {
            if clip.start > new_clip.start && clip.end() < new_clip.end() {
                self.inner.remove(&key);
            }
        }

        // get the overlapping clip
        let overlapping: Vec<_> = self
            .inner
            .range(..=overlapped_clip_key)
            .filter(|(_, clip)| clip.start + clip.duration > new_clip.start)
            .map(|(&k, v)| (k, v.clone()))
            .collect();

        for (start, mut overlap_clip) in overlapping {
            if overlap_clip.start < new_clip.start {
                // Split the beginning part
                let left_duration = new_clip.start - overlap_clip.start;
                let mut left_clip = overlap_clip.clone();
                left_clip.id = ClipId::new();
                left_clip.duration = left_duration;
                self.inner.insert(ClipKey::from(&left_clip), left_clip);

                overlap_clip.start = new_clip.start;
                overlap_clip.duration -= left_duration;
            }
            if overlap_clip.start + overlap_clip.duration > new_end {
                // Split the end part
                let right_start = new_end;
                let right_duration = overlap_clip.start + overlap_clip.duration - new_end;
                let mut right_clip = overlap_clip.clone();
                right_clip.id = ClipId::new();
                right_clip.start = right_start;
                right_clip.duration = right_duration;
                self.inner.insert(ClipKey::from(&right_clip), right_clip);
                overlap_clip.duration = new_end - overlap_clip.start;
            }

            // Remove the original clip
            self.inner.remove(&start);
        }

        // Insert the new clip
        self.inner.insert(ClipKey::from(&new_clip), new_clip);
    }

    /// returns an iterator over the clips in this collection
    pub fn iter(&self) -> Iter<'_, ClipKey, Clip> {
        self.inner.iter()
    }

    /// returns an iterator over the clips in this collection
    pub fn into_iter(self) -> IntoIter<ClipKey, Clip> {
        self.inner.into_iter()
    }

    /// creates a new, empty, 'ClipCollection'
    pub fn new() -> ClipCollection {
        ClipCollection {
            inner: BTreeMap::new(),
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
    type Target = BTreeMap<ClipKey, Clip>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> IntoIterator for &'a ClipCollection {
    type Item = (&'a ClipKey, &'a Clip);
    type IntoIter = Iter<'a, ClipKey, Clip>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for ClipCollection {
    type Item = (ClipKey, Clip);

    type IntoIter = IntoIter<ClipKey, Clip>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

/// A clip is a collection of events
/// They house things like notes and automation data
#[derive(Default, Debug, Clone)]
pub struct Clip {
    /// tick at which the clip starts
    pub start: Tick,
    /// id used to identify data objects
    id: ClipId,
    /// visual name of the clip
    pub name: Box<String>,
    /// notes in this clip
    pub events: EventCollection,
    /// length of the clip
    pub duration: Tick,
}

impl Clip {
    /// Create a new clip
    pub fn new(start: Tick, name: &str, duration: Tick) -> Self {
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
            start,
            id: ClipId::new(),
            name: Box::new(String::from(name)),
            events: test_events,
            duration,
        }
    }
    /// get this clip's id as a string
    pub fn id_as_string(&self) -> String {
        self.id.to_string()
    }

    /// get a clone of this clips id
    pub fn id(&self) -> ClipId {
        self.id
    }

    /// get the end tick of the clip
    pub fn end(&self) -> Tick {
        self.start + self.duration
    }
}

/// data identifier of a clip
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
mod tests {}

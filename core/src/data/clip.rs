use std::{
    collections::{
        btree_map::{IntoIter, Iter},
        BTreeMap,
    },
    fmt::Display,
    ops::{Deref, DerefMut},
};

use tracing::info;

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
    inner: BTreeMap<Tick, Clip>,
}

impl ClipCollection {
    /// insert a new clip to the collection
    pub fn insert(&mut self, new_clip: Clip) {
        // create a new clip key from the new clip
        self.clip_partial_overlapped_clips_to_the_right(&new_clip);
        let overlapped_outer = self.get_surrounded_overlapped_clips(&new_clip);
        self.split_overlapped_clip(overlapped_outer, &new_clip);

        // Insert the new clip
        self.inner.insert(ClipKey::from(&new_clip), new_clip);
    }

    /// splits a clip if it is overlapped by another clip
    fn split_overlapped_clip(&mut self, overlapped_outer: Vec<(ClipKey, Clip)>, new_clip: &Clip) {
        for (overlap_clip_key, mut overlap_clip) in overlapped_outer {
            self.insert_left_of_tick(&mut overlap_clip, &new_clip.start);
            self.insert_right_of_tick(overlap_clip, &new_clip.end());

            // Remove the original clip because it has been split into 2 new clips
            self.inner.remove(&overlap_clip_key);
        }
    }

    /// clips a partially overlapped clip
    fn clip_partial_overlapped_clips_to_the_right(&mut self, new_clip: &Clip) {
        let inner_clip = self.get_overlapped_inner(new_clip);
        for (key, clip) in inner_clip {
            info!("Clipping partial overlapped clips");
            self.remove_overlapped(&clip, new_clip, key);
            self.split_right_clip(clip, new_clip, key);
        }
    }

    /// adds a clip to the right of the overlapping clip, splitting the original
    fn insert_right_of_tick(&mut self, mut overlapped_clip: Clip, tick: &Tick) {
        if overlapped_clip.start + overlapped_clip.duration > *tick {
            // Split the end part
            let right_duration = overlapped_clip.start + overlapped_clip.duration - *tick;
            let mut right_clip = overlapped_clip.clone();
            right_clip.id = ClipId::new();
            right_clip.start = *tick;
            right_clip.duration = right_duration;
            self.inner.insert(ClipKey::from(&right_clip), right_clip);
            overlapped_clip.duration = *tick - overlapped_clip.start;
        }
    }

    /// adds a clip to the left of the overlapping clip, splitting the original
    fn insert_left_of_tick(&mut self, overlapped_clip: &mut Clip, tick: &Tick) {
        if overlapped_clip.start < *tick {
            // Split the beginning part
            let left_duration = *tick - overlapped_clip.start;
            let mut left_clip = overlapped_clip.clone();
            left_clip.id = ClipId::new();
            left_clip.duration = left_duration;
            self.inner.insert(ClipKey::from(&left_clip), left_clip);

            overlapped_clip.start = *tick;
            overlapped_clip.duration -= left_duration;
        }
    }

    /// gets the overlapping clips surrounded the sample
    fn get_surrounded_overlapped_clips(&mut self, sample: &Clip) -> Vec<(ClipKey, Clip)> {
        let overlapped_clip_key = ClipKey::from(sample);
        // get the overlapping clip
        let overlapping: Vec<_> = self
            .inner
            .range(..=overlapped_clip_key)
            .filter(|(_, clip)| clip.start + clip.duration > sample.start)
            .map(|(&k, v)| (k, v.clone()))
            .collect();
        overlapping
    }

    /// splits a clip on the right side
    fn split_right_clip(&mut self, clip: Clip, new_clip: &Clip, key: ClipKey) {
        // handle partial overlapped clip on the right side
        if clip.start > new_clip.start && clip.end() > new_clip.end() {
            self.inner.remove(&key);
            self.insert_right_of_tick(clip, &new_clip.end());
        }
    }

    /// removes a clip if it is fully overlapped
    fn remove_overlapped(&mut self, clip: &Clip, new_clip: &Clip, key: ClipKey) {
        // handle full overlap
        if clip.start >= new_clip.start && clip.end() < new_clip.end() {
            self.inner.remove(&key);
        }
    }

    /// founds clips which find themselves inside of the sample
    fn get_overlapped_inner(&mut self, sample: &Clip) -> Vec<(ClipKey, Clip)> {
        let new_clip_key = ClipKey::from(sample);
        let end_clip_key = ClipKey {
            start: new_clip_key.start + sample.duration,
            id: sample.id(),
        };

        // handle partial and full overlapping
        let inner_clip: Vec<_> = self
            .inner
            .range(new_clip_key..=end_clip_key)
            .map(|(&k, v)| (k, v.clone()))
            .collect();
        inner_clip
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

impl DerefMut for ClipCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
mod tests {
    use super::*;

    #[test]
    fn create_clipkey_from_clip() {
        let clip = Clip::new(Tick::from(999), "test_clip", Tick::from(2000));
        let clip_key = ClipKey::from(&clip);

        assert_eq!(clip_key.start, Tick::from(999));
        assert_eq!(clip_key.id, clip.id);
    }

    #[test]
    fn can_get_overlapped_clips() {
        // 0-100 overlapped by 50-150
        let mut clip_collection = ClipCollection::new();
        let clip = Clip::new(Tick::from(0), "test_clip", Tick::from(100));
        clip_collection.insert(clip.clone());

        let sample_clip = Clip::new(50.into(), "sample_clip", 100.into());
        let overlapped = clip_collection.get_surrounded_overlapped_clips(&sample_clip);
        assert_eq!(overlapped.len(), 1);
    }

    #[test]
    fn should_ignore_non_overlapped_clips() {
        // 0-50 overlapped by 50-150
        let mut clip_collection = ClipCollection::new();
        let clip = Clip::new(0.into(), "test_clip", 50.into());
        clip_collection.insert(clip.clone());

        let sample_clip = Clip::new(50.into(), "sample_clip", 100.into());
        let overlapped = clip_collection.get_surrounded_overlapped_clips(&sample_clip);
        assert_eq!(overlapped.len(), 0);
    }

    #[test]
    fn should_detect_inner_overlap_when_start_is_equal() {
        // 0-50 overlapped by 50-150
        let mut clip_collection = ClipCollection::new();
        let clip = Clip::new(0.into(), "test_clip", 50.into());
        clip_collection.insert(clip.clone());

        let sample_clip = Clip::new(0.into(), "sample_clip", 100.into());
        let overlapped = clip_collection.get_overlapped_inner(&sample_clip);
        assert_eq!(overlapped.len(), 1);
    }
}

#![deny(missing_docs)]
use super::{
    clip::Clip,
    event_list::{EventList, EventSegment},
};
use crate::{instrument::Instrument, Tick};
use std::{collections::BTreeMap, fmt::Display};

type TrackCollection = Vec<Track>;

#[derive(Default, Debug)]
pub struct Tracks {
    inner: TrackCollection,
}

impl Tracks {
    pub fn add(&mut self, new_track: Track) {
        self.inner.push(new_track);
    }

    pub fn get_track_port(&self, index: usize) -> u8 {
        self.inner.get(index).unwrap().instrument.port
    }

    pub fn tracks(&self) -> &[Track] {
        &self.inner
    }

    pub fn get(&self, index: usize) -> Option<&Track> {
        self.inner.get(index)
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn push(&mut self, track: Track) {
        self.inner.push(track);
    }

    pub(crate) fn pop(&mut self) -> Option<Track> {
        self.inner.pop()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Track> {
        self.inner.get_mut(index)
    }

    pub fn iter(&self) -> std::slice::Iter<Track> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Track> {
        self.inner.iter_mut()
    }
}

#[derive(Default, Debug)]
pub struct Track {
    pub id: usize,
    pub name: String,
    pub instrument: Instrument,
    pub event_list: EventList,
    pub clips: BTreeMap<Tick, Clip>,
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}, instrument: {}", self.name, self.instrument))?;
        Ok(())
    }
}

impl Track {
    pub fn new(id: usize, name: &str) -> Track {
        // test events
        let mut event_list = EventList::new();
        for i in 0..8 {
            let event_block = EventSegment::new_midi(Tick::from(i * 480 as u32), 120, 38, 64);
            event_list.add_event_block(event_block);
        }

        // test clips
        let mut clips = BTreeMap::new();
        for i in 0..4 {
            let clip = Clip::new(&format!("clip_{}", i));
            clips.insert(Tick::from(480 * i), clip);
        }

        Self {
            id,
            name: String::from(name),
            event_list,
            instrument: Instrument::new("port0", 0, 0),
            clips,
        }
    }

    pub fn set_port(&mut self, port: u8) {
        self.instrument.port = port;
    }

    pub fn set_channel(&mut self, channel: u8) {
        self.instrument.channel = channel;
    }

    pub(crate) fn add_clip(&mut self, tick: Tick, name: &str) {
        self.clips.insert(tick, Clip::new(name));
    }
}

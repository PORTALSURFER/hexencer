#![deny(missing_docs)]
use super::{
    clip::Clip,
    event_list::{EventList, EventSegment},
};
use crate::{instrument::Instrument, Tick};
use std::{collections::BTreeMap, fmt::Display};

/// collection of tracks
#[derive(Default, Debug)]
pub struct TrackCollection {
    /// inner vector of tracks
    inner: Vec<Track>,
}

impl TrackCollection {
    /// add a new track to the collection
    pub fn add(&mut self, new_track: Track) {
        self.inner.push(new_track);
    }

    /// get the port used by this track
    pub fn get_track_port(&self, index: usize) -> u8 {
        self.inner.get(index).unwrap().instrument.port
    }

    /// get a reference slice to the track collection
    pub fn tracks(&self) -> &[Track] {
        &self.inner
    }

    /// get a reference to a track at a given index, or 'None' if it doesn't exist
    pub fn get(&self, index: usize) -> Option<&Track> {
        self.inner.get(index)
    }

    /// get the number of tracks in the collection
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    /// push a new track onto the collection
    pub(crate) fn push(&mut self, track: Track) {
        self.inner.push(track);
    }

    /// pops the last track from the collection'
    pub(crate) fn pop(&mut self) -> Option<Track> {
        self.inner.pop()
    }

    /// gets a mutable reference to the track at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Track> {
        self.inner.get_mut(index)
    }

    /// get an iterator over references to the tracks
    pub fn iter(&self) -> std::slice::Iter<Track> {
        self.inner.iter()
    }

    /// get an iterator over mutable references to the tracks
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Track> {
        self.inner.iter_mut()
    }
}

/// track object
#[derive(Default, Debug)]
pub struct Track {
    /// unique id of this track
    pub id: usize,
    /// name of the track, used as label
    pub name: String,
    /// instrument assigned to this track
    pub instrument: Instrument,
    /// events on this track
    #[deprecated(note = "clips now house events")]
    pub event_list: EventList,
    /// clips in this track
    pub clips: BTreeMap<Tick, Clip>,
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}, instrument: {}", self.name, self.instrument))?;
        Ok(())
    }
}

impl Track {
    /// create a new track object
    pub fn new(id: usize, name: &str) -> Track {
        // test events
        let mut event_list = EventList::new();
        for i in 0..8 {
            let event_block = EventSegment::new2(
                Tick::from(i * 480_u32),
                Tick::from(i * 480_u32),
                38,
                64,
                true,
            );
            event_list.add_event_segment(event_block);
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

    /// set the midi port for this track
    pub fn set_port(&mut self, port: u8) {
        self.instrument.port = port;
    }

    /// set the midi channel for this track
    pub fn set_channel(&mut self, channel: u8) {
        self.instrument.channel = channel;
    }

    /// add a new clip to the track
    pub(crate) fn add_clip(&mut self, tick: Tick, name: &str) {
        self.clips.insert(tick, Clip::new(name));
    }
}

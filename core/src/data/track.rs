#![deny(missing_docs)]
use super::clip::{Clip, ClipCollection, ClipId, ClipKey};
use crate::{instrument::Instrument, DataId};
use std::{fmt::Display, ops::Deref};
use thiserror::Error;

/// error type for track collection
#[derive(Error, Debug)]
pub enum TrackCollectionError {
    /// track at index does not exist
    #[error("Track at index {0} does not exist")]
    NoTrack(usize),
}

/// collection of tracks
#[derive(Default, Debug)]
pub struct TrackCollection {
    /// inner vector of tracks
    inner: Vec<Track>,
}

impl TrackCollection {
    /// gets slice of clips at a given track index
    pub fn get_clips(&self, index: usize) -> Result<&ClipCollection, TrackCollectionError> {
        match self.inner.get(index) {
            Some(track) => Ok(&track.clip_collection),
            _ => Err(TrackCollectionError::NoTrack(index)),
        }
    }

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

    /// get by index
    pub fn get(&self, index: usize) -> Option<&Track> {
        self.inner.get(index)
    }

    /// get a reference to a track at a given index, or 'None' if it doesn't exist
    pub fn get_by_id(&self, index: TrackId) -> Option<&Track> {
        self.inner.iter().find(|t| t.id == index)
    }

    /// get the number of tracks in the collection
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    /// push a new track onto the collection
    pub fn push(&mut self, track: Track) {
        self.inner.push(track);
    }

    /// pops the last track from the collection'
    pub(crate) fn pop(&mut self) -> Option<Track> {
        self.inner.pop()
    }

    /// gets a mutable reference to the track with the given id
    pub fn get_mut(&mut self, id: TrackId) -> Option<&mut Track> {
        self.inner.iter_mut().find(|t| t.id == id)
    }

    /// get an iterator over references to the tracks
    pub fn iter(&self) -> std::slice::Iter<Track> {
        self.inner.iter()
    }

    /// get an iterator over mutable references to the tracks
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Track> {
        self.inner.iter_mut()
    }

    /// take the clip out of any track if found, removing it from the track
    pub fn take_clip(&mut self, clip_id: ClipId) -> Option<Clip> {
        for track in &mut self.inner {
            if let result @ Some(_) = track.clip_collection.find_take(clip_id) {
                return result;
            }
        }
        None
    }
}

/// data identifier of a track
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrackId(DataId);

impl Display for TrackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl TrackId {
    /// creates a new track id
    pub(crate) fn new() -> Self {
        Self(DataId::new())
    }
}

impl Deref for TrackId {
    type Target = DataId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// track object
#[derive(Default, Debug)]
pub struct Track {
    /// unique id of this track
    pub id: TrackId,
    /// name of the track, used as label
    pub name: String,
    /// instrument assigned to this track
    pub instrument: Instrument,
    /// clips in this track
    pub clip_collection: ClipCollection,
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}, instrument: {}", self.name, self.instrument))?;
        Ok(())
    }
}

impl Track {
    /// create a new track object
    pub fn new(id: TrackId, name: &str) -> Track {
        Self {
            id,
            name: String::from(name),
            instrument: Instrument::new("port0", 0, 0),
            clip_collection: ClipCollection::new(),
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
    pub fn add_clip(&mut self, clip: Clip) {
        self.clip_collection.insert(clip);
    }

    /// removes a clip from the track by its key
    pub fn remove_clip(&mut self, key: &ClipKey) {
        self.clip_collection.remove(key);
    }
}

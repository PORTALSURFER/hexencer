use crate::{
    event::{Event, EventEntry},
    Tick,
};
use std::collections::BTreeMap;

use super::{midi_message::MidiMessage, Id};

type EventListType = BTreeMap<Tick, EventEntry>;

/// a list of events, keyed by their `Tick`
#[derive(Default, Debug)]
pub struct EventList(EventListType);

impl FromIterator<(Tick, EventEntry)> for EventList {
    fn from_iter<T: IntoIterator<Item = (Tick, EventEntry)>>(iter: T) -> Self {
        let mut map = BTreeMap::new();
        for (tick, trig) in iter {
            map.insert(tick, trig);
        }
        EventList(map)
    }
}

impl EventList {
    /// creates an empty `EventList`
    pub fn new() -> EventList {
        EventList(BTreeMap::new())
    }

    /// adds a new event to the 'EventList'
    pub fn add_event(&mut self, tick: Tick, event_entry: EventEntry) {
        self.0.insert(tick, event_entry);
    }

    /// adds an 'EventBlock' to the 'EventList'
    pub fn add_event_block(&mut self, event_block: EventBlock) {
        let note_on_entry = EventEntry::new(event_block.id.clone(), event_block.start.1, true);
        let note_off_entry = EventEntry::new(event_block.id.clone(), event_block.end.1, true);

        self.0.insert(event_block.start.0, note_on_entry);
        self.0.insert(event_block.end.0, note_off_entry);
    }

    /// removes an ['Event'] from the 'EventList'
    pub fn get(&self, tick: &Tick) -> Option<&EventEntry> {
        self.0.get(tick)
    }

    /// gets a mutable iterator over the 'EventList', sorted by 'Tick'
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Tick, &mut EventEntry)> {
        self.0.iter_mut()
    }
}

/// represents a block defined by a starting and ending 'Event'
pub struct EventBlock {
    id: Id,
    start: (Tick, Event),
    end: (Tick, Event),
}

impl EventBlock {
    /// creates a new 'EventBlock' with a `NoteOn` and a `NoteOff` 'Event'
    pub fn new_midi(tick: Tick, length: u32, key: u8, channel: u8, velocity: u8) -> Self {
        let start_event = Event::Midi(MidiMessage::NoteOn { key, velocity });
        let end_event = Event::Midi(MidiMessage::NoteOff { key, velocity });
        Self {
            id: Id::new(),
            start: (tick, start_event),
            end: (tick.offset(length), end_event),
        }
    }
}

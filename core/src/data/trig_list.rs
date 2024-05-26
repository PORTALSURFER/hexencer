use crate::{
    event::{Event, EventEntry, UniqueId},
    Tick,
};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use super::midi_message::MidiMessage;

type EventListType = BTreeMap<Tick, EventEntry>;

#[derive(Default, Debug)]
pub struct EventList(EventListType);

// impl Deref for EventList {
//     type Target = EventListType;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for EventList {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

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
    pub fn new() -> EventList {
        EventList(BTreeMap::new())
    }

    pub fn add_event(&mut self, tick: Tick, event_entry: EventEntry) {
        self.0.insert(tick, event_entry);
    }

    pub fn add_event_block(&mut self, event_block: EventBlock) {
        let note_on_entry = EventEntry::new(event_block.id.clone(), event_block.start.1, true);
        let note_off_entry = EventEntry::new(event_block.id.clone(), event_block.end.1, true);

        self.0.insert(event_block.start.0, note_on_entry);
        self.0.insert(event_block.end.0, note_off_entry);
    }

    pub fn get(&self, tick: &Tick) -> Option<&EventEntry> {
        self.0.get(tick)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Tick, &mut EventEntry)> {
        self.0.iter_mut()
    }
}

pub struct EventBlock {
    id: UniqueId,
    start: (Tick, Event),
    end: (Tick, Event),
}

impl EventBlock {
    pub fn new_midi(tick: Tick, length: u32, key: u8, channel: u8, velocity: u8) -> Self {
        let start_event = Event::Midi(MidiMessage::NoteOn { key, velocity });
        let end_event = Event::Midi(MidiMessage::NoteOff { key, velocity });
        Self {
            id: UniqueId::new(),
            start: (tick, start_event),
            end: (tick.offset(length), end_event),
        }
    }
}

use crate::{
    event::{Event, EventType},
    Tick,
};
use std::collections::BTreeMap;

use super::{midi_message::MidiMessage, DataId};

type EventListType = BTreeMap<Tick, Vec<Event>>;

/// a list of events, keyed by their `Tick`
#[derive(Default, Debug)]
pub struct EventList(EventListType);

impl FromIterator<(Tick, Vec<Event>)> for EventList {
    fn from_iter<T: IntoIterator<Item = (Tick, Vec<Event>)>>(iter: T) -> Self {
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
    pub fn add_event(&mut self, tick: Tick, event_entry: Event) {
        self.0
            .get_mut(&tick)
            .map(move |events| events.push(event_entry))
            .unwrap_or_else(|| {
                self.0.insert(tick, vec![event_entry]);
            });
    }

    /// adds an 'EventBlock' to the 'EventList'
    pub fn add_event_block(&mut self, event_block: EventBlock) {
        let note_on_entry = Event::new(event_block.id.clone(), event_block.start.1, true);
        let note_off_entry = Event::new(event_block.id.clone(), event_block.end.1, true);

        self.add_event(event_block.start.0, note_on_entry);
        self.add_event(event_block.end.0, note_off_entry);
    }

    /// removes an ['Event'] from the 'EventList'
    pub fn get(&self, tick: &Tick) -> Option<&Vec<Event>> {
        self.0.get(tick)
    }

    /// gets a mutable iterator over the 'EventList', sorted by 'Tick'
    pub fn iter(&self) -> impl Iterator<Item = (&Tick, &Vec<Event>)> {
        self.0.iter()
    }

    /// gets a mutable iterator over the 'EventList', sorted by 'Tick'
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Tick, &mut Vec<Event>)> {
        self.0.iter_mut()
    }
}

/// represents a block defined by a starting and ending 'Event'
pub struct EventBlock {
    id: DataId,
    start: (Tick, EventType),
    end: (Tick, EventType),
}

impl EventBlock {
    /// creates a new 'EventBlock' with a `NoteOn` and a `NoteOff` 'Event'
    pub fn new_midi(tick: Tick, length: u32, key: u8, velocity: u8) -> Self {
        let start_event = EventType::Midi(MidiMessage::NoteOn { key, velocity });
        let end_event = EventType::Midi(MidiMessage::NoteOff { key, velocity });
        Self {
            id: DataId::new(),
            start: (tick, start_event),
            end: (tick.offset(length), end_event),
        }
    }
}

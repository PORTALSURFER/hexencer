use crate::{
    event::{Event, EventType},
    Tick,
};
use std::collections::BTreeMap;

use super::{midi_message::MidiMessage, DataId};

/// type used by the eventlist, stores events based on tick
type EventListType = BTreeMap<Tick, Vec<EventSegment>>;

/// a list of events, keyed by their `Tick`
#[derive(Default, Debug)]
pub struct EventList(EventListType);

impl FromIterator<(Tick, Vec<EventSegment>)> for EventList {
    fn from_iter<T: IntoIterator<Item = (Tick, Vec<EventSegment>)>>(iter: T) -> Self {
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
    pub fn add_event(&mut self, tick: Tick, event_entry: EventSegment) {
        self.0
            .get_mut(&tick)
            .map(move |events| events.push(event_entry))
            .unwrap_or_else(|| {
                self.0.insert(tick, vec![event_entry]);
            });
    }

    #[deprecated(note = "use `add_event` instead")]
    /// adds an 'EventBlock' to the 'EventList'
    pub fn add_event_segment(&mut self, event_block: EventSegment) {
        // let note_on_entry = Event::new(event_block.id, event_block.start.1, true);
        // let note_off_entry = Event::new(event_block.id, event_block.end.1, true);

        // self.add_event(event_block.start.0, note_on_entry);
        // self.add_event(event_block.end.0, note_off_entry);
    }

    /// removes an ['Event'] from the 'EventList'
    pub fn get(&self, tick: &Tick) -> Option<&Vec<EventSegment>> {
        self.0.get(tick)
    }

    /// gets a mutable iterator over the 'EventList', sorted by 'Tick'
    pub fn iter(&self) -> impl Iterator<Item = (&Tick, &Vec<EventSegment>)> {
        self.0.iter()
    }

    /// gets a mutable iterator over the 'EventList', sorted by 'Tick'
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Tick, &mut Vec<EventSegment>)> {
        self.0.iter_mut()
    }
}

#[derive(Debug, Clone, Copy)]
/// represents a block defined by a starting and ending 'EventSegment'
pub struct EventSegment {
    /// id used by the data layer
    pub id: DataId,
    /// start event of this segment, note on
    pub start: Tick,
    /// end event of this segment, note off
    pub end: Tick,
    /// the event type of this segment
    pub event_type: EventType,
    /// true if the event is active
    pub is_active: bool,
}

impl EventSegment {
    /// creates a new 'EventBlock' with a `NoteOn` and a `NoteOff` 'Event'
    pub fn new2(start_tick: Tick, end_tick: Tick, key: u8, velocity: u8, is_active: bool) -> Self {
        let event = EventType::Midi(MidiMessage::NoteOn { key, velocity });
        Self {
            id: DataId::new(),
            start: start_tick,
            end: end_tick,
            event_type: event,
            is_active,
        }
    }

    /// create a new event segment
    pub(crate) fn new(
        id: DataId,
        velocity: u8,
        start: Tick,
        end: Tick,
        event_type: EventType,
        is_active: bool,
    ) -> EventSegment {
        Self {
            id,
            start,
            end,
            event_type,
            is_active,
        }
    }

    /// gets the key of this event
    /// TODO should be moved from here to the event type implementation instead
    pub fn get_key(&self) -> u8 {
        match self.event_type {
            EventType::Midi(MidiMessage::NoteOn { key, .. }) => key,
            _ => 0,
        }
    }

    /// get the end tick
    pub fn get_end(&self) -> Tick {
        self.end
    }
}

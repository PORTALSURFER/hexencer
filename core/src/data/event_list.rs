use super::midi_event::MidiEvent;
use crate::Tick;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct EventList(BTreeMap<Tick, MidiEvent>);

impl FromIterator<EventList> for EventList {
    fn from_iter<T: IntoIterator<Item = EventList>>(iter: T) -> Self {
        let mut events = EventList::new();
        for i in iter {
            events.0.extend(i.0);
        }
        events
    }
}

impl EventList {
    pub fn insert(&mut self, tick: Tick, event: MidiEvent) {
        self.0.insert(tick, event);
    }

    pub fn new() -> EventList {
        EventList(BTreeMap::new())
    }

    pub fn get(&self, tick: &Tick) -> Option<&MidiEvent> {
        match self.0.get(tick) {
            Some(event) => Some(event),
            None => None,
        }
    }
}

use crate::{trig::Trig, Tick};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

#[derive(Default, Debug)]
pub struct TrigList(BTreeMap<Tick, Trig>);

impl Deref for TrigList {
    type Target = BTreeMap<Tick, Trig>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TrigList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<(Tick, Trig)> for TrigList {
    fn from_iter<T: IntoIterator<Item = (Tick, Trig)>>(iter: T) -> Self {
        let mut map = BTreeMap::new();
        for (tick, trig) in iter {
            map.insert(tick, trig);
        }
        TrigList(map)
    }
}

impl TrigList {
    pub fn new() -> TrigList {
        TrigList(BTreeMap::new())
    }
}

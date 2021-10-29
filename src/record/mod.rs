mod event;
mod global_property;
mod property;
mod update;

use std::fmt::Display;

pub use event::Event;
pub use global_property::GlobalProperty;
pub use property::{Color, Coords, Property, Tag};
pub use update::Update;

#[derive(Debug)]
pub enum Record {
    GlobalProperty(GlobalProperty),
    Event(Event),
    Remove(u64),
    Frame(f64),
    Update(Update),
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Record::GlobalProperty(r) => r.fmt(f),
            Record::Event(r) => r.fmt(f),
            Record::Remove(id) => write!(f, "-{}", id),
            Record::Frame(n) => write!(f, "#{:.4}", n),
            Record::Update(r) => r.fmt(f),
        }
    }
}

impl From<GlobalProperty> for Record {
    fn from(p: GlobalProperty) -> Self {
        Self::GlobalProperty(p)
    }
}

impl From<Event> for Record {
    fn from(e: Event) -> Self {
        Self::Event(e)
    }
}

impl From<Update> for Record {
    fn from(u: Update) -> Self {
        Self::Update(u)
    }
}
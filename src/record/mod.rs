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
            Record::Frame(n) => write!(f, "#{}", max_precision(*n, 2)),
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

fn max_precision(v: f64, max_precision: u32) -> f64 {
    let p = f64::from(10i32.pow(max_precision));
    (v * p).round() / p
}

#[cfg(test)]
mod test {
    use super::max_precision;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_max_precision() {
        assert_eq!(max_precision(12.3456789, 0), 12.0);
        assert_eq!(max_precision(12.3456789, 1), 12.3);
        assert_eq!(max_precision(12.3456789, 2), 12.35);
        assert_eq!(max_precision(12.3456789, 3), 12.346);
        assert_eq!(max_precision(12.3, 6), 12.3);
    }
}

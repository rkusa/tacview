mod event;
mod global_property;
mod property;
mod update;

pub use event::Event;
pub use global_property::GlobalProperty;
pub use property::Property;
pub use update::Update;

#[derive(Debug)]
pub enum Record {
    GlobalProperty(GlobalProperty),
    Event(Event),
    Remove(u64),
    Frame(f64),
    Update(Update),
}

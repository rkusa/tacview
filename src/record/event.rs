use std::fmt::Display;
use std::str::FromStr;

use crate::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub kind: EventKind,
    pub params: Vec<String>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    /// Generic event.
    Message,

    /// Bookmarks are highlighted in the time line and in the event log. They are easy to spot and
    /// handy to highlight parts of the flight, like a bombing run, or when the trainee was in her
    /// final approach for landing.
    Bookmark,

    /// Debug events are highlighted and easy to spot in the timeline and event log. Because they
    /// must be used for development purposes, they are displayed only when launching Tacview with
    /// the command line argument /Debug:on
    Debug,

    /// This event is useful to specify when an aircraft (or any object) is cleanly removed from the
    /// battlefield (not destroyed). This prevents Tacview from generating a Destroyed event by
    /// error.
    LeftArea,

    /// When an object has been officially destroyed.
    Destroyed,

    /// Because Tacview may not always properly auto-detect take-off events, it can be useful to
    /// manually inject this event in the flight recording.
    TakenOff,

    /// Because Tacview may not always properly auto-detect landing events, it can be useful to
    /// manually inject this event in the flight recording.
    Landed,

    /// Mainly used for real-life training debriefing to specify when a weapon (typically a
    /// missile) reaches or misses its target. Tacview will report in the shot log as well as in the
    /// 3D view the result of the shot. Most parameters are optional. SourceId designates the object
    /// which has fired the weapon, while TargetId designates the target. Even if the displayed
    /// result may be in nautical miles, bullseye coordinates must be specified in meters. The
    /// target must be explicitly (manually) destroyed or disabled using the appropriate properties
    /// independently from this event.
    Timeout,

    /// Unknown event. This only exists for forward compatibility and using it is not recommended
    /// as the event you are using could be move to the known event in a future release.
    Unknown(String),
}

impl FromStr for Event {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|');
        let kind = parts.next().ok_or(ParseError::InvalidEvent)?;
        let kind = match kind {
            "Message" => EventKind::Message,
            "Bookmark" => EventKind::Bookmark,
            "Debug" => EventKind::Debug,
            "LeftArea" => EventKind::LeftArea,
            "Destroyed" => EventKind::Destroyed,
            "TakenOff" => EventKind::TakenOff,
            "Landed" => EventKind::Landed,
            "Timeout" => EventKind::Timeout,
            name => EventKind::Unknown(name.to_string()),
        };

        let mut params = parts.map(String::from).collect::<Vec<_>>();
        let text = if params.is_empty() {
            None
        } else {
            Some(params.remove(params.len() - 1)).filter(|s| !s.is_empty())
        };

        Ok(Event { kind, params, text })
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0,Event={}", self.kind.as_str())?;
        for param in &self.params {
            write!(f, "|{param}")?;
        }
        write!(f, "|{}", self.text.as_deref().unwrap_or_default())?;
        Ok(())
    }
}

impl EventKind {
    fn as_str(&self) -> &str {
        use EventKind::*;
        match self {
            Message => "Message",
            Bookmark => "Bookmark",
            Debug => "Debug",
            LeftArea => "LeftArea",
            Destroyed => "Destroyed",
            TakenOff => "TakenOff",
            Landed => "Landed",
            Timeout => "Timeout",
            Unknown(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_event_text() {
        assert_eq!(
            Event {
                kind: EventKind::Landed,
                params: vec!["1".to_string(), "2".to_string()],
                text: None,
            }
            .to_string(),
            "0,Event=Landed|1|2|"
        )
    }
}

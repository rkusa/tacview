use std::str::FromStr;

use crate::ParseError;

#[derive(Debug)]
pub enum GlobalProperty {
    /// Source simulator, control station or file format.
    DataSource(String),

    /// Software or hardware used to record the data.
    DataRecorder(String),

    /// Base time (UTC) for the current mission. This time is combined with each frame offset (in
    /// seconds) to get the final absolute UTC time for each data sample.
    ReferenceTime(String),

    /// Recording (file) creation (UTC) time.
    RecordingTime(String),

    /// Author or operator who has created this recording.
    Author(String),

    /// Mission/flight title or designation.
    Title(String),

    /// Category of the flight/mission.
    Category(String),

    /// Free text containing the briefing of the flight/mission.
    Briefing(String),

    /// Free text containing the debriefing.
    Debriefing(String),

    /// Free comments about the flight. Do not forget to escape any end-of-line character you want
    /// to inject into the comments.
    Comments(String),

    /// Used to reduce the file size by centring coordinates around a median point. This will be
    /// added to each object Longitude to get the final coordinates.
    ReferenceLongitude(f64),

    /// Used to reduce the file size by centring coordinates around a median point. This will be
    /// added to each object Latitude to get the final coordinates.
    ReferenceLatitude(f64),

    /// Unknown property. This only exists for forward compatibility and using it is not recommended
    /// as the property you are using could be move to the known properties in a future release.
    Unknown(String, String),
}

impl FromStr for GlobalProperty {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s.split_once('=').ok_or(ParseError::MissingDelimiter('='))?;

        Ok(match name {
            "DataSource" => Self::DataSource(value.to_string()),
            "DataRecorder" => Self::DataRecorder(value.to_string()),
            "ReferenceTime" => Self::ReferenceTime(value.to_string()),
            "RecordingTime" => Self::RecordingTime(value.to_string()),
            "Author" => Self::Author(value.to_string()),
            "Title" => Self::Title(value.to_string()),
            "Category" => Self::Category(value.to_string()),
            "Briefing" => Self::Briefing(value.to_string()),
            "Debriefing" => Self::Debriefing(value.to_string()),
            "Comments" => Self::Comments(value.to_string()),
            "ReferenceLongitude" => Self::ReferenceLongitude(value.parse()?),
            "ReferenceLatitude" => Self::ReferenceLongitude(value.parse()?),
            name => Self::Unknown(name.to_string(), value.to_string()),
        })
    }
}
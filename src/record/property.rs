#![allow(clippy::upper_case_acronyms)]

use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use crate::ParseError;

#[derive(Debug)]
pub enum Property {
    /// Object Coordinates.
    T(Coords),

    /// The object name should use the most common notation for each object. It is strongly
    /// recommended to use ICAO or NATO names like: C172 or F/A-18C. This will help Tacview to
    /// associate each object with the corresponding entry in its database. Type and Name are the
    /// only properties which *CANNOT* be predefined in Tacview database.
    Name(String),

    /// Object types are built using tags. This makes object management much more powerful and
    /// transparent than with the previous exclusive types. Type and Name are the only properties
    /// which *CANNOT* be predefined in Tacview database.
    Type(HashSet<Tag>),

    /// Parent object id. Useful to associate for example a missile (child object) and
    /// its launcher aircraft (parent object).
    Parent(u64),

    /// ID of the following object. Typically used to link waypoints together.
    Next(u64),

    /// The call sign will be displayed in priority over the object name and sometimes pilot name,
    /// especially in the 3D view and selection boxes. This is handy for mission debriefings where
    /// call signs are more informative than aircraft names.
    CallSign(String),

    /// Aircraft registration (aka tail number)
    Registration(String),

    /// Current transponder code. Any code is possible, there is no limitation like with the old 4
    /// digit transponders.
    Squawk(String),

    /// Mode S equipped aircraft uniquely assigned ICAO 24-bit address.
    ICAO24(String),

    /// Aircraft pilot in command name.
    Pilot(String),

    /// Group the object belongs to. Used to group objects together. For example, a formation of
    /// F-16 flying a CAP together.
    Group(String),

    /// ISO 3166-1 alpha-2 country code.
    Country(String),

    /// Coalition.
    Coalition(String),

    /// Color of the object.
    Color(Color),

    /// Filename of the 3D model which will be used to represent the object in the 3D view. 3D
    /// models must be in Wavefront .obj file format and stored in either
    /// %ProgramData%\Tacview\Data\Meshes\ or %APPDATA%\Tacview\Data\Meshes\.
    Shape(String),

    /// Debug text visible in the 3D view when Tacview is launched with the /Debug:on command line
    /// argument.
    Debug(String),

    /// Free real-time text displayable in the 3D view and telemetry windows (to provide
    /// miscellaneous info to the end-user)
    Label(String),

    /// Target currently focused by the object (typically used to designate laser beam target
    /// object, can also be used to show what the pilot is currently focused on)
    FocusedTarget(u64),

    /// Primary target id (could be locked using any device, like radar, IR, NVG, ...)
    LockedTarget(u64),

    /// The higher the ratio, the more important is the object is (e.g. locally simulated aircraft
    /// could be 1.0 importance factor).
    /// Unit: ratio
    Importance(f64),

    /// Plane position in its Group (the lowest is the leader).
    Slot(u64),

    /// Specifies that an object is disabled (typically out-of-combat) without being destroyed yet.
    /// This is particularly useful for combat training and shotlogs.
    Disabled(bool),

    /// This flag is useful to hide specific objects from the 3D view. Can be used for a fog-of-war
    /// effect, or to prevent virtual objects from being displayed.
    Visible(bool),

    /// Use this attribute to record the current health status of an object. The ratio is equal to
    /// 1.0 when the object is brand new, and 0.0 whenever the object is out of
    /// combat/dead/destroyed. This attribute as currently no effect on the events, you still need
    /// to remove the object manually whenever it is destroyed.
    /// Unit: ratio
    Health(f64),

    /// Object length. Especially useful when displaying buildings.
    /// Unit: m
    Length(f64),

    /// Object width. Especially useful when displaying buildings.
    /// Unit: m
    Width(f64),

    /// Object height. Especially useful when displaying buildings.
    /// Unit: m
    Height(f64),

    /// Object bounding sphere radius. Object bounding sphere radius. Can be used to define custom
    /// explosion, smoke/grenade radius. Can be animated.
    /// Unit: m
    Radius(f64),

    /// Indicated airspeed.
    /// Unit: m/s
    IAS(f64),

    /// Calibrated airspeed.
    /// Unit: m/s
    CAS(f64),

    /// True airspeed.
    /// Unit: m/s
    TAS(f64),

    /// Mach number.
    /// Unit: ratio
    Mach(f64),

    /// Angle of attack.
    /// Unit: deg
    AOA(f64),

    /// Sideslip angle, also called angle of sideslip.
    /// Unit: deg
    AOS(f64),

    /// Object altitude above ground level.
    /// Unit: m
    AGL(f64),

    /// Aircraft heading. When there is no roll and pitch data available, this property can be used
    /// to specify the yaw while keeping full rotation emulation in the 3D view.
    /// Unit: deg
    HDG(f64),

    /// Aircraft magnetic heading. Heading relative to local magnetic north.
    /// Unit: deg
    HDM(f64),

    /// Main/engine #1 throttle handle position (could be >1 for Afterburner and <0 for reverse).
    /// Unit: ratio
    Throttle(f64),

    /// Main/engine #1 afterburner status.
    /// Unit: ratio
    Afterburner(f64),

    /// Air brakes status.
    /// Unit: ratio
    AirBrakes(f64),

    /// Flaps position.
    /// Unit: ratio
    Flaps(f64),

    /// Landing gear status.
    /// Unit: ratio
    LandingGear(f64),

    /// Landing gear handle position.
    /// Unit: ratio
    LandingGearHandle(f64),

    /// Arresting hook status.
    /// Unit: ratio
    Tailhook(f64),

    /// Parachute status (not to be mistaken for DragChute).
    /// Unit: ratio
    Parachute(f64),

    /// Drogue/Drag Parachute status.
    /// Unit: ratio
    DragChute(f64),

    /// Fuel quantity currently available in each tanks (up to 10 tanks supported).
    /// Unit: kg
    FuelWeight(u8, f64),

    /// Fuel quantity currently available in each tanks (up to 10 tanks supported).
    /// Unit: l
    FuelVolume(u8, f64),

    /// Fuel flow for each engine (up to 8 engines supported).
    /// Unit: kg/hour
    FuelFlowWeight(u8, f64),

    /// Fuel flow for each engine (up to 8 engines supported).
    /// Unit: l/hour
    FuelFlowVolume(u8, f64),

    /// Radar mode (0 = off)
    RadarMode(f64),

    /// Radar azimuth (heading) relative to aircraft orientation.
    /// Unit: deg
    RadarAzimuth(f64),

    /// Radar elevation relative to aircraft orientation.
    /// Unit: deg
    RadarElevation(f64),

    /// Radar roll angle relative to aircraft orientation.
    /// Unit: deg
    RadarRoll(f64),

    /// Radar scan range.
    /// Unit: m
    RadarRange(f64),

    /// Radar beamwidth in azimuth.
    /// Unit: deg
    RadarHorizontalBeamwidth(f64),

    /// Radar beamwidth in elevation.
    /// Unit: deg
    RadarVerticalBeamwidth(f64),

    /// Primary target lock mode (0 = no lock/no target).
    LockedTargetMode(f64),

    /// Primary target azimuth (heading) relative to aircraft orientation.
    /// Unit: deg
    LockedTargetAzimuth(f64),

    /// Primary target elevation relative to aircraft orientation.
    /// Unit: deg
    LockedTargetElevation(f64),

    /// Primary target distance to aircraft.
    /// Unit: m
    LockedTargetRange(f64),

    /// Enable/disable engagement range (such as when a SAM site turns off its radar) (0 = off).
    EngagementMode(f64),

    /// Enable/disable engagement range (such as when a SAM site turns off its radar) (0 = off).
    EngagementMode2(f64),

    /// Engagement range for anti-aircraft units. This is the radius of the sphere which will be
    /// displayed in the 3D view. Typically used for SAM and AAA units, but this can be also
    /// relevant to warships.
    /// Unit: m
    EngagementRange(f64),

    /// Engagement range for anti-aircraft units. This is the radius of the sphere which will be
    /// displayed in the 3D view. Typically used for SAM and AAA units, but this can be also
    /// relevant to warships.
    /// Unit: m
    EngagementRange2(f64),

    /// Engagement range for anti-aircraft units. This is the radius of the sphere which will be
    /// displayed in the 3D view. Typically used for SAM and AAA units, but this can be also
    /// relevant to warships.
    /// Unit: m
    VerticalEngagementRange(f64),

    /// Engagement range for anti-aircraft units. This is the radius of the sphere which will be
    /// displayed in the 3D view. Typically used for SAM and AAA units, but this can be also
    /// relevant to warships.
    /// Unit: m
    VerticalEngagementRange2(f64),

    /// Raw player HOTAS/Yoke position in real-life (flight sim input device).
    /// Unit: ratio
    RollControlInput(f64),

    /// Raw player HOTAS/Yoke position in real-life (flight sim input device).
    /// Unit: ratio
    PitchControlInput(f64),

    /// Raw player HOTAS/Yoke position in real-life (flight sim input device).
    /// Unit: ratio
    YawControlInput(f64),

    /// HOTAS/Yoke position in simulated (with response curves) or real-life cockpit.
    /// Unit: ratio
    RollControlPosition(f64),

    /// HOTAS/Yoke position in simulated (with response curves) or real-life cockpit.
    /// Unit: ratio
    PitchControlPosition(f64),

    /// HOTAS/Yoke position in simulated (with response curves) or real-life cockpit.
    /// Unit: ratio
    YawControlPosition(f64),

    /// Trim position for each axis.
    /// Unit: ratio
    RollTrimTab(f64),

    /// Trim position for each axis.
    /// Unit: ratio
    PitchTrimTab(f64),

    /// Trim position for each axis.
    /// Unit: ratio
    YawTrimTab(f64),

    /// Control surfaces position on the aircraft.
    /// Unit: ratio
    AileronLeft(f64),

    /// Control surfaces position on the aircraft.
    /// Unit: ratio
    AileronRight(f64),

    /// Control surfaces position on the aircraft.
    /// Unit: ratio
    Elevator(f64),

    /// Control surfaces position on the aircraft.
    /// Unit: ratio
    Rudder(f64),

    /// Pilot head orientation in the cockpit relative to the aircraft orientation
    /// Unit: ratio
    PilotHeadRoll(f64),

    /// Pilot head orientation in the cockpit relative to the aircraft orientation
    /// Unit: ratio
    PilotHeadPitch(f64),

    /// Pilot head orientation in the cockpit relative to the aircraft orientation
    /// Unit: ratio
    PilotHeadYaw(f64),

    /// Gravitational force equivalent of the acceleration in each axis relative to the aircraft
    /// orientation
    /// Unit: g
    VerticalGForce(f64),

    /// Gravitational force equivalent of the acceleration in each axis relative to the aircraft
    /// orientation
    /// Unit: g
    LongitudinalGForce(f64),

    /// Gravitational force equivalent of the acceleration in each axis relative to the aircraft
    /// orientation
    /// Unit: g
    LateralGForce(f64),

    /// Ratio between 0 and 1 describing the current Environmental Noise Level measured by the
    /// flight recorder. Typically used by gliders to detect engine use. This is the equivalent of
    /// the ENL field which can be found in IGC files.
    /// Unit: ratio
    ENL(f64),

    /// Unknown property. This only exists for forward compatibility and using it is not recommended
    /// as the property you are using could be move to the known properties in a future release.
    Unknown(String, String),
}

#[derive(Debug, Default)]
pub struct Coords {
    /// Unit: deg
    pub longitude: Option<f64>,

    /// Unit: deg
    pub latitude: Option<f64>,

    /// Unit: m
    pub altitude: Option<f64>,

    /// Native x coordinate from a flat world.
    pub u: Option<f64>,

    /// Native y coordinate from a flat world.
    pub v: Option<f64>,

    /// Positive when rolling the aircraft to the right.
    pub roll: Option<f64>,

    /// Positive when taking off.
    pub pitch: Option<f64>,

    /// Clockwise relative to true north.
    pub yaw: Option<f64>,

    /// Yaw relative to true north of the flat world.
    pub heading: Option<f64>,
}

impl Coords {
    pub fn update(&mut self, other: &Coords, reference_latitude: f64, reference_longitude: f64) {
        if let Some(longitude) = other.longitude {
            self.longitude = Some(longitude + reference_longitude);
        }
        if let Some(latitude) = other.latitude {
            self.latitude = Some(latitude + reference_latitude);
        }
        if let Some(altitude) = other.altitude {
            self.altitude = Some(altitude);
        }
        if let Some(u) = other.u {
            self.u = Some(u);
        }
        if let Some(v) = other.v {
            self.v = Some(v);
        }
        if let Some(roll) = other.roll {
            self.roll = Some(roll);
        }
        if let Some(pitch) = other.pitch {
            self.pitch = Some(pitch);
        }
        if let Some(yaw) = other.yaw {
            self.yaw = Some(yaw);
        }
        if let Some(heading) = other.heading {
            self.heading = Some(heading);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Orange,
    Green,
    Blue,
    Violet,
    Grey,
    Unknown(String),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Tag {
    // Class
    Air,
    Ground,
    Sea,
    Weapon,
    Sensor,
    Navaid,
    Misc,
    // Attributes
    Static,
    Heavy,
    Medium,
    Light,
    Minor,
    // Basic Types
    FixedWing,
    Rotorcraft,
    Armor,
    AntiAircraft,
    Vehicle,
    Watercraft,
    Human,
    Biologic,
    Missile,
    Rocket,
    Bomb,
    Torpedo,
    Projectile,
    Beam,
    Decoy,
    Building,
    Bullseye,
    Waypoint,
    // Specific Types
    Tank,
    Warship,
    AircraftCarrier,
    Submarine,
    Infantry,
    Parachutist,
    Shell,
    Bullet,
    Flare,
    Chaff,
    SmokeGrenade,
    Aerodrome,
    Container,
    Shrapnel,
    Unknown(String),
}

impl FromStr for Property {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s.split_once('=').ok_or(ParseError::MissingDelimiter('='))?;

        Ok(match name {
            "T" => Property::T(Coords::from_str(value)?),
            "Name" => Property::Name(value.to_string()),
            "Type" => Property::Type(value.split('+').map(Tag::from).collect()),
            "Parent" => Property::Parent(u64::from_str_radix(value, 16)?),
            "Next" => Property::Next(u64::from_str_radix(value, 16)?),
            "CallSign" => Property::CallSign(value.to_string()),
            "Registration" => Property::Registration(value.to_string()),
            "Squawk" => Property::Squawk(value.to_string()),
            "ICAO24" => Property::ICAO24(value.to_string()),
            "Pilot" => Property::Pilot(value.to_string()),
            "Group" => Property::Group(value.to_string()),
            "Country" => Property::Country(value.to_string()),
            "Coalition" => Property::Coalition(value.to_string()),
            "Color" => Property::Color(Color::from(value)),
            "Shape" => Property::Shape(value.to_string()),
            "Debug" => Property::Debug(value.to_string()),
            "Label" => Property::Label(value.to_string()),
            "FocusedTarget" => Property::FocusedTarget(u64::from_str_radix(value, 16)?),
            "LockedTarget" => Property::LockedTarget(u64::from_str_radix(value, 16)?),
            "Importance" => Property::Importance(FromStr::from_str(value)?),
            "Slot" => Property::Slot(FromStr::from_str(value)?),
            "Disabled" => Property::Disabled(i64::from_str(value)? != 0),
            "Visible" => Property::Visible(i64::from_str(value)? != 0),
            "Health" => Property::Health(FromStr::from_str(value)?),
            "Length" => Property::Length(FromStr::from_str(value)?),
            "Width" => Property::Width(FromStr::from_str(value)?),
            "Height" => Property::Height(FromStr::from_str(value)?),
            "Radius" => Property::Radius(FromStr::from_str(value)?),
            "IAS" => Property::IAS(FromStr::from_str(value)?),
            "CAS" => Property::CAS(FromStr::from_str(value)?),
            "TAS" => Property::TAS(FromStr::from_str(value)?),
            "Mach" => Property::Mach(FromStr::from_str(value)?),
            "AOA" => Property::AOA(FromStr::from_str(value)?),
            "AOS" => Property::AOS(FromStr::from_str(value)?),
            "AGL" => Property::AGL(FromStr::from_str(value)?),
            "HDG" => Property::HDG(FromStr::from_str(value)?),
            "HDM" => Property::HDM(FromStr::from_str(value)?),
            "Throttle" => Property::Throttle(FromStr::from_str(value)?),
            "Afterburner" => Property::Afterburner(FromStr::from_str(value)?),
            "AirBrakes" => Property::AirBrakes(FromStr::from_str(value)?),
            "Flaps" => Property::Flaps(FromStr::from_str(value)?),
            "LandingGear" => Property::LandingGear(FromStr::from_str(value)?),
            "LandingGearHandle" => Property::LandingGearHandle(FromStr::from_str(value)?),
            "Tailhook" => Property::Tailhook(FromStr::from_str(value)?),
            "Parachute" => Property::Parachute(FromStr::from_str(value)?),
            "DragChute" => Property::DragChute(FromStr::from_str(value)?),
            "FuelWeight" => Property::FuelWeight(0, FromStr::from_str(value)?),
            "FuelWeight2" => Property::FuelWeight(1, FromStr::from_str(value)?),
            "FuelWeight3" => Property::FuelWeight(2, FromStr::from_str(value)?),
            "FuelWeight4" => Property::FuelWeight(3, FromStr::from_str(value)?),
            "FuelWeight5" => Property::FuelWeight(4, FromStr::from_str(value)?),
            "FuelWeight6" => Property::FuelWeight(5, FromStr::from_str(value)?),
            "FuelWeight7" => Property::FuelWeight(6, FromStr::from_str(value)?),
            "FuelWeight8" => Property::FuelWeight(7, FromStr::from_str(value)?),
            "FuelWeight9" => Property::FuelWeight(8, FromStr::from_str(value)?),
            "FuelVolume" => Property::FuelVolume(0, FromStr::from_str(value)?),
            "FuelVolume1" => Property::FuelVolume(1, FromStr::from_str(value)?),
            "FuelVolume2" => Property::FuelVolume(2, FromStr::from_str(value)?),
            "FuelVolume3" => Property::FuelVolume(3, FromStr::from_str(value)?),
            "FuelVolume4" => Property::FuelVolume(4, FromStr::from_str(value)?),
            "FuelVolume5" => Property::FuelVolume(5, FromStr::from_str(value)?),
            "FuelVolume6" => Property::FuelVolume(6, FromStr::from_str(value)?),
            "FuelVolume7" => Property::FuelVolume(7, FromStr::from_str(value)?),
            "FuelVolume8" => Property::FuelVolume(8, FromStr::from_str(value)?),
            "FuelVolume9" => Property::FuelVolume(9, FromStr::from_str(value)?),
            "FuelFlowWeight" => Property::FuelFlowWeight(0, FromStr::from_str(value)?),
            "FuelFlowWeight2" => Property::FuelFlowWeight(1, FromStr::from_str(value)?),
            "FuelFlowWeight3" => Property::FuelFlowWeight(2, FromStr::from_str(value)?),
            "FuelFlowWeight4" => Property::FuelFlowWeight(3, FromStr::from_str(value)?),
            "FuelFlowWeight5" => Property::FuelFlowWeight(4, FromStr::from_str(value)?),
            "FuelFlowWeight6" => Property::FuelFlowWeight(5, FromStr::from_str(value)?),
            "FuelFlowWeight7" => Property::FuelFlowWeight(6, FromStr::from_str(value)?),
            "FuelFlowWeight8" => Property::FuelFlowWeight(7, FromStr::from_str(value)?),
            "FuelFlowVolume" => Property::FuelFlowVolume(0, FromStr::from_str(value)?),
            "FuelFlowVolume2" => Property::FuelFlowVolume(1, FromStr::from_str(value)?),
            "FuelFlowVolume3" => Property::FuelFlowVolume(2, FromStr::from_str(value)?),
            "FuelFlowVolume4" => Property::FuelFlowVolume(3, FromStr::from_str(value)?),
            "FuelFlowVolume5" => Property::FuelFlowVolume(4, FromStr::from_str(value)?),
            "FuelFlowVolume6" => Property::FuelFlowVolume(5, FromStr::from_str(value)?),
            "FuelFlowVolume7" => Property::FuelFlowVolume(6, FromStr::from_str(value)?),
            "FuelFlowVolume8" => Property::FuelFlowVolume(7, FromStr::from_str(value)?),
            "RadarMode" => Property::RadarMode(FromStr::from_str(value)?),
            "RadarAzimuth" => Property::RadarAzimuth(FromStr::from_str(value)?),
            "RadarElevation" => Property::RadarElevation(FromStr::from_str(value)?),
            "RadarRoll" => Property::RadarRoll(FromStr::from_str(value)?),
            "RadarRange" => Property::RadarRange(FromStr::from_str(value)?),
            "RadarHorizontalBeamwidth" => {
                Property::RadarHorizontalBeamwidth(FromStr::from_str(value)?)
            }
            "RadarVerticalBeamwidth" => Property::RadarVerticalBeamwidth(FromStr::from_str(value)?),
            "LockedTargetMode" => Property::LockedTargetMode(FromStr::from_str(value)?),
            "LockedTargetAzimuth" => Property::LockedTargetAzimuth(FromStr::from_str(value)?),
            "LockedTargetElevation" => Property::LockedTargetElevation(FromStr::from_str(value)?),
            "LockedTargetRange" => Property::LockedTargetRange(FromStr::from_str(value)?),
            "EngagementMode" => Property::EngagementMode(FromStr::from_str(value)?),
            "EngagementMode2" => Property::EngagementMode2(FromStr::from_str(value)?),
            "EngagementRange" => Property::EngagementRange(FromStr::from_str(value)?),
            "EngagementRange2" => Property::EngagementRange2(FromStr::from_str(value)?),
            "VerticalEngagementRange" => {
                Property::VerticalEngagementRange(FromStr::from_str(value)?)
            }
            "VerticalEngagementRange2" => {
                Property::VerticalEngagementRange2(FromStr::from_str(value)?)
            }
            "RollControlInput" => Property::RollControlInput(FromStr::from_str(value)?),
            "PitchControlInput" => Property::PitchControlInput(FromStr::from_str(value)?),
            "YawControlInput" => Property::YawControlInput(FromStr::from_str(value)?),
            "RollControlPosition" => Property::RollControlPosition(FromStr::from_str(value)?),
            "PitchControlPosition" => Property::PitchControlPosition(FromStr::from_str(value)?),
            "YawControlPosition" => Property::YawControlPosition(FromStr::from_str(value)?),
            "RollTrimTab" => Property::RollTrimTab(FromStr::from_str(value)?),
            "PitchTrimTab" => Property::PitchTrimTab(FromStr::from_str(value)?),
            "YawTrimTab" => Property::YawTrimTab(FromStr::from_str(value)?),
            "AileronLeft" => Property::AileronLeft(FromStr::from_str(value)?),
            "AileronRight" => Property::AileronRight(FromStr::from_str(value)?),
            "Elevator" => Property::Elevator(FromStr::from_str(value)?),
            "Rudder" => Property::Rudder(FromStr::from_str(value)?),
            "PilotHeadRoll" => Property::PilotHeadRoll(FromStr::from_str(value)?),
            "PilotHeadPitch" => Property::PilotHeadPitch(FromStr::from_str(value)?),
            "PilotHeadYaw" => Property::PilotHeadYaw(FromStr::from_str(value)?),
            "VerticalGForce" => Property::VerticalGForce(FromStr::from_str(value)?),
            "LongitudinalGForce" => Property::LongitudinalGForce(FromStr::from_str(value)?),
            "LateralGForce" => Property::LateralGForce(FromStr::from_str(value)?),
            "ENL" => Property::ENL(FromStr::from_str(value)?),
            name => Self::Unknown(name.to_string(), value.to_string()),
        })
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Property::*;
        match self {
            T(v) => write!(f, "T={}", v),
            Name(v) => write!(f, "Name={}", v),
            Type(v) => write!(f, "Type={}", join(v.iter().map(|v| v.as_str()), "+")),
            Parent(v) => write!(f, "Parent={:x}", v),
            Next(v) => write!(f, "Next={:x}", v),
            CallSign(v) => write!(f, "CallSign={}", v),
            Registration(v) => write!(f, "Registration={}", v),
            Squawk(v) => write!(f, "Squawk={}", v),
            ICAO24(v) => write!(f, "ICAO24={}", v),
            Pilot(v) => write!(f, "Pilot={}", v),
            Group(v) => write!(f, "Group={}", v),
            Country(v) => write!(f, "Country={}", v),
            Coalition(v) => write!(f, "Coalition={}", v),
            Color(v) => write!(f, "Color={}", v.as_str()),
            Shape(v) => write!(f, "Shape={}", v),
            Debug(v) => write!(f, "Debug={}", v),
            Label(v) => write!(f, "Label={}", v),
            FocusedTarget(v) => write!(f, "FocusedTarget={:x}", v),
            LockedTarget(v) => write!(f, "LockedTarget={:x}", v),
            Importance(v) => write!(f, "Importance={}", v),
            Slot(v) => write!(f, "Slot={}", v),
            Disabled(v) => write!(f, "Disabled={}", *v as i32),
            Visible(v) => write!(f, "Visible={}", *v as i32),
            Health(v) => write!(f, "Health={}", v),
            Length(v) => write!(f, "Length={}", v),
            Width(v) => write!(f, "Width={}", v),
            Height(v) => write!(f, "Height={}", v),
            Radius(v) => write!(f, "Radius={}", v),
            IAS(v) => write!(f, "IAS={}", v),
            CAS(v) => write!(f, "CAS={}", v),
            TAS(v) => write!(f, "TAS={}", v),
            Mach(v) => write!(f, "Mach={}", v),
            AOA(v) => write!(f, "AOA={}", v),
            AOS(v) => write!(f, "AOS={}", v),
            AGL(v) => write!(f, "AGL={}", v),
            HDG(v) => write!(f, "HDG={}", v),
            HDM(v) => write!(f, "HDM={}", v),
            Throttle(v) => write!(f, "Throttle={}", v),
            Afterburner(v) => write!(f, "Afterburner={}", v),
            AirBrakes(v) => write!(f, "AirBrakes={}", v),
            Flaps(v) => write!(f, "Flaps={}", v),
            LandingGear(v) => write!(f, "LandingGear={}", v),
            LandingGearHandle(v) => write!(f, "LandingGearHandle={}", v),
            Tailhook(v) => write!(f, "Tailhook={}", v),
            Parachute(v) => write!(f, "Parachute={}", v),
            DragChute(v) => write!(f, "DragChute={}", v),
            FuelWeight(i, v) => write!(f, "FuelWeight{}={}", to_index(*i), v),
            FuelVolume(i, v) => write!(f, "FuelVolume{}={}", to_index(*i), v),
            FuelFlowWeight(i, v) => write!(f, "FuelFlowWeight{}={}", to_index(*i), v),
            FuelFlowVolume(i, v) => write!(f, "FuelFlowVolume{}={}", to_index(*i), v),
            RadarMode(v) => write!(f, "RadarMode={}", v),
            RadarAzimuth(v) => write!(f, "RadarAzimuth={}", v),
            RadarElevation(v) => write!(f, "RadarElevation={}", v),
            RadarRoll(v) => write!(f, "RadarRoll={}", v),
            RadarRange(v) => write!(f, "RadarRange={}", v),
            RadarHorizontalBeamwidth(v) => write!(f, "RadarHorizontalBeamwidth={}", v),
            RadarVerticalBeamwidth(v) => write!(f, "RadarVerticalBeamwidth={}", v),
            LockedTargetMode(v) => write!(f, "LockedTargetMode={}", v),
            LockedTargetAzimuth(v) => write!(f, "LockedTargetAzimuth={}", v),
            LockedTargetElevation(v) => write!(f, "LockedTargetElevation={}", v),
            LockedTargetRange(v) => write!(f, "LockedTargetRange={}", v),
            EngagementMode(v) => write!(f, "EngagementMode={}", v),
            EngagementMode2(v) => write!(f, "EngagementMode2={}", v),
            EngagementRange(v) => write!(f, "EngagementRange={}", v),
            EngagementRange2(v) => write!(f, "EngagementRange2={}", v),
            VerticalEngagementRange(v) => write!(f, "VerticalEngagementRange={}", v),
            VerticalEngagementRange2(v) => write!(f, "VerticalEngagementRange2={}", v),
            RollControlInput(v) => write!(f, "RollControlInput={}", v),
            PitchControlInput(v) => write!(f, "PitchControlInput={}", v),
            YawControlInput(v) => write!(f, "YawControlInput={}", v),
            RollControlPosition(v) => write!(f, "RollControlPosition={}", v),
            PitchControlPosition(v) => write!(f, "PitchControlPosition={}", v),
            YawControlPosition(v) => write!(f, "YawControlPosition={}", v),
            RollTrimTab(v) => write!(f, "RollTrimTab={}", v),
            PitchTrimTab(v) => write!(f, "PitchTrimTab={}", v),
            YawTrimTab(v) => write!(f, "YawTrimTab={}", v),
            AileronLeft(v) => write!(f, "AileronLeft={}", v),
            AileronRight(v) => write!(f, "AileronRight={}", v),
            Elevator(v) => write!(f, "Elevator={}", v),
            Rudder(v) => write!(f, "Rudder={}", v),
            PilotHeadRoll(v) => write!(f, "PilotHeadRoll={}", v),
            PilotHeadPitch(v) => write!(f, "PilotHeadPitch={}", v),
            PilotHeadYaw(v) => write!(f, "PilotHeadYaw={}", v),
            VerticalGForce(v) => write!(f, "VerticalGForce={}", v),
            LongitudinalGForce(v) => write!(f, "LongitudinalGForce={}", v),
            LateralGForce(v) => write!(f, "LateralGForce={}", v),
            ENL(v) => write!(f, "ENL={}", v),
            Unknown(k, v) => write!(f, "{}={}", k, v),
        }
    }
}

impl<'a> From<&'a str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "Red" => Self::Red,
            "Orange" => Self::Orange,
            "Green" => Self::Green,
            "Blue" => Self::Blue,
            "Violet" => Self::Violet,
            color => Self::Unknown(color.to_string()),
        }
    }
}

impl Color {
    fn as_str(&self) -> &str {
        use Color::*;
        match self {
            Red => "Red",
            Orange => "Orange",
            Green => "Green",
            Blue => "Blue",
            Violet => "Violet",
            Grey => "Grey",
            Unknown(color) => color,
        }
    }
}

impl<'a> From<&'a str> for Tag {
    fn from(s: &str) -> Self {
        match s {
            "Air" => Self::Air,
            "Ground" => Self::Ground,
            "Sea" => Self::Sea,
            "Weapon" => Self::Weapon,
            "Sensor" => Self::Sensor,
            "Navaid" => Self::Navaid,
            "Misc" => Self::Misc,
            "Static" => Self::Static,
            "Heavy" => Self::Heavy,
            "Medium" => Self::Medium,
            "Light" => Self::Light,
            "Minor" => Self::Minor,
            "FixedWing" => Self::FixedWing,
            "Rotorcraft" => Self::Rotorcraft,
            "Armor" => Self::Armor,
            "AntiAircraft" => Self::AntiAircraft,
            "Vehicle" => Self::Vehicle,
            "Watercraft" => Self::Watercraft,
            "Human" => Self::Human,
            "Biologic" => Self::Biologic,
            "Missile" => Self::Missile,
            "Rocket" => Self::Rocket,
            "Bomb" => Self::Bomb,
            "Torpedo" => Self::Torpedo,
            "Projectile" => Self::Projectile,
            "Beam" => Self::Beam,
            "Decoy" => Self::Decoy,
            "Building" => Self::Building,
            "Bullseye" => Self::Bullseye,
            "Waypoint" => Self::Waypoint,
            "Tank" => Self::Tank,
            "Warship" => Self::Warship,
            "AircraftCarrier" => Self::AircraftCarrier,
            "Submarine" => Self::Submarine,
            "Infantry" => Self::Infantry,
            "Parachutist" => Self::Parachutist,
            "Shell" => Self::Shell,
            "Bullet" => Self::Bullet,
            "Flare" => Self::Flare,
            "Chaff" => Self::Chaff,
            "SmokeGrenade" => Self::SmokeGrenade,
            "Aerodrome" => Self::Aerodrome,
            "Container" => Self::Container,
            "Shrapnel" => Self::Shrapnel,
            tag => Self::Unknown(tag.to_string()),
        }
    }
}

impl Tag {
    fn as_str(&self) -> &str {
        use Tag::*;
        match self {
            Air => "Air",
            Ground => "Ground",
            Sea => "Sea",
            Weapon => "Weapon",
            Sensor => "Sensor",
            Navaid => "Navaid",
            Misc => "Misc",
            Static => "Static",
            Heavy => "Heavy",
            Medium => "Medium",
            Light => "Light",
            Minor => "Minor",
            FixedWing => "FixedWing",
            Rotorcraft => "Rotorcraft",
            Armor => "Armor",
            AntiAircraft => "AntiAircraft",
            Vehicle => "Vehicle",
            Watercraft => "Watercraft",
            Human => "Human",
            Biologic => "Biologic",
            Missile => "Missile",
            Rocket => "Rocket",
            Bomb => "Bomb",
            Torpedo => "Torpedo",
            Projectile => "Projectile",
            Beam => "Beam",
            Decoy => "Decoy",
            Building => "Building",
            Bullseye => "Bullseye",
            Waypoint => "Waypoint",
            Tank => "Tank",
            Warship => "Warship",
            AircraftCarrier => "AircraftCarrier",
            Submarine => "Submarine",
            Infantry => "Infantry",
            Parachutist => "Parachutist",
            Shell => "Shell",
            Bullet => "Bullet",
            Flare => "Flare",
            Chaff => "Chaff",
            SmokeGrenade => "SmokeGrenade",
            Aerodrome => "Aerodrome",
            Container => "Container",
            Shrapnel => "Shrapnel",
            Unknown(tag) => tag,
        }
    }
}

impl FromStr for Coords {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('|').collect::<Vec<_>>();
        let mut coords = Coords::default();
        match &parts[..] {
            [longitude, latitude, altitude] => {
                if !longitude.is_empty() {
                    coords.longitude = Some(f64::from_str(longitude)?);
                }
                if !latitude.is_empty() {
                    coords.latitude = Some(f64::from_str(latitude)?);
                }
                if !altitude.is_empty() {
                    coords.altitude = Some(f64::from_str(altitude)?);
                }
            }
            [longitude, latitude, altitude, u, v] => {
                if !longitude.is_empty() {
                    coords.longitude = Some(f64::from_str(longitude)?);
                }
                if !latitude.is_empty() {
                    coords.latitude = Some(f64::from_str(latitude)?);
                }
                if !altitude.is_empty() {
                    coords.altitude = Some(f64::from_str(altitude)?);
                }
                if !u.is_empty() {
                    coords.u = Some(f64::from_str(u)?);
                }
                if !v.is_empty() {
                    coords.v = Some(f64::from_str(v)?);
                }
            }
            [longitude, latitude, altitude, roll, pitch, yaw] => {
                if !longitude.is_empty() {
                    coords.longitude = Some(f64::from_str(longitude)?);
                }
                if !latitude.is_empty() {
                    coords.latitude = Some(f64::from_str(latitude)?);
                }
                if !altitude.is_empty() {
                    coords.altitude = Some(f64::from_str(altitude)?);
                }
                if !roll.is_empty() {
                    coords.roll = Some(f64::from_str(roll)?);
                }
                if !pitch.is_empty() {
                    coords.pitch = Some(f64::from_str(pitch)?);
                }
                if !yaw.is_empty() {
                    coords.yaw = Some(f64::from_str(yaw)?);
                }
            }
            [longitude, latitude, altitude, roll, pitch, yaw, u, v, heading] => {
                if !longitude.is_empty() {
                    coords.longitude = Some(f64::from_str(longitude)?);
                }
                if !latitude.is_empty() {
                    coords.latitude = Some(f64::from_str(latitude)?);
                }
                if !altitude.is_empty() {
                    coords.altitude = Some(f64::from_str(altitude)?);
                }
                if !roll.is_empty() {
                    coords.roll = Some(f64::from_str(roll)?);
                }
                if !pitch.is_empty() {
                    coords.pitch = Some(f64::from_str(pitch)?);
                }
                if !yaw.is_empty() {
                    coords.yaw = Some(f64::from_str(yaw)?);
                }
                if !u.is_empty() {
                    coords.u = Some(f64::from_str(u)?);
                }
                if !v.is_empty() {
                    coords.v = Some(f64::from_str(v)?);
                }
                if !heading.is_empty() {
                    coords.heading = Some(f64::from_str(heading)?);
                }
            }
            _ => return Err(ParseError::InvalidCoordinateFormat),
        }
        Ok(coords)
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.heading.is_some() {
            write!(
                f,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}",
                NoneAsEmpty(self.longitude),
                NoneAsEmpty(self.latitude),
                NoneAsEmpty(self.altitude),
                NoneAsEmpty(self.roll),
                NoneAsEmpty(self.pitch),
                NoneAsEmpty(self.yaw),
                NoneAsEmpty(self.u),
                NoneAsEmpty(self.v),
                NoneAsEmpty(self.heading)
            )
        } else if self.yaw.is_some() || self.pitch.is_some() || self.roll.is_some() {
            write!(
                f,
                "{}|{}|{}|{}|{}|{}",
                NoneAsEmpty(self.longitude),
                NoneAsEmpty(self.latitude),
                NoneAsEmpty(self.altitude),
                NoneAsEmpty(self.roll),
                NoneAsEmpty(self.pitch),
                NoneAsEmpty(self.yaw),
            )
        } else if self.u.is_some() || self.v.is_some() {
            write!(
                f,
                "{}|{}|{}|{}|{}",
                NoneAsEmpty(self.longitude),
                NoneAsEmpty(self.latitude),
                NoneAsEmpty(self.altitude),
                NoneAsEmpty(self.u),
                NoneAsEmpty(self.v),
            )
        } else {
            write!(
                f,
                "{}|{}|{}",
                NoneAsEmpty(self.longitude),
                NoneAsEmpty(self.latitude),
                NoneAsEmpty(self.altitude),
            )
        }
    }
}

fn join<'a>(iter: impl Iterator<Item = &'a str>, sep: &'a str) -> String {
    iter.fold(String::new(), |mut acc, v| {
        if !acc.is_empty() {
            acc += sep;
        }
        acc + v
    })
}

struct NoneAsEmpty<V>(Option<V>);

impl<V: Display> Display for NoneAsEmpty<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.0 {
            v.fmt(f)
        } else {
            Ok(())
        }
    }
}

fn to_index(i: u8) -> Cow<'static, str> {
    match i {
        0 => Cow::Borrowed(""),
        1 => Cow::Borrowed("2"),
        2 => Cow::Borrowed("3"),
        3 => Cow::Borrowed("4"),
        4 => Cow::Borrowed("5"),
        5 => Cow::Borrowed("6"),
        6 => Cow::Borrowed("7"),
        7 => Cow::Borrowed("8"),
        8 => Cow::Borrowed("9"),
        i => Cow::Owned((i + 1).to_string()),
    }
}

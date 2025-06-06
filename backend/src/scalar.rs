use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Unit {
    Percent,
    Ratio,
    Celsius,
    Fahrenheit,
    Degrees,
    KiloPascal,
    Pascal,
    RPM,
    KilometersPerHour,
    MilesPerHour,
    GramsPerSecond,
    Volts,
    Seconds,
    Hours,
    Minutes,
    Kilometers,
    Meters,
    Miles,
    Feet,
    Milliampere,
    LitresPerHour,
    GallonsPerHour,
    NewtonMeters,
    FootPounds,
    KilogramsPerSecond,
    PartsPerMillion,
    MiligramsPerStroke,
    PSI,
    Unknown,
    NoData,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ParseUnitError;

impl FromStr for Unit {
    type Err = ParseUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "%" => Ok(Unit::Percent),
            "ratio" => Ok(Unit::Ratio),
            "°C" => Ok(Unit::Celsius),
            "°F" => Ok(Unit::Fahrenheit),
            "°" => Ok(Unit::Degrees),
            "kPa" => Ok(Unit::KiloPascal),
            "Pa" => Ok(Unit::Pascal),
            "RPM" => Ok(Unit::RPM),
            "km/h" => Ok(Unit::KilometersPerHour),
            "g/s" => Ok(Unit::GramsPerSecond),
            "V" => Ok(Unit::Volts),
            "s" => Ok(Unit::Seconds),
            "h" => Ok(Unit::Hours),
            "mins" => Ok(Unit::Minutes),
            "km" => Ok(Unit::Kilometers),
            "mA" => Ok(Unit::Milliampere),
            "L/h" => Ok(Unit::LitresPerHour),
            "Nm" => Ok(Unit::NewtonMeters),
            "Kg/s" => Ok(Unit::KilogramsPerSecond),
            "ppm" => Ok(Unit::PartsPerMillion),
            "mg/stroke" => Ok(Unit::MiligramsPerStroke),
            "PSI" => Ok(Unit::PSI),
            "mph" => Ok(Unit::MilesPerHour),
            "m" => Ok(Unit::Meters),
            "mi" => Ok(Unit::Miles),
            "ft" => Ok(Unit::Feet),
            "gal/h" => Ok(Unit::GallonsPerHour),
            "ft-lb" => Ok(Unit::FootPounds),
            _ => Err(ParseUnitError),
        }
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::NoData
    }
}

impl Unit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Unit::Percent => "%",
            Unit::Ratio => "",
            Unit::Celsius => "°C",
            Unit::Fahrenheit => "°F",
            Unit::Degrees => "°",
            Unit::KiloPascal => "kPa",
            Unit::Pascal => "Pa",
            Unit::RPM => "RPM",
            Unit::KilometersPerHour => "km/h",
            Unit::GramsPerSecond => "g/s",
            Unit::Volts => "V",
            Unit::Seconds => "s",
            Unit::Hours => "h",
            Unit::Minutes => "mins",
            Unit::Kilometers => "km",
            Unit::Milliampere => "mA",
            Unit::LitresPerHour => "L/h",
            Unit::NewtonMeters => "Nm",
            Unit::KilogramsPerSecond => "Kg/s",
            Unit::PartsPerMillion => "ppm",
            Unit::MiligramsPerStroke => "mg/stroke",
            Unit::PSI => "PSI",
            Unit::MilesPerHour => "mph",
            Unit::Meters => "m",
            Unit::Miles => "mi",
            Unit::Feet => "ft",
            Unit::GallonsPerHour => "gal/h",
            Unit::FootPounds => "ft-lb",
            Unit::NoData => "NO DATA",
            Unit::Unknown => "",
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Scalar {
    pub value: f32,
    pub unit: Unit,
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.unit == Unit::NoData {
            return write!(f, "NO DATA");
        }

        write!(f, "{}{}", self.value, self.unit.as_str())
    }
}

// Doesn't matter what unit you are using.
impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, other: Self) -> Self::Output {
        Scalar::new(self.value - other.value, self.unit, None)
    }
}

// Doesn't matter what unit you are using.
impl Add for Scalar {
    type Output = Scalar;

    fn add(self, other: Self) -> Self::Output {
        Scalar::new(self.value + other.value, self.unit, None)
    }
}

impl Scalar {
    pub fn convert(&self, target_unit: Unit) -> Option<Self> {
        use Unit::*;

        match (&self.unit, target_unit) {
            // Distance and speed
            (Kilometers, Meters) => Some(Scalar::new(self.value * 1000.0, Meters, None)),
            (Meters, Kilometers) => Some(Scalar::new(self.value / 1000.0, Kilometers, None)),

            (KilometersPerHour, MilesPerHour) | (Kilometers, Miles) => {
                Some(Scalar::new(self.value / 1.609, target_unit, None))
            }

            (MilesPerHour, KilometersPerHour) | (Miles, Kilometers) => {
                Some(Scalar::new(self.value * 1.609, target_unit, None))
            }

            (Kilometers, Feet) => Some(Scalar::new(self.value * 83281.0, Feet, None)),
            (Feet, Kilometers) => Some(Scalar::new(self.value / 83281.0, Kilometers, None)),

            // Temperature
            (Celsius, Fahrenheit) => Some(Scalar::new((self.value * 1.8) + 32.0, Fahrenheit, None)),
            (Fahrenheit, Celsius) => Some(Scalar::new((self.value - 32.0) * 1.8, Celsius, None)),

            // Time
            (Seconds, Minutes) => Some(Scalar::new(self.value / 60.0, Minutes, None)),
            (Seconds, Hours) => Some(Scalar::new(self.value / 3600.0, Hours, None)),
            (Minutes, Seconds) => Some(Scalar::new(self.value * 60.0, Seconds, None)),
            (Minutes, Hours) => Some(Scalar::new(self.value / 60.0, Hours, None)),
            (Hours, Seconds) => Some(Scalar::new(self.value * 3600.0, Seconds, None)),
            (Hours, Minutes) => Some(Scalar::new(self.value * 60.0, Minutes, None)),

            // Volume
            (LitresPerHour, GallonsPerHour) => {
                Some(Scalar::new(self.value * 0.264172, GallonsPerHour, None))
            }
            (GallonsPerHour, LitresPerHour) => {
                Some(Scalar::new(self.value / 0.264172, LitresPerHour, None))
            }

            // Energy
            (NewtonMeters, FootPounds) => Some(Scalar::new(self.value * 0.73756, FootPounds, None)),
            (FootPounds, NewtonMeters) => {
                Some(Scalar::new(self.value / 0.73756, NewtonMeters, None))
            }

            // Pressure
            (KiloPascal, PSI) => Some(Scalar::new(self.value / 6.895, Unit::PSI, None)),
            (PSI, KiloPascal) => Some(Scalar::new(self.value * 6.895, Unit::KiloPascal, None)),
            (KiloPascal, Pascal) => Some(Scalar::new(self.value * 1000.0, Unit::Pascal, None)),
            (Pascal, KiloPascal) => Some(Scalar::new(self.value / 1000.0, Unit::KiloPascal, None)),

            (_, _) => None,
        }
    }

    pub fn new(value: f32, unit: Unit, preferences: Option<UnitPreferences>) -> Self {
        if let Some(preferences) = preferences {
            let target_unit = *match &unit {
                Unit::KilometersPerHour | Unit::MilesPerHour => preferences.speed(),
                Unit::Kilometers | Unit::Miles | Unit::Meters | Unit::Feet => {
                    preferences.distance()
                }
                Unit::Celsius | Unit::Fahrenheit | Unit::Degrees => preferences.temp(),
                Unit::NewtonMeters | Unit::FootPounds => preferences.torque(),
                Unit::KiloPascal | Unit::Pascal | Unit::PSI => preferences.pressure(),
                Unit::LitresPerHour | Unit::GallonsPerHour => preferences.flow_rate(),
                _ => &unit,
            };

            if target_unit != unit {
                if let Some(converted) = Scalar::new(value, unit, None).convert(target_unit) {
                    return converted;
                }
            }

            // unit is already target unit
            return Self { value, unit };
        }

        Self { value, unit }
    }

    pub fn no_data() -> Self {
        Self {
            value: 0.0,
            unit: Unit::NoData,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitPreferences {
    speed: Unit,
    distance: Unit,
    temperature: Unit,
    torque: Unit,
    pressure: Unit,
    flow_rate: Unit,
}

impl Default for UnitPreferences {
    /// Default global unit preferences
    /// The defaults can be changed here.
    fn default() -> Self {
        Self {
            speed: Unit::KilometersPerHour,
            distance: Unit::Kilometers,
            temperature: Unit::Celsius,
            torque: Unit::NewtonMeters,
            pressure: Unit::KiloPascal,
            flow_rate: Unit::LitresPerHour,
        }
    }
}

impl UnitPreferences {
    pub fn speed(&self) -> &Unit {
        &self.speed
    }
    pub fn distance(&self) -> &Unit {
        &self.distance
    }
    pub fn temp(&self) -> &Unit {
        &self.temperature
    }
    pub fn torque(&self) -> &Unit {
        &self.torque
    }
    pub fn pressure(&self) -> &Unit {
        &self.pressure
    }
    pub fn flow_rate(&self) -> &Unit {
        &self.flow_rate
    }
    pub fn set_speed(&mut self, new: Unit) {
        self.speed = new;
    }
    pub fn set_distance(&mut self, new: Unit) {
        self.distance = new;
    }
    pub fn set_temp(&mut self, new: Unit) {
        self.temperature = new;
    }
    pub fn set_torque(&mut self, new: Unit) {
        self.torque = new;
    }
    pub fn set_pressure(&mut self, new: Unit) {
        self.pressure = new;
    }
    pub fn set_flow_rate(&mut self, new: Unit) {
        self.flow_rate = new;
    }
}

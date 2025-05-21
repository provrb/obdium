use std::{fmt, str::FromStr};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
            "" => Ok(Unit::Ratio),
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
            _ => "",
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

impl Scalar {
    pub fn new(value: f32, unit: Unit) -> Self {
        Self { value, unit }
    }

    pub fn no_data() -> Self {
        Self {
            value: 0.0,
            unit: Unit::NoData,
        }
    }

    pub fn to(&self, target_unit: Unit) -> Option<Self> {
        use Unit::*;

        match (&self.unit, target_unit) {
            // Distance and speed
            (Kilometers, Meters) => Some(Scalar::new(self.value * 1000.0, Meters)),
            (Meters, Kilometers) => Some(Scalar::new(self.value / 1000.0, Kilometers)),

            (KilometersPerHour, MilesPerHour) | (Kilometers, Miles) => {
                Some(Scalar::new(self.value / 1.609, Miles))
            }
            (MilesPerHour, KilometersPerHour) | (Miles, Kilometers) => {
                Some(Scalar::new(self.value * 1.609, Kilometers))
            }

            (Kilometers, Feet) => Some(Scalar::new(self.value * 83281.0, Feet)),
            (Feet, Kilometers) => Some(Scalar::new(self.value / 83281.0, Kilometers)),

            // Temperature
            (Celsius, Fahrenheit) => Some(Scalar::new((self.value * 1.8) + 32.0, Fahrenheit)),
            (Fahrenheit, Celsius) => Some(Scalar::new((self.value - 32.0) * 1.8, Celsius)),

            // Time
            (Seconds, Minutes) => Some(Scalar::new(self.value / 60.0, Minutes)),
            (Seconds, Hours) => Some(Scalar::new(self.value / 3600.0, Hours)),
            (Minutes, Seconds) => Some(Scalar::new(self.value * 60.0, Seconds)),
            (Minutes, Hours) => Some(Scalar::new(self.value / 60.0, Hours)),
            (Hours, Seconds) => Some(Scalar::new(self.value * 3600.0, Seconds)),
            (Hours, Minutes) => Some(Scalar::new(self.value * 60.0, Minutes)),

            // Volume
            (LitresPerHour, GallonsPerHour) => {
                Some(Scalar::new(self.value * 0.264172, GallonsPerHour))
            }
            (GallonsPerHour, LitresPerHour) => {
                Some(Scalar::new(self.value / 0.264172, LitresPerHour))
            }

            // Energy
            (NewtonMeters, FootPounds) => Some(Scalar::new(self.value * 0.73756, FootPounds)),
            (FootPounds, NewtonMeters) => Some(Scalar::new(self.value / 0.73756, NewtonMeters)),

            (_, _) => None,
        }
    }
}

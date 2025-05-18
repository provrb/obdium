use std::fmt;

#[derive(Clone)]
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
    GramsPerSecond,
    Volts,
    Seconds,
    Hours,
    Minutes,
    Kilometers,
    Milliampere,
    LitresPerHour,
    NewtonMeters,
    KilogramsPerSecond,
    PartsPerMillion,
    MiligramsPerStroke,
    None,
    NoData,
}

#[derive(Clone)]
pub struct Scalar {
    value: f32,
    unit: Unit,
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;
        match self.unit {
            Unit::Percent => write!(f, "%"),
            Unit::Ratio => write!(f, "ratio"),
            Unit::Celsius => write!(f, "°C"),
            Unit::Fahrenheit => write!(f, "°F"),
            Unit::Degrees => write!(f, "°"),
            Unit::KiloPascal => write!(f, "kPa"),
            Unit::Pascal => write!(f, "Pa"),
            Unit::RPM => write!(f, "RPM"),
            Unit::KilometersPerHour => write!(f, "Kmh"),
            Unit::GramsPerSecond => write!(f, "g/s"),
            Unit::Volts => write!(f, "V"),
            Unit::Seconds => write!(f, "s"),
            Unit::Hours => write!(f, "h"),
            Unit::Minutes => write!(f, "mins"),
            Unit::Kilometers => write!(f, "km"),
            Unit::Milliampere => write!(f, "mA"),
            Unit::LitresPerHour => write!(f, "L/h"),
            Unit::NewtonMeters => write!(f, "Nm"),
            Unit::KilogramsPerSecond => write!(f, "Kg/s"),
            Unit::PartsPerMillion => write!(f, "ppm"),
            Unit::MiligramsPerStroke => write!(f, "mg/stroke"),
            Unit::NoData => write!(f, "??"),
            _ => write!(f, ""),
        }
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
}

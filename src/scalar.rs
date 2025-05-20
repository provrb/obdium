use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    NoData,
}

#[derive(Clone)]
pub struct Scalar {
    value: f32,
    unit: Unit,
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.unit == Unit::NoData {
            return write!(f, "NO DATA");
        }

        write!(f, "{}", self.value)?;
        match self.unit {
            Unit::Percent => write!(f, "%"),
            Unit::Ratio => write!(f, ""),
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

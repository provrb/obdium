use crate::cmd::Command;
use crate::obd::{SensorNumber, OBD};
use crate::scalar::{Scalar, Unit};

#[derive(Debug)]
pub enum SecondaryAirStatus {
    Status(&'static str),
}

impl SecondaryAirStatus {
    pub fn from_u8(num: u8) -> Self {
        match num {
            1u8 => Self::Status("Upstream"),
            2u8 => Self::Status("Downstream of catalytic converter"),
            4u8 => Self::Status("From the outside atmosphere or off"),
            8u8 => Self::Status("Pump commanded on for diagnostics"),
            _ => Self::Status("Unknown secondary air status"),
        }
    }
}

impl OBD {
    pub fn secondary_air_status(&mut self) -> SecondaryAirStatus {
        let response = self.query(Command::new_pid(b"0112"));
        SecondaryAirStatus::from_u8(response.a_value() as u8)
    }

    pub fn intake_air_temp(&mut self) -> Scalar {
        self.query(Command::new_pid(b"010F"))
            .map_no_data(|r| Scalar::new(r.a_value() - 40.0, Unit::Celsius))
    }

    pub fn maf_air_flow_rate(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0110")).map_no_data(|r| {
            Scalar::new(
                ((r.a_value() * 256.0) + r.b_value()) / 100.0,
                Unit::GramsPerSecond,
            )
        })
    } // Mass airflow sensor

    pub fn ambient_air_temp(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0146"))
            .map_no_data(|r| Scalar::new(r.a_value() - 40.0, Unit::Celsius))
    }

    pub fn max_air_flow_rate_from_maf(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0150"))
            .map_no_data(|r| Scalar::new(r.a_value() * 10.0, Unit::GramsPerSecond))
    }

    pub fn read_oxygen_sensor(&mut self, sensor: &SensorNumber) -> (Scalar, Scalar) {
        let command = match sensor {
            SensorNumber::Sensor1 => Command::new_pid(b"0114"),
            SensorNumber::Sensor2 => Command::new_pid(b"0115"),
            SensorNumber::Sensor3 => Command::new_pid(b"0116"),
            SensorNumber::Sensor4 => Command::new_pid(b"0117"),
            SensorNumber::Sensor5 => Command::new_pid(b"0118"),
            SensorNumber::Sensor6 => Command::new_pid(b"0119"),
            SensorNumber::Sensor7 => Command::new_pid(b"011A"),
            SensorNumber::Sensor8 => Command::new_pid(b"011B"),
        };

        let response = self.query(command);
        if *response.get_payload_size() == 0 {
            return (Scalar::no_data(), Scalar::no_data());
        }

        (
            Scalar::new(response.a_value() / 200.0, Unit::Volts),
            Scalar::new(
                ((100.0 / 128.0) * response.b_value()) - 100.0,
                Unit::Percent,
            ),
        )
    }

    pub fn o2_sensor_air_fuel_ratio(&mut self, sensor: &SensorNumber) -> (Scalar, Scalar) {
        let command = match sensor {
            SensorNumber::Sensor1 => Command::new_pid(b"0124"),
            SensorNumber::Sensor2 => Command::new_pid(b"0125"),
            SensorNumber::Sensor3 => Command::new_pid(b"0126"),
            SensorNumber::Sensor4 => Command::new_pid(b"0127"),
            SensorNumber::Sensor5 => Command::new_pid(b"0128"),
            SensorNumber::Sensor6 => Command::new_pid(b"0129"),
            SensorNumber::Sensor7 => Command::new_pid(b"012A"),
            SensorNumber::Sensor8 => Command::new_pid(b"012B"),
        };

        let response = self.query(command);
        if *response.get_payload_size() == 0 {
            return (Scalar::no_data(), Scalar::no_data());
        }

        let ratio = (2.0 / 65536.0) * ((256.0 * response.a_value()) + response.b_value());
        let voltage = (8.0 / 65536.0) * ((256.0 * response.c_value()) + response.d_value());
        (
            Scalar::new(ratio, Unit::Ratio),
            Scalar::new(voltage, Unit::Volts),
        )
    }

    // Read from sensor a and b
    // If a sensor is not supported it will return 0
    pub fn read_mass_air_flow_sensor(&mut self) -> (Scalar, Scalar) {
        let mut data = (Scalar::no_data(), Scalar::no_data());
        let response = self.query(Command::new_pid(b"0166"));
        if *response.get_payload_size() == 0 {
            return data;
        }

        let sensors_supported = self.sensors_supported_for(response.a_value() as u8);

        if sensors_supported.contains(&SensorNumber::Sensor1) {
            data.0 = Scalar::new(
                ((256.0 * response.b_value()) + response.c_value()) / 32.0,
                Unit::GramsPerSecond,
            );
        }

        if sensors_supported.contains(&SensorNumber::Sensor2) {
            data.1 = Scalar::new(
                ((256.0 * response.d_value()) + response.e_value()) / 32.0,
                Unit::GramsPerSecond,
            );
        }

        data
    }

    pub fn abs_barometric_pressure(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0133"))
            .map_no_data(|r| Scalar::new(r.a_value(), Unit::KiloPascal))
    }
}

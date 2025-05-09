use crate::cmd::Command;
use crate::obd::{SensorNumber, OBD};

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

    pub fn intake_air_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010F"));
        response.a_value() - 40.0
    }

    pub fn maf_air_flow_rate(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0110"));
        ((response.a_value() * 256.0) + response.b_value()) / 100.0
    } // Mass airflow sensor

    pub fn ambient_air_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0146"));
        response.a_value() - 40.0
    }

    pub fn max_air_flow_rate_from_maf(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0150"));
        response.a_value() * 10.0
    }

    pub fn read_oxygen_sensor(&mut self, sensor: &SensorNumber) -> (f32, f32) {
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

        (
            response.a_value() / 200.0,
            ((100.0 / 128.0) * response.b_value()) - 100.0,
        )
    }

    pub fn o2_sensor_air_fuel_ratio(&mut self, sensor: &SensorNumber) -> (f32, f32) {
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
        let ratio = (2.0 / 65536.0) * ((256.0 * response.a_value()) + response.b_value());
        let voltage = (8.0 / 65536.0) * ((256.0 * response.c_value()) + response.d_value());
        (ratio, voltage)
    }

    // Read from sensor a and b
    // If a sensor is not supported it will return 0
    pub fn read_mass_air_flow_sensor(&mut self) -> (f32, f32) {
        let response = self.query(Command::new_pid(b"0166"));
        let sensors_supported = self.sensors_supported_for(response.a_value() as u8);
        let mut data = (0f32, 0f32);

        if sensors_supported.contains(&SensorNumber::Sensor1) {
            data.0 = ((256.0 * response.b_value()) + response.c_value()) / 32.0;
        }

        if sensors_supported.contains(&SensorNumber::Sensor2) {
            data.1 = ((256.0 * response.d_value()) + response.e_value()) / 32.0;
        }

        data
    }

    pub fn abs_barometric_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0133"));
        response.a_value()
    }
}

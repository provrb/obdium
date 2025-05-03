use crate::cmd::Command;
use crate::obd::{OxygenSensor, OBD};

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
        let response = self.query(Command::new_pid(b"0112")).unwrap_or_default();
        SecondaryAirStatus::from_u8(response.a_value() as u8)
    }

    pub fn intake_air_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010F")).unwrap_or_default();
        response.a_value() - 40.0
    }

    pub fn maf_air_flow_rate(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0110")).unwrap_or_default();
        ((response.a_value() * 256.0) + response.b_value()) / 100.0
    } // Mass airflow sensor

    pub fn ambient_air_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0146")).unwrap_or_default();
        response.a_value() - 40.0
    }

    pub fn max_air_flow_rate_from_maf(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0150")).unwrap_or_default();
        response.a_value() * 10.0
    }

    pub fn read_oxygen_sensor(&mut self, sensor: &OxygenSensor) -> (f32, f32) {
        let command;
        match sensor {
            OxygenSensor::Sensor1 => command = Command::new_pid(b"0114"),
            OxygenSensor::Sensor2 => command = Command::new_pid(b"0115"),
            OxygenSensor::Sensor3 => command = Command::new_pid(b"0116"),
            OxygenSensor::Sensor4 => command = Command::new_pid(b"0117"),
            OxygenSensor::Sensor5 => command = Command::new_pid(b"0118"),
            OxygenSensor::Sensor6 => command = Command::new_pid(b"0119"),
            OxygenSensor::Sensor7 => command = Command::new_pid(b"011A"),
            OxygenSensor::Sensor8 => command = Command::new_pid(b"011B"),
        }

        let response = self.query(command).unwrap_or_default();

        (
            response.a_value() / 200.0,
            ((100.0 / 128.0) * response.b_value()) - 100.0,
        )
    }

    pub fn o2_sensor_air_fuel_ratio(&mut self, sensor: &OxygenSensor) -> (f32, f32) {
        let command;
        match sensor {
            OxygenSensor::Sensor1 => command = Command::new_pid(b"0124"),
            OxygenSensor::Sensor2 => command = Command::new_pid(b"0125"),
            OxygenSensor::Sensor3 => command = Command::new_pid(b"0126"),
            OxygenSensor::Sensor4 => command = Command::new_pid(b"0127"),
            OxygenSensor::Sensor5 => command = Command::new_pid(b"0128"),
            OxygenSensor::Sensor6 => command = Command::new_pid(b"0129"),
            OxygenSensor::Sensor7 => command = Command::new_pid(b"012A"),
            OxygenSensor::Sensor8 => command = Command::new_pid(b"012B"),
        }

        let response = self.query(command).unwrap_or_default();
        let ratio = (2.0 / 65536.0) * ((256.0 * response.a_value()) + response.b_value());
        let voltage = (8.0 / 65536.0) * ((256.0 * response.c_value()) + response.d_value());
        (ratio, voltage)
    }

    // Read from sensor a and b
    // If a sensor is not supported it will return 0
    pub fn read_mass_air_flow_sensor(&mut self) -> (f32, f32) {
        let response = self.query(Command::new_pid(b"0166")).unwrap_or_default();
        let sensor_a_supported = (response.a_value() as u32 & 1) != 0;
        let sensor_b_supported = (response.a_value() as u32 & 2) != 0;
        let mut data = (0f32, 0f32);

        if sensor_a_supported {
            data.0 = ((256.0 * response.b_value()) + response.c_value()) / 32.0;
        }

        if sensor_b_supported {
            data.1 = ((256.0 * response.d_value()) + response.e_value()) / 32.0;
        }

        data
    }
}

use crate::cmd::Command;
use crate::obd::{OxygenSensor, OBD};

impl OBD {
    pub fn intake_manifold_abs_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010B")).unwrap_or_default();
        response.a_value()
    }

    pub fn vehicle_speed(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010D")).unwrap_or_default();
        response.a_value()
    }

    pub fn timing_advance(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010E")).unwrap_or_default();
        (response.a_value() / 2.0) - 64.0
    }

    pub fn intake_air_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010F")).unwrap_or_default();
        response.a_value() - 40.0
    }

    pub fn maf_air_flow_rate(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0110")).unwrap_or_default();
        ((response.a_value() * 256.0) + response.b_value()) / 100.0
    } // Mass airflow sensor

    pub fn ambient_air_temp(&self) -> f32 {
        todo!()
    }

    pub fn max_air_flow_rate_from_maf(&self) -> f32 {
        todo!()
    }

    pub fn throttle_position(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0111")).unwrap_or_default();
        response.a_value() * (100.0 / 255.0)
    }

    pub fn read_oxygen_sensor(&mut self, sensor: OxygenSensor) -> (f32, f32) {
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

    pub fn o2_sensor_air_fuel_ratio(&self) -> f32 {
        todo!()
    }

    pub fn read_mass_air_flow_sensor() {
        todo!()
    }

    pub fn max_values_for(&self) -> f32 {
        todo!()
    } // fuel-air equivalance ratio, o2 sensor voltage, current, and instake abs pressure

    pub fn relative_throttle_pos(&self) -> f32 {
        todo!()
    }

    pub fn absolute_throttle_pos(&self) -> f32 {
        todo!()
    }
}

use crate::cmd::Command;
use crate::obd::OBD;

#[derive(PartialEq, Eq)]
pub enum EngineType {
    SparkIgnition,
    CompressionIgnition,
}

impl OBD {
    pub fn rpm(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010C")).unwrap_or_default();
        ((256.0 * response.a_value()) + response.b_value()) / 4.0
    }

    pub fn engine_load(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0104")).unwrap_or_default();
        response.a_value() / 2.55
    }

    pub fn coolant_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0105")).unwrap_or_default();
        response.a_value() - 40.0
    }

    pub fn engine_fuel_rate(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"019D")).unwrap_or_default();
        response.a_value()
    }

    pub fn engine_runtime(&mut self) -> f32 {
        if self.get_engine_type() == EngineType::CompressionIgnition {
            return self.engine_runtime_diesel();
        }

        let response = self.query(Command::new_pid(b"011F")).unwrap_or_default();
        ( 256.0 * response.a_value() ) + response.b_value()
    }

    pub fn engine_runtime_diesel(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"017F")).unwrap_or_default();
        let b = response.b_value();
        let c = response.c_value();
        let d = response.d_value();
        let e = response.e_value();
        let b_power = f32::powf(2f32, 24f32);
        let c_power = f32::powf(2f32, 16f32);
        let d_power = f32::powf(2f32, 8f32);

        (b * b_power) + (c * c_power) + (d * d_power) + e
    }

    pub fn engine_mileage(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"01A6")).unwrap_or_default();
        let a = response.a_value();
        let b = response.b_value();
        let c = response.c_value();
        let d = response.d_value();

        let a_power = f32::powf(2f32, 24f32);
        let b_power = f32::powf(2f32, 16f32);
        let c_power = f32::powf(2f32, 8f32);

        ((a * a_power) + (b * b_power) + (c * c_power) + d) / 40.0
    }

    pub fn engine_oil_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"015C")).unwrap_or_default();
        response.a_value() - 40.0
    }

    pub fn drivers_demand_engine_torque(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0161")).unwrap_or_default();
        response.a_value() - 125.0
    }

    pub fn actual_engine_torque(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0162")).unwrap_or_default();
        response.a_value() - 125.0
    }

    pub fn reference_engine_torque(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0163")).unwrap_or_default();
        (256.0 * response.a_value()) + response.b_value()
    }

    pub fn engine_percent_torque_data(&mut self) -> (f32, f32, f32, f32, f32) {
        let response = self.query(Command::new_pid(b"0164")).unwrap_or_default();
        let idle = response.a_value() - 125.0;
        let engine_point_1 = response.b_value() - 125.0;
        let engine_point_2 = response.c_value() - 125.0;
        let engine_point_3 = response.d_value() - 125.0;
        let engine_point_4 = response.e_value() - 125.0;

        (
            idle,
            engine_point_1,
            engine_point_2,
            engine_point_3,
            engine_point_4,
        )
    }

    pub fn get_engine_type(&mut self) -> EngineType {
        let response = self.query(Command::new_pid(b"0101")).unwrap_or_default();
        match response.b_value() as u32 & 0b00001000 {
            0 => EngineType::SparkIgnition,
            _ => EngineType::CompressionIgnition,
        }
    }
}

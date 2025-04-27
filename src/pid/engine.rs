use std::intrinsics::sqrtf32;

use crate::cmd::Command;
use crate::obd::OBD;

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
        let response = self.query(Command::new_pid(b"017F")).unwrap_or_default();
        let b = response.b_value();
        let c = response.c_value();
        let d = response.d_value();
        let e = response.e_value();
        let b_power = f32::powf(2f32, 21f32);
        let c_power = f32::powf(2f32, 16f32);
        let d_power = f32::powf(2f32, 8f32);

        (b * b_power) + (c * c_power) + (d * d_power) + e
    }

    pub fn engine_oil_temp(&self) -> f32 {
        todo!()
    }

    pub fn drivers_demand_engine_torque(&self) -> f32 {
        todo!()
    }

    pub fn actual_engine_torque(&self) -> f32 {
        todo!()
    }

    pub fn reference_engine_torque(&self) -> f32 {
        todo!()
    }

    pub fn engine_percent_torque_data(&self) -> f32 {
        todo!()
    }
}

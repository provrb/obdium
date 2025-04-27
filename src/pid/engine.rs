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

    pub fn engine_fuel_rate(&self) -> f32 {
        todo!()
    }

    pub fn engine_runtime(&self) -> f32 {
        todo!()
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

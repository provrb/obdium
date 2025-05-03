use crate::obd::OBD;
use crate::cmd::Command;

impl OBD {
    // Commanded exhaust gas recirculation
    pub fn commanded_egr(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"012C")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

    // Exhaust gas recirculation error
    pub fn egr_error(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"012D")).unwrap_or_default();
        ((100.0 / 128.0) * response.a_value()) - 100.0
    }

    pub fn boost_pressure_control(&self) -> f32 {
        todo!()
    }

    pub fn turbocharger_rpm(&self) -> f32 {
        todo!()
    }

    pub fn turbocharger_temp(&self) -> f32 {
        todo!()
    }

    pub fn exhaust_pressure(&self) -> f32 {
        todo!()
    }

    pub fn exhaust_gas_temp(&self) -> f32 {
        todo!()
    }
}

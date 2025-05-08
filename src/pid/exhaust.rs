use crate::cmd::Command;
use crate::obd::{BankNumber, SensorNumber, OBD};

impl OBD {
    // Commanded exhaust gas recirculation
    pub fn commanded_egr(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"012C"));
        (100.0 / 255.0) * response.a_value()
    }

    // Exhaust gas recirculation error
    pub fn egr_error(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"012D"));
        ((100.0 / 128.0) * response.a_value()) - 100.0
    }

    pub fn catalyst_temp(&mut self, bank: BankNumber, sensor: SensorNumber) -> f32 {
        let command;
        match (bank, sensor) {
            (BankNumber::Bank1, SensorNumber::Sensor1) => command = Command::new_pid(b"013C"),
            (BankNumber::Bank2, SensorNumber::Sensor1) => command = Command::new_pid(b"013D"),
            (BankNumber::Bank1, SensorNumber::Sensor2) => command = Command::new_pid(b"013E"),
            (BankNumber::Bank2, SensorNumber::Sensor2) => command = Command::new_pid(b"013F"),
            _ => {
                println!("catalyst temperature only supports bank 1, bank 2, sensor 1, and sensor 2 queries.");
                return -1f32;
            }
        }

        let response = self.query(command);
        (((256.0 * response.a_value()) + response.b_value()) / 10.0) - 40.0
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

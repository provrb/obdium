use crate::cmd::Command;
use crate::obd::{BankNumber, OBD};

impl OBD {
    pub fn short_term_fuel_trim(&mut self, bank: BankNumber) -> f32 {
        let mut command = Command::default();

        match bank {
            BankNumber::Bank1 => {
                command.set_pid(b"0106");
            }
            BankNumber::Bank2 => {
                command.set_pid(b"0108");
            }
        }

        let response = match self.query(command) {
            Some(data) => data,
            None => {
                println!("failed to get short term fuel trim. bank: {bank:?}. response is 'None'");
                return 0.0;
            }
        };

        (response.a_value() / 1.28) - 100.0
    }

    pub fn long_term_fuel_trim(&mut self, bank: BankNumber) -> f32 {
        let mut command = Command::default();

        match bank {
            BankNumber::Bank1 => {
                command.set_pid(b"0107");
            }
            BankNumber::Bank2 => {
                command.set_pid(b"0109");
            }
        }

        let response = match self.query(command) {
            Some(data) => data,
            None => {
                println!("failed to get long term fuel trim. bank: {bank:?}. response is 'None'");
                return 0.0;
            }
        };

        (response.a_value() / 1.28) - 100.0
    }

    pub fn fuel_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010A")).unwrap_or_default();

        response.a_value() * 3.0
    }

    pub fn fuel_rail_pressure(&self) -> f32 {
        todo!()
    }

    pub fn fuel_rail_guage_pressure(&self) -> f32 {
        todo!()
    }

    pub fn fuel_type(&self) -> f32 {
        todo!()
    }

    pub fn ethanol_fuel_percentage(&self) -> f32 {
        todo!()
    }

    pub fn fuel_injection_timing(&self) -> f32 {
        todo!()
    }

    pub fn emission_requirements(&self) -> f32 {
        todo!()
    }
}

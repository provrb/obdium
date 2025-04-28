use crate::cmd::Command;
use crate::obd::{BankNumber, OBD};

#[derive(Debug)]
pub enum FuelType {
    Gasoline = 0x01,
    Methanol = 0x02,
    Ethanol = 0x03,
    Diesel = 0x04,
    LPG = 0x05,
    CNG = 0x06,
    Propane = 0x07,
    Electric = 0x08,
    BifuelGasoline = 0x09,
    BifuelMethanol = 0x0A,
    BifuelEthanol = 0x0B,
    BifuelLPG = 0x0C,
    BifuelCNG = 0x0D,
    BifuelPropane = 0x0E,
    BifuelElectric = 0x0F,
    BifuelElectricEngine = 0x10,
    HybridGasoline = 0x11,
    HybridEthanol = 0x12,
    HybridDiesel = 0x13,
    HybridElectric = 0x14,
    HybridElectricEngine = 0x15,
    HybridRegenerative = 0x16,
    BifuelDiesel = 0x17,
    Unknown = 0x00,
}

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

    pub fn fuel_tank_level(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"012F")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

    pub fn fuel_rail_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0122")).unwrap_or_default();
        0.079 * ((256.0 * response.a_value()) + response.b_value())
    }

    pub fn fuel_rail_guage_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0123")).unwrap_or_default();
        10.0 * ((256.0 * response.a_value()) + response.b_value())
    }

    pub fn fuel_type(&mut self) -> FuelType {
        let response = self.query(Command::new_pid(b"0151")).unwrap_or_default();
        let fuel_index = response.a_value();

        match fuel_index {
            0f32 => FuelType::Unknown,
            1f32 => FuelType::Gasoline,
            2f32 => FuelType::Methanol,
            3f32 => FuelType::Ethanol,
            4f32 => FuelType::Diesel,
            5f32 => FuelType::LPG,
            6f32 => FuelType::CNG,
            7f32 => FuelType::Propane,
            8f32 => FuelType::Electric,
            9f32 => FuelType::BifuelGasoline,
            10f32 => FuelType::BifuelMethanol,
            11f32 => FuelType::BifuelEthanol,
            12f32 => FuelType::BifuelLPG,
            13f32 => FuelType::BifuelCNG,
            14f32 => FuelType::BifuelPropane,
            15f32 => FuelType::BifuelElectric,
            16f32 => FuelType::BifuelElectricEngine,
            17f32 => FuelType::HybridGasoline,
            18f32 => FuelType::HybridEthanol,
            19f32 => FuelType::HybridDiesel,
            20f32 => FuelType::HybridElectric,
            21f32 => FuelType::HybridElectricEngine,
            22f32 => FuelType::HybridRegenerative,
            23f32 => FuelType::BifuelDiesel,
            _ => FuelType::Unknown,
        }
    }

    pub fn ethanol_fuel_percentage(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0152")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

    pub fn fuel_injection_timing(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"015D")).unwrap_or_default();
        (((256.0 * response.a_value()) + response.b_value()) / 128.0) - 210.0
    }
}

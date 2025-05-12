use std::fmt;

use crate::cmd::Command;
use crate::obd::{BankNumber, OBD};

#[derive(Debug)]
pub enum FuelType {
    Type(&'static str),
}

impl fmt::Display for FuelType {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuelType::Type(s) => write!(f, "{}", s),
        }
    }
}

impl FuelType {
    pub fn from_u8(num: u8) -> Self {
        match num {
            0x01 => Self::Type("Gasoline"),
            0x02 => Self::Type("Methanol"),
            0x03 => Self::Type("Ethanol"),
            0x04 => Self::Type("Diesel"),
            0x05 => Self::Type("LPG"),
            0x06 => Self::Type("CNG"),
            0x07 => Self::Type("Propane"),
            0x08 => Self::Type("Electric"),
            0x09 => Self::Type("Bifuel Gasoline"),
            0x0A => Self::Type("Bifuel Methanol"),
            0x0B => Self::Type("Bifuel Ethanol"),
            0x0C => Self::Type("Bifuel LPG"),
            0x0D => Self::Type("Bifuel CNG"),
            0x0E => Self::Type("Bifuel Propane"),
            0x0F => Self::Type("Bifuel Electric"),
            0x10 => Self::Type("Bifuel ElectricEngine"),
            0x11 => Self::Type("Hybrid Gasoline"),
            0x12 => Self::Type("Hybrid Ethanol"),
            0x13 => Self::Type("Hybrid Diesel"),
            0x14 => Self::Type("Hybrid Electric"),
            0x15 => Self::Type("Hybrid ElectricEngine"),
            0x16 => Self::Type("Hybrid Regenerative"),
            0x17 => Self::Type("Bifuel Diesel"),
            _ => Self::Type("Unknown"),
        }
    }
}

#[derive(Debug)]
pub enum FuelSystemStatus {
    Status(&'static str),
}

impl FuelSystemStatus {
    pub fn from_u8(num: u8) -> Self {
        match num {
            0u8 => Self::Status("Motor is off"),
            1u8 => Self::Status("Open loop due to insufficient engine temperature"),
            2u8 => Self::Status("Closed loop, using O2 sensor feedback for fuel mix"),
            4u8 => Self::Status("Open loop due to engine load, or fuel cut due to deceleration"),
            8u8 => Self::Status("Open loop due to system failure"),
            16u8 => Self::Status("Closed loop. Feedback system fault. Using atleast one O2 sensor"),
            _ => Self::Status("Unknown fuel system status"),
        }
    }
}

#[derive(Debug)]
pub enum FuelDeliveryType {
    Type(&'static str),
}

impl FuelDeliveryType {
    pub fn from_u8(num: u8) -> Self {
        match num {
            1u8 => Self::Type("Stoichiometric Gasoline Direction Injection"),
            2u8 => Self::Type("Lean-Burn Gasoline Direct Injection"),
            3u8 => Self::Type("Multipoint Fuel Injection"),
            4u8 => Self::Type("Sequential Fuel Injection"),
            5u8 => Self::Type("Throttle Body Fuel Injection"),
            6u8 => Self::Type("Common Rail Direct Injection Diesel"),
            7u8 => Self::Type("Unit Injector Direct Injection Diesel"),
            9u8 => Self::Type("Compression Ignition"),
            10u8 => Self::Type("Transistor Controlled Ignition"),
            _ => Self::Type("Unknown"),
        }
    }
}

impl fmt::Display for FuelDeliveryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FuelDeliveryType::Type(s) => write!(f, "{}", s),
        }
    }
}

impl OBD {
    pub fn short_term_fuel_trim(&mut self, bank: BankNumber) -> f32 {
        let mut command = Command::default();

        match bank {
            BankNumber::Bank1 => command.set_pid(b"0106"),
            BankNumber::Bank2 => command.set_pid(b"0108"),
        }

        let response = self.query(command);

        (response.a_value() / 1.28) - 100.0
    }

    pub fn long_term_fuel_trim(&mut self, bank: BankNumber) -> f32 {
        let command = match bank {
            BankNumber::Bank1 => Command::new_pid(b"0107"),
            BankNumber::Bank2 => Command::new_pid(b"0109"),
        };

        let response = self.query(command);
        (response.a_value() / 1.28) - 100.0
    }

    pub fn fuel_system_status(&mut self) -> (FuelSystemStatus, FuelSystemStatus) {
        let response = self.query(Command::new_pid(b"0103"));

        (
            FuelSystemStatus::from_u8(response.a_value() as u8),
            FuelSystemStatus::from_u8(response.b_value() as u8),
        )
    }

    pub fn fuel_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010A"));
        response.a_value() * 3.0
    }

    pub fn fuel_tank_level(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"012F"));
        (100.0 / 255.0) * response.a_value()
    }

    pub fn fuel_rail_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0122"));
        0.079 * ((256.0 * response.a_value()) + response.b_value())
    }

    pub fn fuel_rail_guage_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0123"));
        10.0 * ((256.0 * response.a_value()) + response.b_value())
    }

    pub fn fuel_type(&mut self) -> FuelType {
        let response = self.query(Command::new_pid(b"0151"));

        FuelType::from_u8(response.a_value() as u8)
    }

    pub fn ethanol_fuel_percentage(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0152"));
        (100.0 / 255.0) * response.a_value()
    }

    pub fn fuel_injection_timing(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"015D"));
        (((256.0 * response.a_value()) + response.b_value()) / 128.0) - 210.0
    }

    pub fn commanded_evap_purge(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"012E"));
        (100.0 / 255.0) * response.a_value()
    }

    pub fn evap_system_vapor_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0132"));
        ((256.0 * response.a_value()) + response.b_value()) / 4.0
    }

    pub fn cylinder_fuel_rate(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"01A2"));
        ((256.0 * response.a_value()) + response.b_value()) / 32.0
    }
}

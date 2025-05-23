use std::fmt;

use crate::{
    scalar::{Scalar, Unit},
    BankNumber, Command, OBD,
};

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

impl OBD {
    pub fn short_term_fuel_trim(&mut self, bank: &BankNumber) -> Scalar {
        let mut command = Command::default();

        match bank {
            BankNumber::Bank1 => command.set_pid(b"0106"),
            BankNumber::Bank2 => command.set_pid(b"0108"),
        }

        self.query(command)
            .map_no_data(|r| Scalar::new((r.a_value() / 1.28) - 100.0, Unit::Percent))
    }

    pub fn long_term_fuel_trim(&mut self, bank: &BankNumber) -> Scalar {
        let command = match bank {
            BankNumber::Bank1 => Command::new_pid(b"0107"),
            BankNumber::Bank2 => Command::new_pid(b"0109"),
        };

        self.query(command)
            .map_no_data(|r| Scalar::new((r.a_value() / 1.28) - 100.0, Unit::Percent))
    }

    pub fn fuel_system_status(&mut self) -> (FuelSystemStatus, FuelSystemStatus) {
        let response = self.query(Command::new_pid(b"0103"));
        if *response.get_payload_size() == 0 {
            // unknown
            return (
                FuelSystemStatus::from_u8(200),
                FuelSystemStatus::from_u8(200),
            );
        }

        (
            FuelSystemStatus::from_u8(response.a_value() as u8),
            FuelSystemStatus::from_u8(response.b_value() as u8),
        )
    }

    pub fn fuel_pressure(&mut self) -> Scalar {
        self.query(Command::new_pid(b"010A"))
            .map_no_data(|r| Scalar::new(r.a_value() * 3.0, Unit::KiloPascal))
    }

    pub fn fuel_tank_level(&mut self) -> Scalar {
        self.query(Command::new_pid(b"012F"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    pub fn fuel_rail_pressure(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0122")).map_no_data(|r| {
            Scalar::new(
                0.079 * ((256.0 * r.a_value()) + r.b_value()),
                Unit::KiloPascal,
            )
        })
    }

    pub fn fuel_rail_guage_pressure(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0123")).map_no_data(|r| {
            Scalar::new(
                10.0 * ((256.0 * r.a_value()) + r.b_value()),
                Unit::KiloPascal,
            )
        })
    }

    pub fn fuel_type(&mut self) -> FuelType {
        let response = self.query(Command::new_pid(b"0151"));
        if *response.get_payload_size() == 0 {
            // unknown
            return FuelType::from_u8(200);
        }

        FuelType::from_u8(response.a_value() as u8)
    }

    pub fn ethanol_fuel_percentage(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0152"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    pub fn fuel_injection_timing(&mut self) -> Scalar {
        self.query(Command::new_pid(b"015D")).map_no_data(|r| {
            Scalar::new(
                (((256.0 * r.a_value()) + r.b_value()) / 128.0) - 210.0,
                Unit::Degrees,
            )
        })
    }

    pub fn commanded_evap_purge(&mut self) -> Scalar {
        self.query(Command::new_pid(b"012E"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    pub fn evap_system_vapor_pressure(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0132"))
            .map_no_data(|r| Scalar::new(((256.0 * r.a_value()) + r.b_value()) / 4.0, Unit::Pascal))
    }

    pub fn cylinder_fuel_rate(&mut self) -> Scalar {
        self.query(Command::new_pid(b"01A2")).map_no_data(|r| {
            Scalar::new(
                ((256.0 * r.a_value()) + r.b_value()) / 32.0,
                Unit::MiligramsPerStroke,
            )
        })
    }
}

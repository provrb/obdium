use std::fmt;

use crate::pid::engine::EngineType;
use crate::{cmd::Command, obd::OBD};
use sqlite::State;

#[derive(Debug)]
pub enum OBDStandard {
    Standard(&'static str),
}

impl OBDStandard {
    pub fn from_u8(num: u8) -> Self {
        match num {
            1u8 => Self::Standard("OBD-II as defined by CARB"),
            2u8 => Self::Standard("OBD as defined by the EPA"),
            3u8 => Self::Standard("OBD and OBD-II"),
            4u8 => Self::Standard("OBD-I"),
            5u8 => Self::Standard("Not OBD compliant"),
            6u8 => Self::Standard("EOBD"),
            7u8 => Self::Standard("EOBD and OBD-II"),
            8u8 => Self::Standard("EOBD and OBD"),
            9u8 => Self::Standard("EOBD, OBD and OBD-II"),
            10u8 => Self::Standard("JOBD"),
            11u8 => Self::Standard("JOBD and OBD-II"),
            12u8 => Self::Standard("JOBD and EOBD"),
            13u8 => Self::Standard("JOBD, EOBD and OBD-II"),
            // 14-16: Reserved
            17u8 => Self::Standard("Engine Manufacturer Diagnostics"),
            18u8 => Self::Standard("Engine Manufacturer Diagnostics Enhanced"),
            19u8 => Self::Standard("Heavy Duty On-Board Diagnostics (Child/Partial)"),
            20u8 => Self::Standard("Heavy Duty On-Board Diagnostics"),
            21u8 => Self::Standard("World Wide Harmonized OBD"),
            // 22: Reserved
            23u8 => Self::Standard("Heavy Duty Euro OBD Stage I without NOx control"),
            24u8 => Self::Standard("Heavy Duty Euro OBD Stage I with NOx control"),
            25u8 => Self::Standard("Heavy Duty Euro OBD Stage II without NOx control"),
            26u8 => Self::Standard("Heavy Duty Euro OBD Stage II with NOx control"),
            // 27: Reserved
            28u8 => Self::Standard("Brazil OBD Phase 1"),
            29u8 => Self::Standard("Brazil OBD Phase 2"),
            30u8 => Self::Standard("Korean OBD"),
            31u8 => Self::Standard("India OBD I"),
            32u8 => Self::Standard("India OBD II"),
            33u8 => Self::Standard("Heavy Duty Euro OBD Stage VI"),
            // 34-250: Reserved
            // 251-255: Unavailable
            _ => Self::Standard("No data"),
        }
    }
}

impl fmt::Display for OBDStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OBDStandard::Standard(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug)]
pub enum AuxiliaryInputStatus {
    InUse,
    NotInUse,
}

impl fmt::Display for AuxiliaryInputStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuxiliaryInputStatus::InUse => write!(f, "Active"),
            AuxiliaryInputStatus::NotInUse => write!(f, "Inactive"),
        }
    }
}

#[derive(Debug)]
pub enum TroubleCodeCategory {
    Powertrain,
    Chassis,
    Body,
    Network,
    Unknown,
}

impl TroubleCodeCategory {
    pub fn system_letter(&self) -> char {
        match self {
            TroubleCodeCategory::Powertrain => 'P',
            TroubleCodeCategory::Chassis => 'C',
            TroubleCodeCategory::Body => 'B',
            TroubleCodeCategory::Network => 'U',
            TroubleCodeCategory::Unknown => '?',
        }
    }
}

impl Default for TroubleCodeCategory {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for TroubleCodeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TroubleCodeCategory::Powertrain => write!(f, "Powertrain"),
            TroubleCodeCategory::Chassis => write!(f, "Chassis"),
            TroubleCodeCategory::Body => write!(f, "Body"),
            TroubleCodeCategory::Network => write!(f, "Network"),
            TroubleCodeCategory::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum MILStatus {
    On,
    Off,
}

#[derive(Debug, Default)]
pub struct TroubleCode {
    category: TroubleCodeCategory,
    dtc: String,
    description: String,
}

impl TroubleCode {
    pub fn new(category: TroubleCodeCategory, dtc: String) -> Self {
        let mut s = Self::default();
        s.category = category;
        s.dtc = dtc;
        s.set_description();

        s
    }

    pub fn set_description(&mut self) {
        self.description = "none".to_string(); // default

        // connect to trouble code data base
        let con = match sqlite::Connection::open("./code-descriptions.db") {
            Ok(con) => con,
            Err(err) => {
                println!("when connecting to codes database: {err}");
                return;
            }
        };

        // query description for 'dtc'
        let query = "SELECT desc FROM codes WHERE id = ?";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return;
            }
        };

        match statement.bind((1, self.dtc.as_str())) {
            Ok(_) => {}
            Err(err) => {
                println!("when binding dtc '{}' to query {query}: {err}", self.dtc);
                return;
            }
        };

        if let Ok(State::Row) = statement.next() {
            self.description = statement.read::<String, _>("desc").unwrap_or_default();
        }
    }
}

impl fmt::Display for TroubleCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Showing details for trouble code {}:", self.dtc)?;
        writeln!(f, "Trouble code: {}", self.dtc)?;
        writeln!(f, "System letter: {}", self.category.system_letter())?;
        writeln!(f, "Location: {}", self.category)?;
        writeln!(f, "Description: {}", self.description)?;
        write!(f, "Overview: {} is an issue related to {}. This code is from the {} component of the vehicle.", self.dtc, self.description, self.category)
    }
}

pub struct DiagnosisStatus {
    mil: MILStatus,
    num_trouble_codes: u32,
    trouble_codes: Vec<TroubleCode>,
    engine_type: EngineType,
}

impl OBD {
    pub fn get_mil_status(&mut self) -> MILStatus {
        let response = self.query(Command::new_pid(b"0101"));
        match response.a_value() as u32 & 0x80 {
            0 => MILStatus::Off,
            _ => MILStatus::On,
        }
    }

    // Resource heavy compared to other methods
    pub fn get_diagnosis_status(&mut self) -> DiagnosisStatus {
        DiagnosisStatus {
            mil: self.get_mil_status(),
            num_trouble_codes: self.get_num_trouble_codes(),
            trouble_codes: self.get_trouble_codes(),
            engine_type: self.get_engine_type(),
        }
    }

    pub fn warm_ups_since_codes_cleared(&mut self) -> u8 {
        let response = self.query(Command::new_pid(b"0130"));
        response.a_value() as u8
    }

    pub fn distance_traveled_since_codes_cleared(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0131"));
        (256.0 * response.a_value()) + response.b_value()
    }

    pub fn get_num_trouble_codes(&mut self) -> u32 {
        let response = self.query(Command::new_pid(b"0101"));
        response.a_value() as u32 & 0x7F
    }

    // Check on this. Might be broken when there are more than 3 DTC's
    pub fn get_trouble_codes(&mut self) -> Vec<TroubleCode> {
        let n_dtcs = self.get_num_trouble_codes();
        if n_dtcs <= 0 {
            // no trouble codes
            return Vec::new();
        }

        let response = self.query(Command::new_svc(b"03"));
        // println!("dtc response: {:#?}", response);
        let sanitized = response
            .full_response()
            .unwrap_or_default()
            .replace(" ", "")
            .split("43")
            .collect::<Vec<_>>()
            .join("");

        println!("dbg: dtc response from ecu: {sanitized}");

        let mut codes = Vec::new();

        for chunk in sanitized.as_bytes().chunks(4) {
            if chunk.len() != 4 {
                break;
            }

            let as_string = String::from_utf8_lossy(chunk);
            let a = as_string.chars().nth(0).unwrap_or('\0');
            let b = as_string.chars().nth(1).unwrap_or('\0');
            let c = as_string.chars().nth(2).unwrap_or('\0');
            let d = as_string.chars().nth(3).unwrap_or('\0');
            let left = u8::from_str_radix(&format!("{}{}", a, b), 16).unwrap_or_default();
            let right = u8::from_str_radix(&format!("{}{}", c, d), 16).unwrap_or_default();

            if left == 0x00 && right == 0x00 || (chunk == b"0000") {
                break;
            }

            //println!("\nchunk: {:?}, {:?}. a: {a}, b: {b}, c: {c}, d: {d}", as_string, chunk);
            let bit_7 = (left & 0b1000_0000) >> 7;
            let bit_6 = (left & 0b0100_0000) >> 6;
            let bit_5 = (left & 0b0010_0000) >> 5;
            let bit_4 = (left & 0b0001_0000) >> 4;
            let c2 = (bit_5 << 1) | bit_4;
            let new_left = left & 0b00001111;
            // println!("left: {left:b}, right: {right:b}");
            // println!("bit 7: {bit_7:b}, bit 6: {bit_6:b}, bit 5: {bit_5:b}, bit 4: {bit_4:b}");
            // println!("C2: {c2:b}");

            let category = match (bit_7, bit_6) {
                (0, 0) => TroubleCodeCategory::Powertrain,
                (0, 1) => TroubleCodeCategory::Chassis,
                (1, 0) => TroubleCodeCategory::Body,
                (1, 1) => TroubleCodeCategory::Network,
                _ => TroubleCodeCategory::Unknown,
            };

            let dtc_code = format!(
                "{}{:01X}{:01X}{:02X}",
                category.system_letter(),
                c2,
                new_left,
                right
            );

            // println!("left: {}", left);
            // println!("transformed left: {}", new_left);
            // println!("right: {}", right);

            // println!("C2: {}", c2);

            // println!("- DTC: {}", dtc_code);

            // println!(
            //     "left: {:02X}\nright: {:02X}\nbit 7: {}\nbit 6: {}\nbit 5: {}\nbit 4: {}\ncategory: {:?}\nc2: {}\ndtc_code: {}",
            //     left, right, bit_7, bit_6, bit_5, bit_4, category, c2, dtc_code
            // );

            codes.push(TroubleCode::new(category, dtc_code));
        }

        codes
    }

    pub fn obd_standards(&mut self) -> OBDStandard {
        let response = self.query(Command::new_pid(b"011C"));
        OBDStandard::from_u8(response.a_value() as u8)
    }

    pub fn aux_input_status(&mut self) -> AuxiliaryInputStatus {
        let response = self.query(Command::new_pid(b"011E"));
        let in_use = (response.a_value() as u32 & 1) != 0;

        if in_use {
            AuxiliaryInputStatus::InUse
        } else {
            AuxiliaryInputStatus::NotInUse
        }
    }

    pub fn distance_traveled_with_mil(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0121"));
        (256.0 * response.a_value()) + response.b_value()
    }

    pub fn time_run_with_mil(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"014D"));
        (256.0 * response.a_value()) + response.b_value()
    }

    pub fn control_module_voltage(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0142"));
        ((256.0 * response.a_value()) + response.b_value()) / 1000.0
    }

    pub fn time_since_codes_cleared(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"014E"));
        (256.0 * response.a_value()) + response.b_value()
    }
}

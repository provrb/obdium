use crate::pid::engine::EngineType;
use crate::response::{self, Response};
use crate::{cmd::Command, obd::OBD};

#[derive(Debug)]
pub enum OBDStandard {
    Standard(&'static str),
}

#[derive(Debug)]
pub enum AuxiliaryInputStatus {
    InUse,
    NotInUse,
}

#[derive(Debug)]
enum TroubleCodeCategory {
    Powertrain,
    Chassis,
    Body,
    Network,
    Unknown,
}

impl TroubleCodeCategory {
    pub fn catagory_char(&self) -> char {
        match self {
            TroubleCodeCategory::Powertrain => 'P',
            TroubleCodeCategory::Chassis => 'C',
            TroubleCodeCategory::Body => 'B',
            TroubleCodeCategory::Network => 'U',
            TroubleCodeCategory::Unknown => '?',
        }
    }
}

enum MILStatus {
    On,
    Off,
}

#[derive(Debug)]
pub struct TroubleCode {
    category: TroubleCodeCategory,
    dtc: String,
    description: String,
}

pub struct DiagnosisStatus {
    mil: MILStatus,
    num_trouble_codes: u16,
    trouble_codes: Vec<TroubleCode>,
    engine_type: EngineType,
}

impl OBD {
    pub fn get_mil_status(&mut self) -> MILStatus {
        let response = self.query(Command::new_pid(b"0101")).unwrap_or_default();
        match response.a_value() as u32 & 0x80 {
            0 => MILStatus::Off,
            _ => MILStatus::On,
        }
    }

    // Resource heavy compared to other methods
    pub fn get_diagnosis_status(&mut self) -> DiagnosisStatus {
        let mil = self.get_mil_status();
        let n_dtcs = self.get_num_trouble_codes();
        let engine_type = self.get_engine_type();
        let trouble_codes = self.get_trouble_codes();

        todo!()
    }

    pub fn get_num_trouble_codes(&mut self) -> u32 {
        let response = self.query(Command::new_pid(b"0101")).unwrap_or_default();
        response.a_value() as u32 & 0x7F
    }

    pub fn get_trouble_codes(&mut self) -> Vec<TroubleCode> {
        // let n_dtcs = self.get_num_trouble_codes();
        // if n_dtcs <= 0 { // no trouble codes
        //     return Vec::new();
        // }

        // By passing "03  " we are bypassing the new_pid requirement for a [u8; 4] when we are only giving a [u8; 2].
        // ELM327 ignores all space characters, so this will be parsed normally as "03" instead of "03  "
        let response = self.query(Command::new_pid(b"03  ")).unwrap_or_default();   
        println!("dtc response: {:#?}",response);
        // TODO: FIX
        let mut codes = Vec::new();
        for chunk in response
            .full_response()
            .unwrap_or_default()
            .replace(" ", "")
            .as_bytes()[2..]
            .chunks(2)
        {
            if chunk.len() != 2 {
                break;
            }
            
            // println!("chunk: {:?}, {:?}", String::from_utf8_lossy(chunk), chunk);

            let left = chunk[0];
            let right = chunk[1];

            if left == 0x00 || right == 0x00 || (chunk == b"00") {
                break;
            }

            let bit_7 = (left & 0b10000000) >> 7;
            let bit_6 = (left & 0b01000000) >> 6;
            let c2 = (left & 0b00110000) >> 4;
            let new_left = left & 0b00001111;

            let category = match (bit_7, bit_6) {
                (0, 0) => TroubleCodeCategory::Powertrain,
                (0, 1) => TroubleCodeCategory::Chassis,
                (1, 0) => TroubleCodeCategory::Body,
                (1, 1) => TroubleCodeCategory::Network,
                _ => TroubleCodeCategory::Unknown,
            };

            let dtc_code = format!(
                "{}{:01X}{:01X}{:02X}",
                category.catagory_char(),
                c2,
                new_left,
                right
            );

            // println!(
            //     "left: {:02X}\nright: {:02X}\nbit 7: {}\nbit 6: {}\nbit 5: {}\nbit 4: {}\ncategory: {:?}\nc2: {}\ndtc_code: {}",
            //     left, right, bit_7, bit_6, bit_5, bit_4, category, c2, dtc_code
            // );

            codes.push(TroubleCode {
                dtc: dtc_code,
                category: category,
                description: "placeholder".to_string(),
            });
        }

        println!("{:?}", codes);
        codes
    }

    pub fn obd_standards(&mut self) -> OBDStandard {
        let response = self.query(Command::new_pid(b"011C")).unwrap_or_default();
        match response.a_value() {
            1f32 => OBDStandard::Standard("OBD-II as defined by CARB"),
            2f32 => OBDStandard::Standard("OBD as defined by the EPA"),
            3f32 => OBDStandard::Standard("OBD and OBD-II"),
            4f32 => OBDStandard::Standard("OBD-I"),
            5f32 => OBDStandard::Standard("Not OBD compliant"),
            6f32 => OBDStandard::Standard("EOBD"),
            7f32 => OBDStandard::Standard("EOBD and OBD-II"),
            8f32 => OBDStandard::Standard("EOBD and OBD"),
            9f32 => OBDStandard::Standard("EOBD, OBD and OBD-II"),
            10f32 => OBDStandard::Standard("JOBD"),
            11f32 => OBDStandard::Standard("JOBD and OBD-II"),
            12f32 => OBDStandard::Standard("JOBD and EOBD"),
            13f32 => OBDStandard::Standard("JOBD, EOBD and OBD-II"),
            // 14-16: Reserved
            17f32 => OBDStandard::Standard("Engine Manufacturer Diagnostics"),
            18f32 => OBDStandard::Standard("Engine Manufacturer Diagnostics Enhanced"),
            19f32 => OBDStandard::Standard("Heavy Duty On-Board Diagnostics (Child/Partial)"),
            20f32 => OBDStandard::Standard("Heavy Duty On-Board Diagnostics"),
            21f32 => OBDStandard::Standard("World Wide Harmonized OBD"),
            // 22: Reserved
            23f32 => OBDStandard::Standard("Heavy Duty Euro OBD Stage I without NOx control"),
            24f32 => OBDStandard::Standard("Heavy Duty Euro OBD Stage I with NOx control"),
            25f32 => OBDStandard::Standard("Heavy Duty Euro OBD Stage II without NOx control"),
            26f32 => OBDStandard::Standard("Heavy Duty Euro OBD Stage II with NOx control"),
            // 27: Reserved
            28f32 => OBDStandard::Standard("Brazil OBD Phase 1"),
            29f32 => OBDStandard::Standard("Brazil OBD Phase 2"),
            30f32 => OBDStandard::Standard("Korean OBD"),
            31f32 => OBDStandard::Standard("India OBD I"),
            32f32 => OBDStandard::Standard("India OBD II"),
            33f32 => OBDStandard::Standard("Heavy Duty Euro OBD Stage VI"),
            // 34-250: Reserved
            // 251-255: Unavailable
            _ => OBDStandard::Standard("No data"),
        }
    }

    pub fn aux_input_status(&mut self) -> AuxiliaryInputStatus {
        let response = self.query(Command::new_pid(b"011C")).unwrap_or_default();
        let in_use = (response.a_value() as u32 & 1) != 0;

        if in_use {
            AuxiliaryInputStatus::InUse
        } else {
            AuxiliaryInputStatus::NotInUse
        }
    }

    pub fn distance_traveled_with_mil(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0121")).unwrap_or_default();
        (256.0 * response.a_value()) + response.b_value()
    }

    pub fn control_module_voltage(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0142")).unwrap_or_default();
        ((256.0 * response.a_value()) + response.b_value()) / 1000.0
    }

    pub fn time_since_codes_cleared(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"014E")).unwrap_or_default();
        (256.0 * response.a_value()) + response.b_value()
    }
}

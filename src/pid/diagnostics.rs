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

impl OBD {
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

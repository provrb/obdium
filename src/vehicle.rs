use std::{error::Error, io::ErrorKind};

pub struct VIN {
    country_of_origin: String,
    vehicle_manufacturer: String,
    vehicle_type: String,

}

impl VIN {
    pub fn new(vin: String) -> Option<Self> {
        if vin.len() != 17 {
            println!("invalid vin length {}. must be 17 characters", vin.len());
            return None;
        }

        if !VIN::checksum(&vin) {
            println!("invalid vin. check sum does not match.");
            return None;
        }

        todo!()
    }

    pub fn checksum(vin: &str) -> bool {
        todo!()
    }
}
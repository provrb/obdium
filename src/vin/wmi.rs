use sqlite::State;

use crate::vin::parser::{VinError, VIN};

impl VIN {
    pub fn get_wmi(&self) -> Result<String, VinError> {
        if self.vin.len() < 3 {
            return Err(VinError::InvalidVinLength);
        }

        let wmi = &self.vin[..3];
        let last = match wmi.chars().last() {
            Some(ch) => ch,
            None => return Err(VinError::WMIError),
        };

        // ISO 3780's WMI extended form
        if last == '9' && self.vin.len() >= 14 {
            let extended_wmi = format!("{}{}", wmi, &self.vin[11..14]);
            return Ok(extended_wmi);
        }

        Ok(wmi.to_string())
    }

    pub fn get_wmi_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;

        let query = "SELECT Id FROM Wmi WHERE Wmi = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("Id")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_vehicle_type_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;

        let query = "SELECT VehicleTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("VehicleTypeId")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Ok(-1),
        }
    }

    pub fn get_truck_type_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;

        let query = "SELECT TruckTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("TruckTypeId")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Ok(-1),
        }
    }

    pub fn get_make_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;
        let query = "SELECT MakeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("MakeId")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_manufacturer_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;
        let query = "SELECT ManufacturerId FROM Wmi WHERE Wmi = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("ManufacturerId")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }
}

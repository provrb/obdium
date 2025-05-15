use std::str::FromStr;

use sqlite::State;

use crate::vin::parser::{VinError, VIN};

impl VIN {
    pub(crate) fn query_wmi<T: FromStr>(&self, column_name: &str) 
    -> Result<T, VinError> 
    where 
        T::Err: std::fmt::Debug,
    {
        let wmi = self.get_wmi().as_str();
        let con = self.vpic_connection()?;

        let query = format!("SELECT {} FROM Wmi WHERE Wmi = ?", column_name);
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError("query_wmi"))?;

        statement.bind((1, wmi)).map_err(|_| VinError::VPICQueryError("query_wmi"))?;

        if let Ok(State::Row) = statement.next() {
            let data = statement.read::<String, _>(column_name).map_err(|_| VinError::VPICQueryError("query_wmi"))?;
            return  data.parse::<T>().map_err(|_| VinError::ParseError)
        }

        Err(VinError::NoResultsFound("query_wmi"))
    }

    pub fn get_wmi(&self) -> &String {
        self.wmi.get_or_init(||{
            let vin = self.get_vin();
            let wmi = &vin[..3];
            let last = match wmi.chars().last() {
                Some(ch) => ch,
                None => '\0',
            };
    
            // ISO 3780's WMI extended form
            if last == '9' {
                let extended_wmi = format!("{}{}", wmi, &vin[11..14]);
                return extended_wmi;
            }
    
            wmi.to_string()
        })
    }

    pub fn get_wmi_id(&self) -> Result<&i64, VinError> {
        let res = self.wmi_schema_id.get_or_init(|| {
            match self.query_wmi("Id") {
                Ok(res) => res,
                Err(_) => -1,
            }
        });

        if *res == -1 {
            Err(VinError::NoResultsFound("SELECT Id FROM Wmi WHERE Wmi = ?"))
        } else {
            Ok(res)
        }
    }

    pub fn get_vehicle_type_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;

        let query = "SELECT VehicleTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("VehicleTypeId")
                .map_err(|_| VinError::VPICQueryError(query))?),
            _ => Ok(-1),
        }
    }

    pub fn get_truck_type_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;

        let query = "SELECT TruckTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("TruckTypeId")
                .map_err(|_| VinError::VPICQueryError(query))?),
            _ => Ok(-1),
        }
    }

    pub fn get_make_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;
        let query = "SELECT MakeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("MakeId")
                .map_err(|_| VinError::VPICQueryError(query))?),
            _ => Err(VinError::NoResultsFound(query)),
        }
    }

    pub fn get_manufacturer_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = self.vpic_connection()?;
        let query = "SELECT ManufacturerId FROM Wmi WHERE Wmi = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((1, wmi))
            .map_err(|_| VinError::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("ManufacturerId")
                .map_err(|_| VinError::VPICQueryError(query))?),
            _ => Err(VinError::NoResultsFound(query)),
        }
    }
}

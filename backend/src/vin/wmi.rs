use sqlite::State;
use std::str::FromStr;

use crate::vin::{Error, VIN};

impl VIN {
    pub(crate) fn query_wmi<T: FromStr>(&self, column_name: &str) -> Result<T, Error>
    where
        T::Err: std::fmt::Debug,
    {
        let wmi = self.get_wmi();
        let con = self.vpic_connection()?;

        let query = format!("SELECT {} FROM Wmi WHERE Wmi = ?", column_name);
        let mut statement = con
            .prepare(query)
            .map_err(|_| Error::VPICQueryError("query_wmi"))?;

        statement
            .bind((1, wmi))
            .map_err(|_| Error::VPICQueryError("query_wmi"))?;

        if let Ok(State::Row) = statement.next() {
            let data = statement
                .read::<String, _>(column_name)
                .map_err(|_| Error::VPICQueryError("query_wmi"))?;
            return data.parse::<T>().map_err(|_| Error::ParseError);
        }

        Err(Error::NoResultsFound("query_wmi"))
    }

    pub fn get_wmi(&self) -> &str {
        self.wmi.get_or_init(|| {
            let vin = self.get_vin();
            let wmi = &vin[..3];
            let last = wmi.chars().last().unwrap_or('\0');

            // ISO 3780's WMI extended form
            if last == '9' {
                let extended_wmi = format!("{}{}", wmi, &vin[11..14]);
                return extended_wmi;
            }

            wmi.to_string()
        })
    }

    pub fn get_wmi_id(&self) -> Result<i64, Error> {
        let res = *self
            .wmi_id
            .get_or_init(|| self.query_wmi("Id").unwrap_or(-1));

        if res == -1 {
            Err(Error::NoResultsFound("SELECT Id FROM Wmi WHERE Wmi = ?"))
        } else {
            Ok(res)
        }
    }

    pub fn get_vehicle_type_id(&self) -> Result<i64, Error> {
        let wmi = self.get_wmi();
        let con = self.vpic_connection()?;

        let query = "SELECT VehicleTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| Error::VPICQueryError(query))?;

        statement
            .bind((1, wmi))
            .map_err(|_| Error::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("VehicleTypeId")
                .map_err(|_| Error::VPICQueryError(query))?),
            _ => Ok(-1),
        }
    }

    pub fn get_truck_type_id(&self) -> Result<i64, Error> {
        let wmi = self.get_wmi();
        let con = self.vpic_connection()?;

        let query = "SELECT TruckTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| Error::VPICQueryError(query))?;

        statement
            .bind((1, wmi))
            .map_err(|_| Error::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("TruckTypeId")
                .map_err(|_| Error::VPICQueryError(query))?),
            _ => Ok(-1),
        }
    }

    pub fn get_make_id(&self) -> Result<i64, Error> {
        let wmi_id = self.get_wmi_id()?;
        let con = self.vpic_connection()?;
        let query = "SELECT MakeId FROM Wmi_Make WHERE WmiId = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| Error::VPICQueryError(query))?;

        statement
            .bind((1, wmi_id))
            .map_err(|_| Error::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("MakeId")
                .map_err(|_| Error::VPICQueryError(query))?),
            _ => Err(Error::NoResultsFound(query)),
        }
    }

    pub fn get_manufacturer_id(&self) -> Result<i64, Error> {
        let wmi = self.get_wmi();
        let con = self.vpic_connection()?;
        let query = "SELECT ManufacturerId FROM Wmi WHERE Wmi = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| Error::VPICQueryError(query))?;

        statement
            .bind((1, wmi))
            .map_err(|_| Error::VPICQueryError(query))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<i64, _>("ManufacturerId")
                .map_err(|_| Error::VPICQueryError(query))?),
            _ => Err(Error::NoResultsFound(query)),
        }
    }
}

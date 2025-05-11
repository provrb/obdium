use chrono::{self, Datelike};
use regex::Regex;
use sqlite::{Connection, State};

const VPIC_DB_PATH: &'static str = "./data/vpic.sqlite";

#[derive(Debug)]
pub enum VinError {
    VPICConnectFailed,
    VPICNoConnection,
    VPICQueryError,

    InvalidVinLength,
    WMIError,
    ModelYearError,

    BadKey,
}

pub struct VIN {
    vpic_db_con: Option<sqlite::Connection>,
    vin: String,
}

impl VIN {
    pub fn new(vin: String) -> Self {
        let mut _vin = Self {
            vin,
            vpic_db_con: None,
        };

        if _vin.connect_to_vpic_db().is_err() {
            println!("Error connecting to VPIC database.");
        }

        _vin
    }

    fn connect_to_vpic_db(&mut self) -> Result<&Connection, VinError> {
        if self.vpic_db_con.is_none() {
            let conn = Connection::open(VPIC_DB_PATH)
                .map_err(|_| VinError::VPICConnectFailed)
                .ok();
            self.vpic_db_con = conn;
        }
        self.vpic_db_con.as_ref().ok_or(VinError::VPICConnectFailed)
    }

    // This code is not mine.
    // fn pattern_to_regex(pattern: &str) -> Result<Regex, regex::Error> {
    //     let mut regex = String::with_capacity(pattern.len() + 3); // Pre-allocate enough space
    //     regex.push('^'); // Start of regex

    //     let mut chars = pattern.chars().peekable();

    //     while let Some(ch) = chars.next() {
    //         match ch {
    //             '*' => regex.push('.'), // '*' becomes '.'
    //             '[' => {
    //                 regex.push('['); // Start of character class
    //                 while let Some(inner_ch) = chars.next() {
    //                     regex.push(inner_ch);
    //                     if inner_ch == ']' {
    //                         break; // End of character class
    //                     }
    //                 }
    //             }
    //             '.' | '+' | '(' | ')' | '|' | '^' | '$' | '{' | '}' | '\\' => {
    //                 regex.push('\\'); // Escape special characters
    //                 regex.push(ch);
    //             }
    //             _ => regex.push(ch), // Non-special characters
    //         }
    //     }

    //     regex.push_str(".*$"); // End of regex
    //     Regex::new(&regex)
    // }

    // pub fn pattern_matches(input: &str, pattern: &str) -> bool {
    //     match Self::pattern_to_regex(pattern) {
    //         Ok(regex) => regex.is_match(input),
    //         Err(_) => false,
    //     }
    // }

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

    pub fn get_vehicle_type_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => {
                println!("get_vehicle_type_id(): no database connection established. quitting.");
                return Err(VinError::VPICNoConnection);
            }
        };

        let query = "SELECT VehicleTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        match statement.bind((1, wmi)) {
            Ok(_) => {}
            Err(err) => {
                println!("when binding wmi '{}' to query {query}: {err}", wmi);
                return Err(VinError::VPICQueryError);
            }
        };

        if let Ok(State::Row) = statement.next() {
            return Ok(statement
                .read::<i64, _>("VehicleTypeId")
                .map_err(|_| VinError::VPICQueryError)?);
        }

        return Err(VinError::VPICQueryError);
    }

    pub fn get_truck_type_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => {
                println!("get_truck_type_id(): no database connection established. quitting.");
                return Err(VinError::VPICNoConnection);
            }
        };

        let query = "SELECT TruckTypeId FROM Wmi WHERE Wmi = ?";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        match statement.bind((1, wmi)) {
            Ok(_) => {}
            Err(err) => {
                println!("when binding wmi '{}' to query {query}: {err}", wmi);
                return Err(VinError::VPICQueryError);
            }
        };

        if let Ok(State::Row) = statement.next() {
            return Ok(statement
                .read::<i64, _>("TruckTypeId")
                .map_err(|_| VinError::VPICQueryError)?);
        }

        return Err(VinError::VPICQueryError);
    }

    // Based off of NHTSA's ModelYear2 MS SQL Server procedure.
    pub fn get_model_year(&self) -> Result<i32, VinError> {
        if self.vin.len() < 10 {
            return Err(VinError::InvalidVinLength);
        }

        let pos10 = match self.vin.chars().nth(9) {
            Some(ch) => ch,
            None => return Err(VinError::ModelYearError),
        };

        let mut model_year = match pos10 {
            'A'..'H' => 2010 + (pos10 as i32) - ('A' as i32),
            'J'..'N' => 2010 + (pos10 as i32) - ('A' as i32) - 1,
            'P' => 2023,
            'R'..'T' => 2010 + (pos10 as i32) - ('A' as i32) - 3,
            'V'..'Y' => 2010 + (pos10 as i32) - ('A' as i32) - 4,
            '1'..'9' => 2031 + (pos10 as i32) - ('1' as i32),
            _ => unreachable!(),
        };

        let wmi = self.get_wmi()?;
        let vehicle_type_id = self.get_vehicle_type_id(&wmi)?;
        let truck_type_id = self.get_truck_type_id(&wmi)?;
        let mut car_lt = false;

        if (2..=7).contains(&vehicle_type_id) || (vehicle_type_id == 3 && truck_type_id == 1) {
            car_lt = true;
        }

        let pos7 = match self.vin.chars().nth(6) {
            Some(ch) => ch,
            None => return Err(VinError::ModelYearError),
        };

        if car_lt {
            match pos7 {
                '0'..'9' => model_year -= 30,
                _ => (),
            }
        }

        if model_year > chrono::Utc::now().year() + 1 {
            model_year -= 30;
        }

        Ok(model_year)
    }

    pub fn get_wmi_id(&self, wmi: &str) -> Result<i64, VinError> {
        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => {
                println!("get_wmi_id(): no database connection established. quitting.");
                return Err(VinError::VPICNoConnection);
            }
        };

        let query = "SELECT Id FROM Wmi WHERE Wmi = ?";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        match statement.bind((1, wmi)) {
            Ok(_) => {}
            Err(err) => {
                println!("when binding wmi '{}' to query {query}: {err}", wmi);
                return Err(VinError::VPICQueryError);
            }
        };

        if let Ok(State::Row) = statement.next() {
            return Ok(statement
                .read::<i64, _>("Id")
                .map_err(|_| VinError::VPICQueryError)?);
        }

        return Err(VinError::VPICQueryError);
    }

    pub fn get_schema_id(&self, wmi_id: i64, model_year: i32) -> Result<i64, VinError> {
        let key = self.as_key();
        if key.is_empty() {
            return Err(VinError::BadKey);
        }

        // Look for engine model.
        // Element Id for engine model is 18.
        // Check if pattern matches keys
        //      They match:
        //          Check Wmi_VinSchema
        //          - Ensure model year is in range 'YearFrom' - 'YearTo'
        //          - Ensure WmiId == wmi_id
        //          If these conditions are met, this is the VinSchemaId
        todo!()
    }

    pub fn as_key(&self) -> String {
        if self.vin.len() < 17 {
            return String::new();
        }

        let mut vin_clone = self.vin.clone()[3..].to_string(); // without wmi
        let mut vin_bytes = vin_clone.into_bytes();
        vin_bytes[5] = b'|';

        vin_clone = String::from_utf8(vin_bytes).unwrap_or_default();

        vin_clone
    }
}

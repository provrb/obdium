use chrono::Datelike;
use regex::Regex;
use sqlite::{Connection, State};

use crate::pid::{
    engine::ValveTrainDesign,
    fuel::{FuelDeliveryType, FuelType},
};

use crate::vin::element_ids::ElementId;

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
    NoResultsFound,
}

pub struct VIN {
    vpic_db_con: Option<sqlite::Connection>,
    vin: String,
}

impl Default for VIN {
    fn default() -> Self {
        Self {
            vpic_db_con: None,
            vin: String::new(),
        }
    }
}

impl PartialEq for VIN {
    fn eq(&self, other: &Self) -> bool {
        self.vin == other.vin
    }

    fn ne(&self, other: &Self) -> bool {
        self.vin != other.vin
    }
}

impl VIN {
    pub fn new(vin: String) -> Self {
        let mut _vin = Self {
            vin,
            ..Default::default()
        };

        if _vin.connect_to_vpic_db().is_err() {
            println!("Error connecting to VPIC database.");
        }

        _vin
    }

    pub fn test_database_connection(&self) -> bool {
        self.vpic_db_con.is_some()
    }

    fn connect_to_vpic_db(&mut self) -> Result<&Connection, VinError> {
        if self.vpic_db_con.is_none() {
            let conn = Connection::open(VPIC_DB_PATH)
                .map_err(|_| VinError::VPICConnectFailed);
            self.vpic_db_con = conn.ok();
        }
        self.vpic_db_con.as_ref().ok_or(VinError::VPICConnectFailed)
    }

    pub fn matches_vin_pattern(pattern: &str, input: &str) -> bool {
        // Convert our custom pattern to proper regex
        let regex_pattern = VIN::convert_to_regex(pattern);
        println!("Converted regex: {}", regex_pattern);

        // Create regex (case insensitive)
        match Regex::new(&regex_pattern) {
            Ok(re) => {
                let result = re.is_match(input);
                println!(
                    "Matching '{}' against '{}' => {}",
                    input, regex_pattern, result
                );
                result
            }
            Err(e) => {
                println!("Regex error: {}", e);
                false
            }
        }
    }

    pub fn convert_to_regex(pattern: &str) -> String {
        let mut regex = String::new();
        regex.push_str(r"(?i)^"); // Case insensitive, start of string

        let mut in_char_class = false;
        let mut char_class = String::new();

        for c in pattern.chars() {
            if in_char_class {
                char_class.push(c);
                if c == ']' {
                    // Handle character classes like [A-H1-3]
                    regex.push_str(&char_class);
                    in_char_class = false;
                    char_class.clear();
                }
            } else {
                match c {
                    '*' => regex.push_str(".*"),
                    '?' => regex.push('.'),
                    '[' => {
                        in_char_class = true;
                        char_class.push(c);
                    }
                    _ => {
                        // Escape special regex characters
                        if ".\\+*?^$()[]{}|".contains(c) {
                            regex.push('\\');
                        }
                        regex.push(c);
                    }
                }
            }
        }

        regex.push('$'); // End of string
        regex
    }

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

        return Ok(-1);
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

    fn match_pattern(key: &str, pattern: &str) -> bool {
        let mut key_chars = key.chars().peekable();
        let mut pat_chars = pattern.chars().peekable();

        while let Some(pc) = pat_chars.next() {
            match pc {
                '_' => {
                    if key_chars.next().is_none() {
                        return false;
                    }
                }
                '%' => {
                    return true;
                }
                '[' => {
                    let mut class = Vec::new();
                    let mut negated = false;
                    if let Some(&'^') = pat_chars.peek() {
                        pat_chars.next(); // consume ^
                        negated = true;
                    }

                    while let Some(c) = pat_chars.next() {
                        if c == ']' {
                            break;
                        }

                        if let Some(&'-') = pat_chars.peek() {
                            pat_chars.next(); // consume '-'
                            if let Some(end) = pat_chars.next() {
                                for ch in c..=end {
                                    class.push(ch);
                                }
                            }
                        } else {
                            class.push(c);
                        }
                    }

                    match key_chars.next() {
                        Some(kc) => {
                            let contains = class.contains(&kc);
                            if (contains && negated) || (!contains && !negated) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                c => match key_chars.next() {
                    Some(kc) if kc == c => {}
                    _ => return false,
                },
            }
        }

        key_chars.next().is_none()
    }

    pub fn get_schema_id(&self, wmi_id: i64, model_year: i64) -> Result<i64, VinError> {
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

        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => {
                println!("get_schema_id(): no database connection established. quitting.");
                return Err(VinError::VPICNoConnection);
            }
        };

        let query = "SELECT * FROM Pattern WHERE ElementId = 18";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        let mut matched_schema_ids = Vec::new();

        while let Ok(State::Row) = statement.next() {
            // this is where you would check the pattern from Pattern matches the key.
            let pattern = statement
                .read::<String, _>("Keys")
                .map_err(|_| VinError::VPICQueryError)?;

            let pattern_sql_like = pattern.replace("*", "_") + "%"; // simulate MSSQL logic

            if VIN::match_pattern(&key, &pattern_sql_like) {
                matched_schema_ids.push(
                    statement
                        .read::<i64, _>("VinSchemaId")
                        .map_err(|_| VinError::VPICQueryError)?,
                );
            }
        }

        let query = "SELECT * FROM Wmi_VinSchema WHERE WmiId = ? and ? BETWEEN YearFrom and IFNULL(YearTo, 2999)";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        statement
            .bind((1, wmi_id))
            .map_err(|_| VinError::VPICQueryError)?;
        statement
            .bind((2, model_year))
            .map_err(|_| VinError::VPICQueryError)?;

        while let Ok(State::Row) = statement.next() {
            let schema_id = statement
                .read::<i64, _>("VinSchemaId")
                .map_err(|_| VinError::VPICQueryError)?;

            if matched_schema_ids.contains(&schema_id) {
                return Ok(schema_id);
            }
        }

        Err(VinError::NoResultsFound)
    }

    /// Returns a row from Pattern table
    /// that matches conditions:
    /// 1. Schema ID
    /// 2. Element Id
    /// 3. Key matches pattern 'Keys'
    fn query_pattern(
        &self,
        schema_id: i64,
        element_id: ElementId,
        key: &str,
    ) -> Result<(i64, i64, String, i64, String), VinError> {
        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => {
                println!("query_pattern(): no database connection established. quitting.");
                return Err(VinError::VPICNoConnection);
            }
        };

        let query = "SELECT * FROM Pattern WHERE VinSchemaId = ? and ElementId = ?";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        statement
            .bind((1, schema_id))
            .map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((2, element_id.as_i64()))
            .map_err(|_| VinError::VPICQueryError)?;

        while let Ok(State::Row) = statement.next() {
            let pattern = statement
                .read::<String, _>("Keys")
                .map_err(|_| VinError::VPICQueryError)?;

            let pattern_sql_like = pattern.replace("*", "_") + "%"; // simulate MSSQL logic

            if VIN::match_pattern(&key, &pattern_sql_like) {
                let pattern_id = statement
                    .read::<i64, _>("Id")
                    .map_err(|_| VinError::VPICQueryError)?;

                let attribute_id = statement
                    .read::<String, _>("AttributeId")
                    .map_err(|_| VinError::VPICQueryError)?;

                return Ok((
                    pattern_id,
                    schema_id,
                    pattern,
                    element_id.as_i64(),
                    attribute_id,
                ));
            }
        }

        return Err(VinError::NoResultsFound);
    }

    pub fn get_engine_model(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineModel, &key)?;

        Ok(data.4)
    }

    pub fn get_cylinder_count(&self, schema_id: i64) -> Result<i64, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineCylinderCount, &key)?;

        data.4.parse().map_err(|_| VinError::VPICQueryError)
    }

    pub fn get_engine_displacement(&self, schema_id: i64) -> Result<f64, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineDisplacement, &key)?;

        data.4.parse().map_err(|_| VinError::VPICQueryError)
    }

    pub fn get_fuel_type(&self, schema_id: i64) -> Result<FuelType, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::FuelType, &key)?;
        let fuel_type: i64 = data.4.parse().map_err(|_| VinError::VPICQueryError)?;

        // NHTSA fuel type bindings is different
        // than the OBD fuel type bindings so we cannot
        // do FuelType::from_u8
        match fuel_type {
            1 => Ok(FuelType::Type("Diesel")),
            2 => Ok(FuelType::Type("Electric")),
            4 => Ok(FuelType::Type("Gasoline")),
            6 => Ok(FuelType::Type("Compressed Natural Gas")),
            7 => Ok(FuelType::Type("Liquefied Natural Gas")),
            8 => Ok(FuelType::Type("Compressed Hydrogen/Hydrogen")),
            9 => Ok(FuelType::Type("Liquefied Petroleum Gas")),
            10 => Ok(FuelType::Type("Ethanol")),
            11 => Ok(FuelType::Type("Neat Ethanol")),
            13 => Ok(FuelType::Type("Methanol")),
            14 => Ok(FuelType::Type("Neat Methanol")),
            15 => Ok(FuelType::Type("Flexible Fuel Vehicle")),
            17 => Ok(FuelType::Type("Natural Gas")),
            18 => Ok(FuelType::Type("Fuel Cell")),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_valve_train_design(&self, schema_id: i64) -> Result<ValveTrainDesign, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::ValveTrainDesign, &key)?;
        let id: u8 = data.4.parse().map_err(|_| VinError::VPICQueryError)?;

        Ok(ValveTrainDesign::from_u8(id))
    }

    pub fn get_fuel_delivery_type(&self, schema_id: i64) -> Result<FuelDeliveryType, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::FuelDeliveryType, &key)?;
        let id: u8 = data.4.parse().map_err(|_| VinError::VPICQueryError)?;

        Ok(FuelDeliveryType::from_u8(id))
    }

    pub fn has_turbo(&self, schema_id: i64) -> Result<bool, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::HasTurbo, &key)?;
        let turbo: u8 = data.4.parse().map_err(|_| VinError::VPICQueryError)?;

        Ok(turbo == 1)
    }

    pub fn get_engine_manufacturer(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineManufacturer, &key)?;

        Ok(data.4)
    }

    pub fn get_vehicle_door_count(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::VehicleDoorCount, &key)?;

        data.4.parse().map_err(|_| VinError::VPICQueryError)
    }

    pub fn get_vehicle_model(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::VehicleModel, &key)?;
        let model_id: i64 = data.4.parse().map_err(|_| VinError::VPICQueryError)?;

        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => return Err(VinError::VPICNoConnection),
        };

        let query = "SELECT Name FROM Model WHERE Id = ?";

        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        statement
            .bind((1, model_id))
            .map_err(|_| VinError::VPICQueryError)?;

        if let Ok(State::Row) = statement.next() {
            return Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?);
        }

        Err(VinError::NoResultsFound)
    }

    pub fn get_vehicle_type(&self, type_id: i64) -> Result<String, VinError> {
        if type_id <= 0 {
            return Err(VinError::NoResultsFound);
        }

        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => return Err(VinError::VPICNoConnection),
        };

        let query = "SELECT Name FROM VehicleType WHERE Id = ?";

        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return Err(VinError::VPICQueryError);
            }
        };

        statement
            .bind((1, type_id))
            .map_err(|_| VinError::VPICQueryError)?;

        if let Ok(State::Row) = statement.next() {
            return Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?);
        }

        Err(VinError::NoResultsFound)
    }

    pub fn get_manufacturer_country(&self, schema_id: i64) -> Result<String, VinError> {
        todo!()
    }

    /*
       declare
       @descriptor varchar(17) = dbo.fVinDescriptor(@vin)

       if LEN(@vin) > 3
       Begin
           set @keys = SUBSTRING(@vin, 4, 5)
           if LEN(@vin) > 9
               set @keys  = @keys + '|' + SUBSTRING(@vin, 10, 8)
       end
    */
    pub fn as_key(&self) -> String {
        let vin_clone = &self.vin;
        if self.vin.len() < 4 {
            return String::new();
        }

        let mut key = vin_clone[3..8].to_string();

        if self.vin.len() > 9 {
            key.push('|');
            key.push_str(&vin_clone[9..17]);
        }

        key
    }
}

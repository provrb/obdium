use chrono::Datelike;
use sqlite::{Connection, State};
use std::cell::OnceCell;

use crate::pid::{engine::ValveTrainDesign, fuel::FuelDeliveryType};
use crate::vin::element_ids::ElementId;

const VPIC_DB_PATH: &str = "./data/vpic.sqlite";

#[derive(Debug)]
pub enum VinError {
    VPICConnectFailed,
    VPICNoConnection,
    VPICQueryError,
    VPICNoLookupTable,

    ParseError,
    InvalidVinLength,
    WMIError,
    ModelYearError,

    BadKey,
    NoResultsFound,
}

#[derive(Default)]
pub struct VIN {
    vpic_db_con: Option<sqlite::Connection>,
    vin: String,

    key_cache: OnceCell<String>,
}

impl PartialEq for VIN {
    fn eq(&self, other: &Self) -> bool {
        self.vin == other.vin
    }
}

impl VIN {
    pub fn new<T>(vin: T) -> Self
    where
        T: Into<String>,
    {
        let mut _vin = Self {
            vin: vin.into(),
            ..Default::default()
        };

        if _vin.connect_to_vpic_db().is_err() {
            println!("Error connecting to VPIC database.");
        }

        _vin
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
    pub fn as_key(&self) -> &str {
        self.key_cache.get_or_init(|| {
            let vin = self.vin.as_str();
            if vin.len() < 4 {
                return String::new();
            }

            match vin.len() {
                0..=8 => vin[3..8].to_string(),
                10..=usize::MAX => {
                    let mut key = String::with_capacity(13); // 5 + 1 + 8
                    key.push_str(&vin[3..8]);
                    key.push('|');
                    key.push_str(&vin[9..17]);
                    key
                }
                _ => vin[3..8].to_string(),
            }
        })
    }

    pub fn test_database_connection(&self) -> bool {
        self.vpic_db_con.is_some()
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

        let con = self.vpic_connection()?;

        let query = "SELECT * FROM Pattern WHERE ElementId = 18";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        let mut matched_schema_ids = Vec::new();

        while let Ok(State::Row) = statement.next() {
            // this is where you would check the pattern from Pattern matches the key.
            let pattern = statement
                .read::<String, _>("Keys")
                .map_err(|_| VinError::VPICQueryError)?;

            let pattern_sql_like = pattern.replace("*", "_") + "%"; // simulate MSSQL logic

            if VIN::match_pattern(key, &pattern_sql_like) {
                matched_schema_ids.push(
                    statement
                        .read::<i64, _>("VinSchemaId")
                        .map_err(|_| VinError::VPICQueryError)?,
                );
            }
        }

        let query = "SELECT * FROM Wmi_VinSchema WHERE WmiId = ? and ? BETWEEN YearFrom and IFNULL(YearTo, 2999)";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

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

    pub fn get_model_id(&self, schema_id: i64) -> Result<i64, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::VehicleModel, key)?;

        data.4.parse().map_err(|_| VinError::ParseError)
    }

    pub fn get_vspec_schema_id(&self, model_id: i64, make_id: i64) -> Result<i64, VinError> {
        // query VehicleSpecSchema with MakeId
        // save all rows 'Id'
        // iterate through all rows 'Id' that matched with 'MakeId'
        // query VehicleSpecSchema_Model with 'Id' comparing ModelId to 'ModelId'
        //      if match return Id
        //      No match: Error no result found

        let con = self.vpic_connection()?;
        let mut matched_spec_schema_ids = Vec::new();

        {
            let query = format!(
                "SELECT Id FROM VehicleSpecSchema WHERE MakeId = {}",
                make_id
            );
            let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

            while let Ok(State::Row) = statement.next() {
                let id = statement
                    .read::<i64, _>("Id")
                    .map_err(|_| VinError::VPICQueryError)?;
                matched_spec_schema_ids.push(id);
            }

            matched_spec_schema_ids.sort();
        }

        {
            let query = format!(
                "SELECT VehicleSpecSchemaId FROM VehicleSpecSchema_Model WHERE ModelId = {}",
                model_id
            );
            let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

            while let Ok(State::Row) = statement.next() {
                let spec_schema_id = statement
                    .read::<i64, _>("VehicleSpecSchemaId")
                    .map_err(|_| VinError::VPICQueryError)?;

                match matched_spec_schema_ids.binary_search(&spec_schema_id) {
                    Ok(_) => return Ok(spec_schema_id),
                    Err(_) => continue,
                }
            }
        }

        Err(VinError::NoResultsFound)
    }

    pub fn get_vspec_pattern_id(
        &self,
        vspec_schema_id: i64,
        vin_schema_id: i64, // AKA 'schema_id'
    ) -> Result<i64, VinError> {
        /*
            So apparently VSpecSchemaPatternIds have a row
            with ElementId 38 (for 'Trim'). If one VSpecSchemaId
            matches multiple VSpecSchemaPatternIds, you can differentiate
            to find the one that matches the correct model by looking at
            the ElementId 38 row's AttributeId. An example might be "Preferred".
            This will always be the same as querying 'Pattern' with ElementId 'Trim'.

            For example, querying ElementId::Trim with id '15103' will give:
            Ok((2080567, 15103, "*J[AE]", 38, "Preferred"))
            Notice the AttributeId, "Preferred".

            Let's get the VSpecSchemaId using get_vehicle_spec_schema_id.
            VSpecSchemaId will be 248 if WmiSchemaId is 15103.
            Now we try and search for the VSpecSchemaPatternId with
            VSpecSchemaId as 248. This query yields multiple VSpecSchemaPatternIds for 248.

            Results:
            VSpecSchemaPatternId - VSpecSchemaId
                            461 - 248
                            462 - 248
                            463 - 248
                            464 - 248
                            465 - 248

            This is an issue because we need the correct VSpecSchemaPatternId to get
            the respective information about the correct vehicle model.

            This is where querying the Trim element id will come in.
            Find expected Trim attribute:
            1. Query 'Pattern' with WmiSchemaId where ElementId = 38 (Trim)
            2. Store this in 'expected_trim'

            For every id (v_id) returned (VSpecSchemaPatternIds)
                1. Query VSpecSchemaPattern for AttributeId
                    where VSpecSchemaPatternId = v_id
                    and ElementId = 38 (Trim)
                2. If AttributeId == expected_trim
                    - Then we found our correct VSpecSchemaPatternId for our vehicle.
                3. Otherwise, continue

            Worst case: If there are no VSpecSchemaPatternIds found, we assume there is no
            VSpecSchemaPatternId for our VSpecSchemaId and return VinError::NoResultsFound.
        */
        let con = self.vpic_connection()?;
        let vin_key = self.as_key();
        let query = "SELECT Id FROM VSpecSchemaPattern WHERE SchemaId = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, vspec_schema_id))
            .map_err(|_| VinError::VPICQueryError)?;

        while let Ok(State::Row) = statement.next() {
            let pattern_id = statement
                .read::<i64, _>("Id")
                .map_err(|_| VinError::VPICQueryError)?;

            // Check the key element id and find its attribute

            // Find key element id
            let mut pattern_query = con.prepare(
                "SELECT ElementId, AttributeId FROM VehicleSpecPattern WHERE IsKey = 1 AND VSpecSchemaPatternId = ?"
            ).map_err(|_| VinError::VPICQueryError)?;

            pattern_query
                .bind((1, pattern_id))
                .map_err(|_| VinError::VPICQueryError)?;

            if let Ok(State::Row) = pattern_query.next() {
                let key_element_id = pattern_query
                    .read::<i64, _>("ElementId")
                    .map_err(|_| VinError::VPICQueryError)?;

                let key_attribute = pattern_query
                    .read::<String, _>("AttributeId")
                    .map_err(|_| VinError::VPICQueryError)?;

                // query pattern with schema id and key_element_id
                // compare key_attribute to the attribute from schema_id
                if let Ok(element_id) = ElementId::try_from(key_element_id as u16) {
                    let data = self.query_pattern(vin_schema_id, element_id, vin_key)?;
                    if data.4 == key_attribute {
                        return Ok(pattern_id);
                    }
                }
            }
        }

        Err(VinError::NoResultsFound)
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
            if let '0'..='9' = pos7 {
                model_year -= 30;
            }
        }

        if model_year > chrono::Utc::now().year() + 1 {
            model_year -= 30;
        }

        Ok(model_year)
    }

    pub fn get_engine_model(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineModel, key)?;

        Ok(data.4)
    }

    pub fn get_cylinder_count(&self, schema_id: i64) -> Result<i64, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineCylinderCount, key)?;

        data.4.parse().map_err(|_| VinError::ParseError)
    }

    pub fn get_engine_displacement(&self, schema_id: i64) -> Result<f64, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineDisplacement, key)?;

        data.4.parse().map_err(|_| VinError::ParseError)
    }

    pub fn get_fuel_type(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::FuelType, key)?;
        let fuel_type_id: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;
        let query = "SELECT Name FROM FuelType WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, fuel_type_id))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_valve_train_design(&self, schema_id: i64) -> Result<ValveTrainDesign, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::ValveTrainDesign, key)?;
        let id: u8 = data.4.parse().map_err(|_| VinError::ParseError)?;

        Ok(ValveTrainDesign::from_u8(id))
    }

    pub fn get_fuel_delivery_type(&self, schema_id: i64) -> Result<FuelDeliveryType, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::FuelDeliveryType, key)?;
        let id: u8 = data.4.parse().map_err(|_| VinError::ParseError)?;

        Ok(FuelDeliveryType::from_u8(id))
    }

    pub fn has_turbo(&self, schema_id: i64) -> Result<bool, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::HasTurbo, key)?;
        let turbo: u8 = data.4.parse().map_err(|_| VinError::ParseError)?;

        Ok(turbo == 1)
    }

    pub fn get_engine_manufacturer(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::EngineManufacturer, key)?;

        Ok(data.4)
    }

    pub fn get_vehicle_door_count(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::VehicleDoorCount, key)?;

        data.4.parse().map_err(|_| VinError::ParseError)
    }

    pub fn get_vehicle_model(&self, schema_id: i64) -> Result<String, VinError> {
        let model_id = self.get_model_id(schema_id)?;

        let con = match &self.vpic_db_con {
            Some(con) => con,
            None => return Err(VinError::VPICNoConnection),
        };

        let query = "SELECT Name FROM Model WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, model_id))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
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
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, type_id))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_plant_country(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::PlantCountry, key)?;
        let country_id: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;

        let query = "SELECT Name FROM Country WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, country_id))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_plant_city(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::PlantCity, key)?;

        Ok(data.4)
    }

    pub fn get_vehicle_make(&self, make_id: i64) -> Result<String, VinError> {
        let con = self.vpic_connection()?;
        let query = "SELECT Name FROM Make WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, make_id))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_body_class(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::BodyClass, key)?;
        let body_style_id: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;
        let query = "SELECT Name FROM BodyStyle WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, body_style_id))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn get_transmission_style(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::TransmissionStyle)
    }

    pub fn get_steering_location(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::SteeringLocation)
    }

    pub fn abs_availablility(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::ABS)
    }

    pub fn adaptive_driving_beam_availability(
        &self,
        vspec_pattern_id: i64,
    ) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::AdaptiveDrivingBeam)
    }

    pub fn keyless_ignition_availability(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::KeylessIgnition)
    }

    // FIXME: airbags are in regular 'Pattern'
    pub fn airbag_locations_front(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::AirbagLocationsFront)
    }

    pub fn airbag_locations_knee(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::AirbagLocationsKnee)
    }

    pub fn airbag_locations_side(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::AirbagLocationsSide)
    }

    pub fn airbag_locations_curtain(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::AirbagLocationsCurtain)
    }

    pub fn airbag_locations_seat_cushion(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::AirbagLocationsSeatCushion)
    }

    fn vpic_connection(&self) -> Result<&Connection, VinError> {
        self.vpic_db_con.as_ref().ok_or(VinError::VPICConnectFailed)
    }

    fn connect_to_vpic_db(&mut self) -> Result<&Connection, VinError> {
        if self.vpic_db_con.is_none() {
            let conn = Connection::open(VPIC_DB_PATH).map_err(|_| VinError::VPICConnectFailed);
            self.vpic_db_con = conn.ok();
        }

        self.vpic_connection()
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
        let con = self.vpic_connection()?;

        let query = "SELECT * FROM Pattern WHERE VinSchemaId = ? and ElementId = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

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

            if VIN::match_pattern(key, &pattern_sql_like) {
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

        Err(VinError::NoResultsFound)
    }

    fn query_vspec_pattern(
        &self,
        vspec_pattern_id: i64,
        element_id: ElementId,
    ) -> Result<(i64, i64, i64, String), VinError> {
        let con = self.vpic_connection()?;

        let query =
            "SELECT * FROM VehicleSpecPattern WHERE VSpecSchemaPatternId = ? and ElementId = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, vspec_pattern_id))
            .map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((2, element_id.as_i64()))
            .map_err(|_| VinError::VPICQueryError)?;

        if let Ok(State::Row) = statement.next() {
            let pattern_id = statement
                .read::<i64, _>("Id")
                .map_err(|_| VinError::VPICQueryError)?;

            let attribute_id = statement
                .read::<String, _>("AttributeId")
                .map_err(|_| VinError::VPICQueryError)?;

            return Ok((
                pattern_id,
                vspec_pattern_id,
                element_id.as_i64(),
                attribute_id,
            ));
        }

        Err(VinError::NoResultsFound)
    }

    fn get_vehicle_spec(&self, lookup_table: &str, spec_id: i64) -> Result<String, VinError> {
        let con = self.vpic_connection()?;
        let query = format!("SELECT Name FROM {} WHERE Id = ?", lookup_table);
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, spec_id))
            .map_err(|_| VinError::VPICQueryError)?;

        if let Ok(State::Row) = statement.next() {
            return statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError);
        }

        Err(VinError::NoResultsFound)
    }

    fn get_lookup_table(&self, element_id: ElementId) -> Option<String> {
        // try and find the lookup table name for Element Id
        let con = self.vpic_connection().ok()?;

        let query = "SELECT LookupTable FROM Element WHERE Id = ?";
        let mut statement = con.prepare(query).ok()?;
        statement.bind((1, element_id.as_i64())).ok()?;

        if let Ok(State::Row) = statement.next() {
            let x = statement.read::<String, _>("LookupTable").ok();
            println!("Result: {:?}", x);
            x
        } else {
            None
        }
    }

    fn get_spec_from_pattern(
        &self,
        vspec_pattern_id: i64,
        element_id: ElementId,
    ) -> Result<String, VinError> {
        println!(
            "VSpecPatternId: {}, ElementId: {:?}",
            vspec_pattern_id, element_id
        );
        let table_name = match self.get_lookup_table(element_id) {
            Some(name) => name,
            None => return Err(VinError::VPICNoLookupTable),
        };

        let data = self.query_vspec_pattern(vspec_pattern_id, element_id)?;
        let id: i64 = data.3.parse().map_err(|_| VinError::ParseError)?;

        let spec = self.get_vehicle_spec(&table_name, id);
        println!("{table_name} {:?}", spec);
        spec
    }
}

use chrono::Datelike;
use sqlite::State;

use crate::pid::fuel::FuelDeliveryType;
use crate::vin::element_ids::ElementId;
use crate::vin::parser::{VinError, VIN};

impl VIN {
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

    pub fn get_valve_train_design(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::ValveTrainDesign, key)?;
        let id: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;
        let query = "SELECT Name FROM FuelType WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, id))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
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

    pub fn airbag_locations_front(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::AirbagLocationsFront, key)?;
        let airbag_loc: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;

        let query = "SELECT Name FROM AirBagLocFront WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, airbag_loc))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn airbag_locations_knee(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::AirbagLocationsFront, key)?;
        let airbag_loc: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;

        let query = "SELECT Name FROM AirBagLocKnee WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, airbag_loc))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn airbag_locations_side(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::AirbagLocationsSide, key)?;
        let airbag_loc: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;

        let query = "SELECT Name FROM AirBagLocations WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, airbag_loc))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }

    pub fn airbag_locations_curtain(&self, schema_id: i64) -> Result<String, VinError> {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, ElementId::AirbagLocationsCurtain, key)?;
        let airbag_loc: i64 = data.4.parse().map_err(|_| VinError::ParseError)?;

        let con = self.vpic_connection()?;

        let query = "SELECT Name FROM AirBagLocations WHERE Id = ?";
        let mut statement = con.prepare(query).map_err(|_| VinError::VPICQueryError)?;

        statement
            .bind((1, airbag_loc))
            .map_err(|_| VinError::VPICQueryError)?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError)?),
            _ => Err(VinError::NoResultsFound),
        }
    }
}

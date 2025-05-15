use std::str::FromStr;

use chrono::Datelike;
use sqlite::State;

use crate::vin::element_ids::ElementId;
use crate::vin::parser::{VinError, VIN};

impl VIN {
    pub(crate) fn get_attribute_id_from_pattern<T: FromStr>(
        &self,
        schema_id: i64,
        element_id: ElementId,
    ) -> Result<T, VinError>
    where
        T::Err: std::fmt::Debug,
    {
        let key = self.as_key();
        let data = self.query_pattern(schema_id, element_id, key)?;

        data.4.parse::<T>().map_err(|_| VinError::ParseError)
    }

    pub(crate) fn lookup_name_from_id(
        &self,
        table_name: &str,
        id: i64,
    ) -> Result<String, VinError> {
        let con = self.vpic_connection()?;
        let query = format!("SELECT Name FROM {} WHERE Id = ?", table_name);
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError("lookup_name_from_id: prepare statement"))?;

        statement
            .bind((1, id))
            .map_err(|_| VinError::VPICQueryError("lookup_name_from_id: bind statement"))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError("lookup_name_from_id: read response"))?),
            _ => Err(VinError::NoResultsFound("lookup_name_from_id")),
        }
    }

    // Based off of NHTSA's ModelYear2 MS SQL Server procedure.
    pub fn get_model_year(&self) -> Result<i32, VinError> {
        let vin = self.get_vin();
        let pos10 = match vin.chars().nth(9) {
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

        let wmi = self.get_wmi();
        let vehicle_type_id = self.get_vehicle_type_id(&wmi)?;
        let truck_type_id = self.get_truck_type_id(&wmi)?;
        let mut car_lt = false;

        if (2..=7).contains(&vehicle_type_id) || (vehicle_type_id == 3 && truck_type_id == 1) {
            car_lt = true;
        }

        let pos7 = match vin.chars().nth(6) {
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
        self.get_attribute_id_from_pattern::<String>(schema_id, ElementId::EngineModel)
    }

    pub fn get_cylinder_count(&self, schema_id: i64) -> Result<i64, VinError> {
        self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::EngineCylinderCount)
    }

    pub fn get_engine_displacement(&self, schema_id: i64) -> Result<f64, VinError> {
        self.get_attribute_id_from_pattern::<f64>(schema_id, ElementId::EngineDisplacement)
    }

    pub fn get_fuel_type(&self, schema_id: i64) -> Result<String, VinError> {
        let fuel_type_id =
            self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::FuelType)?;
        self.lookup_name_from_id("FuelType", fuel_type_id)
    }

    pub fn get_valve_train_design(&self, schema_id: i64) -> Result<String, VinError> {
        let id =
            self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::ValveTrainDesign)?;
        self.lookup_name_from_id("ValvetrainDesign", id)
    }

    pub fn get_fuel_delivery_type(&self, schema_id: i64) -> Result<String, VinError> {
        let id = self.get_attribute_id_from_pattern(schema_id, ElementId::FuelDeliveryType)?;

        self.lookup_name_from_id("FuelDeliveryType", id)
    }

    pub fn has_turbo(&self, schema_id: i64) -> Result<bool, VinError> {
        let turbo = self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::HasTurbo)?;

        Ok(turbo == 1)
    }

    pub fn get_engine_manufacturer(&self, schema_id: i64) -> Result<String, VinError> {
        self.get_attribute_id_from_pattern::<String>(schema_id, ElementId::EngineManufacturer)
    }

    pub fn get_vehicle_door_count(&self, schema_id: i64) -> Result<String, VinError> {
        self.get_attribute_id_from_pattern(schema_id, ElementId::VehicleDoorCount)
    }

    pub fn get_vehicle_model(&self, schema_id: i64) -> Result<String, VinError> {
        let model_id = self.get_model_id(schema_id)?;
        self.lookup_name_from_id("Model", model_id)
    }

    pub fn get_vehicle_type(&self, type_id: i64) -> Result<String, VinError> {
        self.lookup_name_from_id("VehicleType", type_id)
    }

    pub fn get_plant_country(&self, schema_id: i64) -> Result<String, VinError> {
        let country_id =
            self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::PlantCountry)?;
        self.lookup_name_from_id("Country", country_id)
    }

    pub fn get_plant_city(&self, schema_id: i64) -> Result<String, VinError> {
        self.get_attribute_id_from_pattern(schema_id, ElementId::PlantCity)
    }

    pub fn get_vehicle_make(&self, make_id: i64) -> Result<String, VinError> {
        self.lookup_name_from_id("Make", make_id)
    }

    pub fn get_body_class(&self, schema_id: i64) -> Result<String, VinError> {
        let body_style_id = self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::BodyClass)?;

        self.lookup_name_from_id("BodyStyle", body_style_id)
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

    pub fn keyless_ignition_availability(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::KeylessIgnition)
    }

    pub fn airbag_locations_front(&self, schema_id: i64) -> Result<String, VinError> {
        let airbag_id =
            self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::AirbagLocationsFront)?;
        self.lookup_name_from_id("AirBagLocFront", airbag_id)
    }

    pub fn airbag_locations_knee(&self, schema_id: i64) -> Result<String, VinError> {
        let airbag_id =
            self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::AirbagLocationsKnee)?;

        self.lookup_name_from_id("AirBagLocKnee", airbag_id)
    }

    pub fn airbag_locations_side(&self, schema_id: i64) -> Result<String, VinError> {
        let airbag_id =
            self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::AirbagLocationsSide)?;
        self.lookup_name_from_id("AirBagLocations", airbag_id)
    }

    pub fn airbag_locations_curtain(&self, schema_id: i64) -> Result<String, VinError> {
        let airbag_id = self
            .get_attribute_id_from_pattern::<i64>(schema_id, ElementId::AirbagLocationsCurtain)?;

        self.lookup_name_from_id("AirBagLocations", airbag_id)
    }

    pub fn get_drive_type(&self, schema_id: i64) -> Result<String, VinError> {
        let id = self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::DriveType)?;
        self.lookup_name_from_id("DriveType", id)
    }

    pub fn get_axle_count(&self, vspec_pattern_id: i64) -> Result<i64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::AxleCount)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }

    pub fn get_brake_system(&self, schema_id: i64) -> Result<String, VinError> {
        let id = self.get_attribute_id_from_pattern(schema_id, ElementId::BrakeSystem)?;
        self.lookup_name_from_id("BrakeSystem", id)
    }

    pub fn electronic_stability_control(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::ElectronicStabilityControl)
    }

    pub fn traction_control(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::TractionControl)
    }

    pub fn windows_auto_reverse(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::AutoReverseSystem)
    }

    pub fn get_vehicle_weight_rating(&self, schema_id: i64) -> Result<String, VinError> {
        let id =
            self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::VehicleWeightRating)?;
        self.lookup_name_from_id("GrossVehicleWeightRating", id)
    }

    pub fn get_plant_company(&self, schema_id: i64) -> Result<String, VinError> {
        self.get_attribute_id_from_pattern::<String>(schema_id, ElementId::PlantCompanyName)
    }

    pub fn get_plant_state(&self, schema_id: i64) -> Result<String, VinError> {
        match self.get_attribute_id_from_pattern::<String>(schema_id, ElementId::PlantState) {
            Ok(state) => Ok(state),
            Err(_) => Ok("Not Applicable".to_string()),
        }
    }

    /// MPH
    pub fn get_vehicle_top_speed(&self, vspec_pattern_id: i64) -> Result<i64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::TopSpeedMPH)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }

    /// Inches
    pub fn get_front_wheel_size(&self, vspec_pattern_id: i64) -> Result<i64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::WheelSizeFront)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }

    /// Inches
    pub fn get_rear_wheel_size(&self, vspec_pattern_id: i64) -> Result<i64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::WheelSizeRear)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }

    pub fn dynamic_brake_support(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::DynamicBrakeSupport)
    }

    pub fn backup_camera(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::BackupCamera)
    }

    pub fn automatic_crash_notification(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::ACN)
    }

    pub fn daytime_running_light(&self, vspec_pattern_id: i64) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::DaytimeRunningLight)
    }

    pub fn semiauto_headlamp_beam_switching(
        &self,
        vspec_pattern_id: i64,
    ) -> Result<String, VinError> {
        self.get_spec_from_pattern(vspec_pattern_id, ElementId::SemiAutoHeadlampBeamSwitching)
    }

    pub fn get_transmission_speeds(&self, vspec_pattern_id: i64) -> Result<i64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::TransmissionSpeeds)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }

    pub fn get_vehicle_base_price(&self, vspec_pattern_id: i64) -> Result<f64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::VehicleBasePrice)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }

    pub fn vehicle_trim(&self, schema_id: i64) -> Result<String, VinError> {
        self.get_attribute_id_from_pattern(schema_id, ElementId::Trim)
    }

    pub fn seatbelt_type(&self, schema_id: i64) -> Result<String, VinError> {
        let id = self.get_attribute_id_from_pattern::<i64>(schema_id, ElementId::SeatbeltType)?;
        self.lookup_name_from_id("SeatBeltsAll", id)
    }

    pub fn number_of_seats(&self, vspec_pattern_id: i64) -> Result<i64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::NumberOfSeats)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }

    pub fn number_of_rows(&self, vspec_pattern_id: i64) -> Result<i64, VinError> {
        let data = self.query_vspec_pattern(vspec_pattern_id, ElementId::NumberOfRows)?;
        data.3.parse().map_err(|_| VinError::ParseError)
    }
}

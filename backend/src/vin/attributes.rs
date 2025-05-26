use chrono::Datelike;
use sqlite::State;
use std::str::FromStr;

use crate::vin::{ElementId, Error, VIN};

impl VIN {
    pub(crate) fn get_attribute_id_from_pattern<T: FromStr>(
        &self,
        element_id: ElementId,
    ) -> Result<T, Error>
    where
        T::Err: std::fmt::Debug,
    {
        let schema_id = self.get_vin_schema_id()?;
        let key = self.as_key();
        let data = self.query_pattern(schema_id, element_id, key)?;

        data.4.parse::<T>().map_err(|_| Error::ParseError)
    }

    pub(crate) fn lookup_name_from_id(&self, table_name: &str, id: i64) -> Result<String, Error> {
        let con = self.vpic_connection()?;
        let query = format!("SELECT Name FROM {} WHERE Id = ?", table_name);
        let mut statement = con
            .prepare(query)
            .map_err(|_| Error::VPICQueryError("lookup_name_from_id: prepare statement"))?;

        statement
            .bind((1, id))
            .map_err(|_| Error::VPICQueryError("lookup_name_from_id: bind statement"))?;

        match statement.next() {
            Ok(State::Row) => Ok(statement
                .read::<String, _>("Name")
                .map_err(|_| Error::VPICQueryError("lookup_name_from_id: read response"))?),
            _ => Err(Error::NoResultsFound("lookup_name_from_id")),
        }
    }

    // Based off of NHTSA's ModelYear2 MS SQL Server procedure.
    pub fn get_model_year(&self) -> Result<i32, Error> {
        let vin = self.get_vin();
        let pos10 = vin.chars().nth(9).unwrap_or('\0');

        let mut model_year = match pos10 {
            'A'..='H' => 2010 + (pos10 as i32) - ('A' as i32),
            'J'..='N' => 2010 + (pos10 as i32) - ('A' as i32) - 1,
            'P' => 2023,
            'R'..='T' => 2010 + (pos10 as i32) - ('A' as i32) - 3,
            'V'..='Y' => 2010 + (pos10 as i32) - ('A' as i32) - 4,
            '1'..='9' => 2031 + (pos10 as i32) - ('1' as i32),
            _ => {
                return Err(Error::InvalidCharacter {
                    ch: pos10,
                    pos: Some(9),
                    msg: "got invalid character for model year.",
                })
            }
        };

        let mut car_lt = false;
        let vehicle_type_id = self.get_vehicle_type_id()?;
        let truck_type_id = self.get_truck_type_id()?;

        if (2..=7).contains(&vehicle_type_id) || (vehicle_type_id == 3 && truck_type_id == 1) {
            car_lt = true;
        }

        let pos7 = vin.chars().nth(6).unwrap_or('\0');

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

    pub fn get_vehicle_manufacturer(&self) -> Result<String, Error> {
        let manufactur_id = self.get_manufacturer_id()?;
        self.lookup_name_from_id("Manufacturer", manufactur_id)
    }

    pub fn get_engine_model(&self) -> Result<String, Error> {
        self.get_attribute_id_from_pattern::<String>(ElementId::EngineModel)
    }

    pub fn get_cylinder_count(&self) -> Result<i64, Error> {
        self.get_attribute_id_from_pattern::<i64>(ElementId::EngineCylinderCount)
    }

    pub fn get_engine_displacement(&self) -> Result<f64, Error> {
        self.get_attribute_id_from_pattern::<f64>(ElementId::EngineDisplacement)
    }

    pub fn get_fuel_type(&self) -> Result<String, Error> {
        let fuel_type_id = self.get_attribute_id_from_pattern::<i64>(ElementId::FuelType)?;
        self.lookup_name_from_id("FuelType", fuel_type_id)
    }

    pub fn get_valve_train_design(&self) -> Result<String, Error> {
        let id = self.get_attribute_id_from_pattern::<i64>(ElementId::ValveTrainDesign)?;
        self.lookup_name_from_id("ValvetrainDesign", id)
    }

    pub fn get_fuel_delivery_type(&self) -> Result<String, Error> {
        let id = self.get_attribute_id_from_pattern(ElementId::FuelDeliveryType)?;

        self.lookup_name_from_id("FuelDeliveryType", id)
    }

    pub fn has_turbo(&self) -> Result<bool, Error> {
        let turbo = self.get_attribute_id_from_pattern::<i64>(ElementId::HasTurbo)?;

        Ok(turbo == 1)
    }

    pub fn get_engine_manufacturer(&self) -> Result<String, Error> {
        self.get_attribute_id_from_pattern::<String>(ElementId::EngineManufacturer)
    }

    pub fn get_vehicle_door_count(&self) -> Result<String, Error> {
        self.get_attribute_id_from_pattern(ElementId::VehicleDoorCount)
    }

    pub fn get_vehicle_model(&self) -> Result<String, Error> {
        let model_id = self.get_model_id()?;
        self.lookup_name_from_id("Model", model_id)
    }

    pub fn get_vehicle_type(&self) -> Result<String, Error> {
        let type_id = self.get_vehicle_type_id()?;
        self.lookup_name_from_id("VehicleType", type_id)
    }

    pub fn get_plant_country(&self) -> Result<String, Error> {
        let country_id = self.get_attribute_id_from_pattern::<i64>(ElementId::PlantCountry)?;
        self.lookup_name_from_id("Country", country_id)
    }

    pub fn get_plant_city(&self) -> Result<String, Error> {
        self.get_attribute_id_from_pattern(ElementId::PlantCity)
    }

    pub fn get_vehicle_make(&self) -> Result<String, Error> {
        let make_id = self.get_make_id()?;
        self.lookup_name_from_id("Make", make_id)
    }

    pub fn get_body_class(&self) -> Result<String, Error> {
        let body_style_id = self.get_attribute_id_from_pattern::<i64>(ElementId::BodyClass)?;

        self.lookup_name_from_id("BodyStyle", body_style_id)
    }

    pub fn get_transmission_style(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::TransmissionStyle)
    }

    pub fn get_steering_location(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::SteeringLocation)
    }

    pub fn abs_availablility(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::ABS)
    }

    pub fn keyless_ignition_availability(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::KeylessIgnition)
    }

    pub fn airbag_locations_front(&self) -> Result<String, Error> {
        let airbag_id =
            self.get_attribute_id_from_pattern::<i64>(ElementId::AirbagLocationsFront)?;
        self.lookup_name_from_id("AirBagLocFront", airbag_id)
    }

    pub fn airbag_locations_knee(&self) -> Result<String, Error> {
        let airbag_id =
            self.get_attribute_id_from_pattern::<i64>(ElementId::AirbagLocationsKnee)?;

        self.lookup_name_from_id("AirBagLocKnee", airbag_id)
    }

    pub fn airbag_locations_side(&self) -> Result<String, Error> {
        let airbag_id =
            self.get_attribute_id_from_pattern::<i64>(ElementId::AirbagLocationsSide)?;
        self.lookup_name_from_id("AirBagLocations", airbag_id)
    }

    pub fn airbag_locations_curtain(&self) -> Result<String, Error> {
        let airbag_id =
            self.get_attribute_id_from_pattern::<i64>(ElementId::AirbagLocationsCurtain)?;

        self.lookup_name_from_id("AirBagLocations", airbag_id)
    }

    pub fn get_drive_type(&self) -> Result<String, Error> {
        let id = self.get_attribute_id_from_pattern::<i64>(ElementId::DriveType)?;
        self.lookup_name_from_id("DriveType", id)
    }

    pub fn get_axle_count(&self) -> Result<i64, Error> {
        let data = self.query_vspec_pattern(ElementId::AxleCount)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }

    pub fn get_brake_system(&self) -> Result<String, Error> {
        let id = self.get_attribute_id_from_pattern(ElementId::BrakeSystem)?;
        self.lookup_name_from_id("BrakeSystem", id)
    }

    pub fn electronic_stability_control(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::ElectronicStabilityControl)
    }

    pub fn traction_control(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::TractionControl)
    }

    pub fn windows_auto_reverse(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::AutoReverseSystem)
    }

    pub fn get_vehicle_weight_rating(&self) -> Result<String, Error> {
        let id = self.get_attribute_id_from_pattern::<i64>(ElementId::VehicleWeightRating)?;
        self.lookup_name_from_id("GrossVehicleWeightRating", id)
    }

    pub fn get_plant_company(&self) -> Result<String, Error> {
        self.get_attribute_id_from_pattern::<String>(ElementId::PlantCompanyName)
    }

    pub fn get_plant_state(&self) -> Result<String, Error> {
        match self.get_attribute_id_from_pattern::<String>(ElementId::PlantState) {
            Ok(state) => Ok(state),
            Err(_) => Ok("Not Applicable".to_string()),
        }
    }

    /// MPH
    pub fn get_vehicle_top_speed(&self) -> Result<i64, Error> {
        let data = self.query_vspec_pattern(ElementId::TopSpeedMPH)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }

    /// Inches
    pub fn get_front_wheel_size(&self) -> Result<i64, Error> {
        let data = self.query_vspec_pattern(ElementId::WheelSizeFront)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }

    /// Inches
    pub fn get_rear_wheel_size(&self) -> Result<i64, Error> {
        let data = self.query_vspec_pattern(ElementId::WheelSizeRear)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }

    pub fn dynamic_brake_support(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::DynamicBrakeSupport)
    }

    pub fn backup_camera(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::BackupCamera)
    }

    pub fn automatic_crash_notification(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::ACN)
    }

    pub fn daytime_running_light(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::DaytimeRunningLight)
    }

    pub fn semiauto_headlamp_beam_switching(&self) -> Result<String, Error> {
        self.get_vspec_from_pattern(ElementId::SemiAutoHeadlampBeamSwitching)
    }

    pub fn get_transmission_speeds(&self) -> Result<i64, Error> {
        let data = self.query_vspec_pattern(ElementId::TransmissionSpeeds)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }

    pub fn get_vehicle_base_price(&self) -> Result<f64, Error> {
        let data = self.query_vspec_pattern(ElementId::VehicleBasePrice)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }

    pub fn vehicle_trim(&self) -> Result<String, Error> {
        self.get_attribute_id_from_pattern(ElementId::Trim)
    }

    pub fn seatbelt_type(&self) -> Result<String, Error> {
        let id = self.get_attribute_id_from_pattern::<i64>(ElementId::SeatbeltType)?;
        self.lookup_name_from_id("SeatBeltsAll", id)
    }

    pub fn number_of_seats(&self) -> Result<i64, Error> {
        let data = self.query_vspec_pattern(ElementId::NumberOfSeats)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }

    pub fn number_of_rows(&self) -> Result<i64, Error> {
        let data = self.query_vspec_pattern(ElementId::NumberOfRows)?;
        data.3.parse().map_err(|_| Error::ParseError)
    }
}

use sqlite::State;

use crate::vin::element_ids::ElementId;
use crate::vin::parser::{VinError, VIN};

impl VIN {
    pub fn get_vin_schema_id(&self) -> Result<i64, VinError> {
        let model_year = self.get_model_year()?;
        let vin_schema_id = *self.vin_schema_id.get_or_init(|| {
            // The closure must return Result<i64, VinError>
            let key = self.as_key();
            let wmi_id = self.get_wmi_id().unwrap_or(-1); // This is okay because the query will return no results.

            // Look for engine model.
            // Element Id for engine model is 18.
            // Check if pattern matches keys
            //      They match:
            //          Check Wmi_VinSchema
            //          - Ensure model year is in range 'YearFrom' - 'YearTo'
            //          - Ensure WmiId == wmi_id
            //          If these conditions are met, this is the VinSchemaId

            let con = match self.vpic_connection() {
                Ok(c) => c,
                Err(_) => return -1,
            };

            let query = "SELECT * FROM Pattern WHERE ElementId = 18";
            let mut statement = match con.prepare(query) {
                Ok(stmt) => stmt,
                Err(_) => return -1,
            };

            let mut matched_schema_ids = Vec::new();

            while let Ok(State::Row) = statement.next() {
                // this is where you would check the pattern from Pattern matches the key.
                let pattern = match statement.read::<String, _>("Keys") {
                    Ok(p) => p,
                    Err(_) => return -1,
                };

                let pattern_sql_like = pattern.replace("*", "_") + "%"; // simulate MSSQL logic

                if VIN::match_pattern(key, &pattern_sql_like) {
                    let vin_schema_id = match statement.read::<i64, _>("VinSchemaId") {
                        Ok(id) => id,
                        Err(_) => return -1,
                    };
                    matched_schema_ids.push(vin_schema_id);
                }
            }

            let query = "SELECT * FROM Wmi_VinSchema WHERE WmiId = ? and ? BETWEEN YearFrom and IFNULL(YearTo, 2999)";
            let mut statement = match con.prepare(query) {
                Ok(stmt) => stmt,
                Err(_) => return -1,
            };

            if statement.bind((1, wmi_id)).is_err() {
                return -1;
            }
            if statement.bind((2, model_year as i64)).is_err() {
                return -1;
            }

            while let Ok(State::Row) = statement.next() {
                let schema_id = match statement.read::<i64, _>("VinSchemaId") {
                    Ok(id) => id,
                    Err(_) => return -1,
                };

                if matched_schema_ids.contains(&schema_id) {
                    return schema_id;
                }
            }

            -1
        });

        if vin_schema_id == -1 {
            Err(VinError::InvalidVinSchemaId)
        } else {
            Ok(vin_schema_id)
        }
    }

    pub fn get_model_id(&self) -> Result<i64, VinError> {
        let key = self.as_key();
        let vin_schema_id = self.get_vin_schema_id()?;
        let data = self.query_pattern(vin_schema_id, ElementId::VehicleModel, key)?;

        data.4.parse().map_err(|_| VinError::ParseError)
    }

    pub fn get_vspec_schema_id(&self) -> Result<i64, VinError> {
        // query VehicleSpecSchema with MakeId
        // save all rows 'Id'
        // iterate through all rows 'Id' that matched with 'MakeId'
        // query VehicleSpecSchema_Model with 'Id' comparing ModelId to 'ModelId'
        //      if match return Id
        //      No match: Error no result found

        let con = self.vpic_connection()?;
        let make_id = self.get_make_id()?;
        let model_id = self.get_model_id()?;
        let mut matched_spec_schema_ids = Vec::new();

        let vspec_schema_id = *self.vspec_schema_id.get_or_init(|| {
            let query1 = "SELECT Id FROM VehicleSpecSchema WHERE MakeId = ?";
            let mut statement1 = match con.prepare(query1) {
                Ok(stmt) => stmt,
                Err(_) => return -1,
            };
            if statement1.bind((1, make_id)).is_err() {
                return -1;
            }

            while let Ok(State::Row) = statement1.next() {
                let id = match statement1.read::<i64, _>("Id") {
                    Ok(id) => id,
                    Err(_) => return -1,
                };
                matched_spec_schema_ids.push(id);
            }
            matched_spec_schema_ids.sort();

            let query2 =
                "SELECT VehicleSpecSchemaId FROM VehicleSpecSchema_Model WHERE ModelId = ?";
            let mut statement2 = match con.prepare(query2) {
                Ok(stmt) => stmt,
                Err(_) => return -1,
            };
            if statement2.bind((1, model_id)).is_err() {
                return -1;
            }

            while let Ok(State::Row) = statement2.next() {
                let spec_schema_id = match statement2.read::<i64, _>("VehicleSpecSchemaId") {
                    Ok(id) => id,
                    Err(_) => return -1,
                };

                match matched_spec_schema_ids.binary_search(&spec_schema_id) {
                    Ok(_) => return spec_schema_id,
                    Err(_) => continue,
                }
            }

            -1
        });

        if vspec_schema_id == -1 {
            Err(VinError::InvalidVSpecSchemaId)
        } else {
            Ok(vspec_schema_id)
        }
    }

    pub fn get_vspec_pattern_id(&self) -> Result<i64, VinError> {
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
        let vspec_schema_id = self.get_vspec_schema_id()?;
        let vin_schema_id = self.get_vin_schema_id()?;
        let con = self.vpic_connection()?;
        let vin_key = self.as_key();
        let query = "SELECT Id FROM VSpecSchemaPattern WHERE SchemaId = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((1, vspec_schema_id))
            .map_err(|_| VinError::VPICQueryError(query))?;

        while let Ok(State::Row) = statement.next() {
            let pattern_id = statement
                .read::<i64, _>("Id")
                .map_err(|_| VinError::VPICQueryError(query))?;

            // Check the key element id and find its attribute

            // Find key element id
            let mut pattern_query = con.prepare(
                "SELECT ElementId, AttributeId FROM VehicleSpecPattern WHERE IsKey = 1 AND VSpecSchemaPatternId = ?"
            ).map_err(|_| VinError::VPICQueryError(query))?;

            pattern_query
                .bind((1, pattern_id))
                .map_err(|_| VinError::VPICQueryError(query))?;

            if let Ok(State::Row) = pattern_query.next() {
                let key_element_id = pattern_query
                    .read::<i64, _>("ElementId")
                    .map_err(|_| VinError::VPICQueryError(query))?;

                let key_attribute = pattern_query
                    .read::<String, _>("AttributeId")
                    .map_err(|_| VinError::VPICQueryError(query))?;

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

        Err(VinError::NoResultsFound(query))
    }
}

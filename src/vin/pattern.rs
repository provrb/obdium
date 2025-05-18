use sqlite::State;

use crate::vin::element_ids::ElementId;
use crate::vin::parser::{VinError, VIN};

impl VIN {
    pub(crate) fn get_lookup_table(&self, element_id: ElementId) -> Option<String> {
        // try and find the lookup table name for Element Id
        let con = self.vpic_connection().ok()?;

        let query = "SELECT LookupTable FROM Element WHERE Id = ?";
        let mut statement = con.prepare(query).ok()?;
        statement.bind((1, element_id.as_i64())).ok()?;

        if let Ok(State::Row) = statement.next() {
            statement.read::<String, _>("LookupTable").ok()
        } else {
            None
        }
    }

    pub(crate) fn get_vspec_from_pattern(
        &self,
        element_id: ElementId,
    ) -> Result<String, VinError> {
        let table_name = match self.get_lookup_table(element_id) {
            Some(name) => name,
            None => return Err(VinError::VPICNoLookupTable(element_id)),
        };

        let data = self.query_vspec_pattern(element_id)?;
        let id: i64 = data.3.parse().map_err(|_| VinError::ParseError)?;

        self.get_vehicle_spec(&table_name, id)
    }

    pub(crate) fn match_pattern(key: &str, pattern: &str) -> bool {
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
    pub(crate) fn query_pattern(
        &self,
        vin_schema_id: i64,
        element_id: ElementId,
        key: &str,
    ) -> Result<(i64, i64, String, i64, String), VinError> {
        let con = self.vpic_connection()?;

        let query = "SELECT * FROM Pattern WHERE VinSchemaId = ? and ElementId = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((1, vin_schema_id))
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((2, element_id.as_i64()))
            .map_err(|_| VinError::VPICQueryError(query))?;

        while let Ok(State::Row) = statement.next() {
            let pattern = statement
                .read::<String, _>("Keys")
                .map_err(|_| VinError::VPICQueryError(query))?;

            let pattern_sql_like = pattern.replace("*", "_") + "%"; // simulate MSSQL logic

            if VIN::match_pattern(key, &pattern_sql_like) {
                let pattern_id = statement
                    .read::<i64, _>("Id")
                    .map_err(|_| VinError::VPICQueryError(query))?;

                let attribute_id = statement
                    .read::<String, _>("AttributeId")
                    .map_err(|_| VinError::VPICQueryError(query))?;

                return Ok((
                    pattern_id,
                    vin_schema_id,
                    pattern,
                    element_id.as_i64(),
                    attribute_id,
                ));
            }
        }

        Err(VinError::NoResultsFound(query))
    }

    pub(crate) fn query_vspec_pattern(
        &self,
        element_id: ElementId,
    ) -> Result<(i64, i64, i64, String), VinError> {
        let vspec_pattern_id = self.get_vspec_pattern_id()?;
        let con = self.vpic_connection()?;

        let query =
            "SELECT * FROM VehicleSpecPattern WHERE VSpecSchemaPatternId = ? and ElementId = ?";
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((1, vspec_pattern_id))
            .map_err(|_| VinError::VPICQueryError(query))?;

        statement
            .bind((2, element_id.as_i64()))
            .map_err(|_| VinError::VPICQueryError(query))?;

        if let Ok(State::Row) = statement.next() {
            let pattern_id = statement
                .read::<i64, _>("Id")
                .map_err(|_| VinError::VPICQueryError(query))?;

            let attribute_id = statement
                .read::<String, _>("AttributeId")
                .map_err(|_| VinError::VPICQueryError(query))?;

            return Ok((
                pattern_id,
                vspec_pattern_id,
                element_id.as_i64(),
                attribute_id,
            ));
        }

        Err(VinError::NoResultsFound(query))
    }

    pub(crate) fn get_vehicle_spec(
        &self,
        lookup_table: &str,
        spec_id: i64,
    ) -> Result<String, VinError> {
        let con = self.vpic_connection()?;
        let query = format!("SELECT Name FROM {} WHERE Id = ?", lookup_table);
        let mut statement = con
            .prepare(query)
            .map_err(|_| VinError::VPICQueryError("get_vehicle_spec: prepare statement"))?;

        statement
            .bind((1, spec_id))
            .map_err(|_| VinError::VPICQueryError("get_vehicle_spec: bind statement"))?;

        if let Ok(State::Row) = statement.next() {
            return statement
                .read::<String, _>("Name")
                .map_err(|_| VinError::VPICQueryError("get_vehicle_spec: parse response"));
        }

        Err(VinError::NoResultsFound("get_vehicle_spec"))
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
        let vspec_pattern_id = *self.vspec_pattern_id.get_or_init(|| {
            let vspec_schema_id = match self.get_vspec_schema_id() {
                Ok(id) => id,
                Err(_) => return -1,
            };
            let vin_schema_id = match self.get_vin_schema_id() {
                Ok(id) => id,
                Err(_) => return -1,
            };
            let con = match self.vpic_connection() {
                Ok(c) => c,
                Err(_) => return -1,
            };
            let vin_key = self.as_key();
            let query = "SELECT Id FROM VSpecSchemaPattern WHERE SchemaId = ?";
            let mut statement = match con.prepare(query) {
                Ok(stmt) => stmt,
                Err(_) => return -1
            };

            if statement.bind((1, vspec_schema_id)).is_err() {
                return -1;
            }

            while let Ok(State::Row) = statement.next() {
                let pattern_id = match statement.read::<i64, _>("Id") {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                // Check the key element id and find its attribute
                let mut pattern_query = match con.prepare(
                    "SELECT ElementId, AttributeId FROM VehicleSpecPattern WHERE IsKey = 1 AND VSpecSchemaPatternId = ?"
                ) {
                    Ok(stmt) => stmt,
                    Err(_) => continue,
                };

                if pattern_query.bind((1, pattern_id)).is_err() {
                    continue;
                }

                if let Ok(State::Row) = pattern_query.next() {
                    let key_element_id = match pattern_query.read::<i64, _>("ElementId") {
                        Ok(id) => id,
                        Err(_) => continue,
                    };

                    let key_attribute = match pattern_query.read::<String, _>("AttributeId") {
                        Ok(attr) => attr,
                        Err(_) => continue,
                    };

                    // query pattern with schema id and key_element_id
                    // compare key_attribute to the attribute from schema_id
                    if let Ok(element_id) = ElementId::try_from(key_element_id as u16) {
                        if let Ok(data) = self.query_pattern(vin_schema_id, element_id, vin_key) {
                            if data.4 == key_attribute {
                                return pattern_id;
                            }
                        }
                    }
                }
            }

            panic!("NoResultsFound: {}", query);
        });

        if vspec_pattern_id == -1 {
            Err(VinError::InvalidVSpecPatternId)
        } else {
            Ok(vspec_pattern_id)
        }
        
    }
}

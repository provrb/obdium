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
            let x = statement.read::<String, _>("LookupTable").ok();
            println!("Result: {:?}", x);
            x
        } else {
            None
        }
    }

    pub(crate) fn get_spec_from_pattern(
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

    pub(crate) fn query_vspec_pattern(
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

    pub(crate) fn get_vehicle_spec(
        &self,
        lookup_table: &str,
        spec_id: i64,
    ) -> Result<String, VinError> {
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
}

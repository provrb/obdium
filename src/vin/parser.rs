use sqlite::Connection;
use std::cell::OnceCell;
use thiserror::Error;

use super::element_ids::ElementId;

const VPIC_DB_PATH: &str = "./data/vpic.sqlite";

#[derive(Debug, Error)]
pub enum VinError {
    #[error("connection to vpic db failed.")]
    VPICConnectFailed,
    #[error("not connected to vpic db")]
    VPICNoConnection,
    #[error("no lookup table for element id {0:?}")]
    VPICNoLookupTable(ElementId),
    #[error("when querying vpic db. query: {0}")]
    VPICQueryError(&'static str),
    #[error("when converting attribute id to data type")]
    ParseError,
    #[error("vin length invalid. must be 17 characters")]
    InvalidVinLength,
    #[error("when calculate vin wmi")]
    WMIError,
    #[error("when calculating model year from vin")]
    ModelYearError,
    #[error("no results found for query {0}")]
    NoResultsFound(&'static str),
    #[error("vin schema id is invalid")]
    InvalidVinSchemaId,
    #[error("model year is invalid")]
    InvalidModelYear,
    #[error("vehicle spec schema id is invalid")]
    InvalidVSpecSchemaId,
    #[error("vehicle spec pattern id is invalid")]
    InvalidVSpecPatternId,
}

#[derive(Default)]
pub struct VIN {
    pub(crate) vpic_db_con: Option<sqlite::Connection>,

    /// Caches. These values should not change
    /// once a VIN structure is initialized.
    pub(crate) vin: OnceCell<String>,
    pub(crate) wmi: OnceCell<String>,
    pub(crate) key_cache: OnceCell<String>,
    pub(crate) wmi_id: OnceCell<i64>,
    pub(crate) vin_schema_id: OnceCell<i64>,
    pub(crate) vspec_schema_id: OnceCell<i64>,
    pub(crate) vspec_pattern_id: OnceCell<i64>,
}

impl PartialEq for VIN {
    fn eq(&self, other: &Self) -> bool {
        self.vin == other.vin
    }
}

impl VIN {
    pub fn new<T>(vin: T) -> Self
    where
        T: Into<String> + std::fmt::Debug,
    {
        let vin_string = vin.into();
        if vin_string.len() != 17 {
            panic!(
                "panic when creating new vin. vin expected length 17, got {}",
                vin_string.len()
            )
        }

        let mut _vin = Self::default();
        match _vin.vin.set(vin_string) {
            Ok(()) => (),
            Err(err) => {
                panic!("panic when creating new vin. call to OnceCell::set failed. error {err}");
            }
        }

        _vin.get_wmi();

        if _vin.connect_to_vpic_db().is_err() {
            println!("Error connecting to VPIC database. Features are limited.");
        }

        _vin
    }

    pub fn get_vin(&self) -> &str {
        match self.vin.get() {
            Some(vin) => vin,
            _ => unreachable!(),
        }
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
            let vin = self.get_vin();
            if vin.len() != 17 {
                return String::new();
            }

            let mut key = String::with_capacity(13);
            key.push_str(&vin[3..8]);
            key.push('|');
            key.push_str(&vin[9..17]);
            key
        })
    }

    pub fn test_database_connection(&self) -> bool {
        self.vpic_db_con.is_some()
    }

    pub(crate) fn vpic_connection(&self) -> Result<&Connection, VinError> {
        self.vpic_db_con.as_ref().ok_or(VinError::VPICConnectFailed)
    }

    pub(crate) fn connect_to_vpic_db(&mut self) -> Result<&Connection, VinError> {
        if self.vpic_db_con.is_none() {
            let conn = Connection::open(VPIC_DB_PATH).map_err(|_| VinError::VPICConnectFailed);
            self.vpic_db_con = conn.ok();
        }

        self.vpic_connection()
    }
}

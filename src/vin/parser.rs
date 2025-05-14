use sqlite::Connection;
use std::cell::OnceCell;

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
    pub(crate) vpic_db_con: Option<sqlite::Connection>,
    pub(crate) vin: String,
    pub(crate) key_cache: OnceCell<String>,
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

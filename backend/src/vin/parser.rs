use sqlite::Connection;
use std::cell::OnceCell;
use thiserror::Error;

use crate::vin::{ElementId, VPIC_DB_PATH};

#[derive(Debug, Error)]
pub enum Error {
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
    #[error("invalid vin character: {ch} at position {pos:?}. {msg}")]
    InvalidCharacter {
        ch: char,
        pos: Option<usize>,
        msg: &'static str,
    },
    #[error("check digit does not calculate properly. expected {expected}, found {found}")]
    InvalidCheckDigit { expected: char, found: char },
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
    pub fn new<T>(vin: T) -> Result<Self, Error>
    where
        T: Into<String> + std::fmt::Debug,
    {
        let vin_string = vin.into();
        if vin_string.len() != 17 {
            return Err(Error::InvalidVinLength);
        }

        let mut _vin = Self::default();
        match _vin.vin.set(vin_string) {
            Ok(()) => (),
            Err(err) => {
                panic!("panic when creating new vin. call to OnceCell::set failed. error {err}");
            }
        }

        _vin.checksum()?;
        _vin.get_wmi();

        if _vin.connect_to_vpic_db().is_err() {
            println!("Error connecting to VPIC database. Features are limited.");
        }

        Ok(_vin)
    }

    pub fn get_vin(&self) -> &str {
        match self.vin.get() {
            Some(vin) => vin,
            _ => panic!("tried getting gin on 'none' type."),
        }
    }

    fn get_transliteration(ch: &char) -> Result<u8, Error> {
        // Numeric digits use their own digits as transliteration
        if ch.is_numeric() {
            return Ok(ch.to_digit(10).unwrap() as u8);
        }

        match ch {
            'A' | 'J' => Ok(1),
            'B' | 'K' | 'S' => Ok(2),
            'C' | 'L' | 'T' => Ok(3),
            'D' | 'M' | 'U' => Ok(4),
            'E' | 'N' | 'V' => Ok(5),
            'F' | 'W' => Ok(6),
            'G' | 'P' | 'X' => Ok(7),
            'H' | 'Y' => Ok(8),
            'R' | 'Z' => Ok(9),
            _ => Err(Error::InvalidCharacter {
                ch: *ch,
                pos: None,
                msg: "unexpected character when transliterating.",
            }),
        }
    }

    fn get_weight(char_position: usize) -> u8 {
        match char_position {
            1 | 11 => 8,
            2 | 12 => 7,
            3 | 13 => 6,
            4 | 14 => 5,
            5 | 15 => 4,
            6 | 16 => 3,
            7 | 17 => 2,
            8 => 10,
            9 => 0,
            10 => 9,
            _ => 0,
        }
    }

    pub fn checksum(&self) -> Result<char, Error> {
        // transliterate
        // multiply each number by its weight
        // sum the products
        // divide the sum by 11 and take the remainder for the check digit

        let vin = self.get_vin();
        let mut products = Vec::new();

        // 1. Transliterate
        //    - Convert character to its number representation
        // 2. Get weight
        //    - Get the weight of the character based on its index in the vin
        //    - We add one since index starts at 0
        for (index, ch) in vin.chars().enumerate() {
            let trans = VIN::get_transliteration(&ch)?;
            let weight = VIN::get_weight(index + 1);
            products.push((trans * weight) as u16);
        }

        // Take sum and mod the sum by 11.
        // If the remainder is 10, check digit is X,
        // otherwise it is the remainder.
        let sum: u16 = products.iter().sum();
        let check_digit = (sum % 11) as u8;
        let check_char = if check_digit == 10 {
            'X'
        } else {
            char::from_digit(check_digit as u32, 10).unwrap()
        };

        // Check if the check digit in the vin
        // is the calculated check digit.
        // If not, throw an error.
        // If so, return the check digit.

        // Ninth character in the vin is the check digit
        let check_char_from_vin = vin.chars().nth(8).unwrap();
        if check_char_from_vin == check_char {
            Ok(check_char_from_vin)
        } else {
            Err(Error::InvalidCheckDigit {
                expected: check_char,
                found: check_char_from_vin,
            })
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

    pub fn is_connected(&self) -> bool {
        self.vpic_db_con.is_some()
    }

    pub(crate) fn vpic_connection(&self) -> Result<&Connection, Error> {
        self.vpic_db_con.as_ref().ok_or(Error::VPICConnectFailed)
    }

    pub(crate) fn connect_to_vpic_db(&mut self) -> Result<&Connection, Error> {
        if self.vpic_db_con.is_none() {
            match Connection::open(VPIC_DB_PATH) {
                Ok(con) => {
                    self.vpic_db_con = Some(con);
                    return self.vpic_connection();
                }
                Err(err) => {
                    println!("error: {err}");
                    return Err(Error::VPICConnectFailed);
                }
            }
        }

        self.vpic_connection()
    }
}

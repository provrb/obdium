use std::collections::HashMap;

use crate::{engine::CylinderNumber, BankNumber, Command, SensorNumber, OBD};

pub enum EvapLeakSize {
    Large,  // 0.150"
    Medium, // 0.090"
    Small,  // 0.040"
    Tiny,   // 0.020"
}

pub struct MonitorTest {
    /// Min and maximum allowed values for the specific test
    /// If a value is less than `min` or greater than `max`, the test is considered a FAIL.
    pub min: u64,
    pub max: u64,

    pub value: u64,
    pub mid: [u8; 2],
}

impl MonitorTest {
    pub fn has_passed(&self) -> bool {
        self.value >= self.min && self.value <= self.max
    }
}

impl OBD {
    // Helper function to check for supported MIDs
    fn check_mid(byte: u8, base: u8, mids: &mut HashMap<String, Vec<String>>) {
        for bit in 0..8 {
            if (byte & (1 << bit)) != 0 {
                let mid = base + bit;
                let mid_str = format!("06{:02X}", mid);
                let test_name = match mid_str.as_str() {
                    "0601" => "O2 Sensor Monitor Bank 1 - Sensor 1",
                    "0602" => "O2 Sensor Monitor Bank 1 - Sensor 2",
                    "0603" => "O2 Sensor Monitor Bank 1 - Sensor 3",
                    "0604" => "O2 Sensor Monitor Bank 1 - Sensor 4",
                    "0605" => "O2 Sensor Monitor Bank 2 - Sensor 1",
                    "0606" => "O2 Sensor Monitor Bank 2 - Sensor 2",
                    "0607" => "O2 Sensor Monitor Bank 2 - Sensor 3",
                    "0608" => "O2 Sensor Monitor Bank 2 - Sensor 4",
                    "0609" => "O2 Sensor Monitor Bank 3 - Sensor 1",
                    "060A" => "O2 Sensor Monitor Bank 3 - Sensor 2",
                    "060B" => "O2 Sensor Monitor Bank 3 - Sensor 3",
                    "060C" => "O2 Sensor Monitor Bank 3 - Sensor 4",
                    "060D" => "O2 Sensor Monitor Bank 4 - Sensor 1",
                    "060E" => "O2 Sensor Monitor Bank 4 - Sensor 2",
                    "060F" => "O2 Sensor Monitor Bank 4 - Sensor 3",
                    "0610" => "O2 Sensor Monitor Bank 4 - Sensor 4",
                    "0621" => "Catalyst Monitor Bank 1",
                    "0622" => "Catalyst Monitor Bank 2",
                    "0623" => "Catalyst Monitor Bank 3",
                    "0624" => "Catalyst Monitor Bank 4",
                    "0631" => "EGR Monitor Bank 1",
                    "0632" => "EGR Monitor Bank 2",
                    "0633" => "EGR Monitor Bank 3",
                    "0634" => "EGR Monitor Bank 4",
                    "0635" => "VVT Monitor Bank 1",
                    "0636" => "VVT Monitor Bank 2",
                    "0637" => "VVT Monitor Bank 3",
                    "0638" => "VVT Monitor Bank 4",
                    "0639" => "EVAP Monitor (Cap Off / 0.150\")",
                    "063A" => "EVAP Monitor (0.090\")",
                    "063B" => "EVAP Monitor (0.040\")",
                    "063C" => "EVAP Monitor (0.020\")",
                    "063D" => "Purge Flow Monitor",
                    "0641" => "O2 Sensor Heater Monitor Bank 1 - Sensor 1",
                    "0642" => "O2 Sensor Heater Monitor Bank 1 - Sensor 2",
                    "0643" => "O2 Sensor Heater Monitor Bank 1 - Sensor 3",
                    "0644" => "O2 Sensor Heater Monitor Bank 1 - Sensor 4",
                    "0645" => "O2 Sensor Heater Monitor Bank 2 - Sensor 1",
                    "0646" => "O2 Sensor Heater Monitor Bank 2 - Sensor 2",
                    "0647" => "O2 Sensor Heater Monitor Bank 2 - Sensor 3",
                    "0648" => "O2 Sensor Heater Monitor Bank 2 - Sensor 4",
                    "0649" => "O2 Sensor Heater Monitor Bank 3 - Sensor 1",
                    "064A" => "O2 Sensor Heater Monitor Bank 3 - Sensor 2",
                    "064B" => "O2 Sensor Heater Monitor Bank 3 - Sensor 3",
                    "064C" => "O2 Sensor Heater Monitor Bank 3 - Sensor 4",
                    "064D" => "O2 Sensor Heater Monitor Bank 4 - Sensor 1",
                    "064E" => "O2 Sensor Heater Monitor Bank 4 - Sensor 2",
                    "064F" => "O2 Sensor Heater Monitor Bank 4 - Sensor 3",
                    "0650" => "O2 Sensor Heater Monitor Bank 4 - Sensor 4",
                    "0661" => "Heated Catalyst Monitor Bank 1",
                    "0662" => "Heated Catalyst Monitor Bank 2",
                    "0663" => "Heated Catalyst Monitor Bank 3",
                    "0664" => "Heated Catalyst Monitor Bank 4",
                    "0671" => "Secondary Air Monitor 1",
                    "0672" => "Secondary Air Monitor 2",
                    "0673" => "Secondary Air Monitor 3",
                    "0674" => "Secondary Air Monitor 4",
                    "0681" => "Fuel System Monitor Bank 1",
                    "0682" => "Fuel System Monitor Bank 2",
                    "0683" => "Fuel System Monitor Bank 3",
                    "0684" => "Fuel System Monitor Bank 4",
                    "0685" => "Boost Pressure Control Monitor Bank 1",
                    "0686" => "Boost Pressure Control Monitor Bank 2",
                    "0690" => "NOx Absorber Monitor Bank 1",
                    "0691" => "NOx Absorber Monitor Bank 2",
                    "0698" => "NOx Catalyst Monitor Bank 1",
                    "0699" => "NOx Catalyst Monitor Bank 2",
                    "06A1" => "Misfire Monitor General Data",
                    "06A2" => "Misfire Cylinder 1 Data",
                    "06A3" => "Misfire Cylinder 2 Data",
                    "06A4" => "Misfire Cylinder 3 Data",
                    "06A5" => "Misfire Cylinder 4 Data",
                    "06A6" => "Misfire Cylinder 5 Data",
                    "06A7" => "Misfire Cylinder 6 Data",
                    "06A8" => "Misfire Cylinder 7 Data",
                    "06A9" => "Misfire Cylinder 8 Data",
                    "06AA" => "Misfire Cylinder 9 Data",
                    "06AB" => "Misfire Cylinder 10 Data",
                    "06AC" => "Misfire Cylinder 11 Data",
                    "06AD" => "Misfire Cylinder 12 Data",
                    "06B0" => "PM Filter Monitor Bank 1",
                    "06B1" => "PM Filter Monitor Bank 2",
                    _ => "Unknown Monitor",
                };
                mids.insert(mid_str, vec![test_name.to_string()]);
            }
        }
    }

    pub fn get_supported_mids(&mut self) -> HashMap<String, Vec<String>> {
        let mut mids = HashMap::new();

        // Query the ECU for supported MIDs
        let response = self.query(Command::new_pid(&[b'0', b'6', b'0', b'0']));
        if *response.get_payload_size() == 0 {
            return mids;
        }

        // The response format is [06, MID, TID, data1, data2, data3]
        // We need to parse the response to get the supported MIDs
        let mid_value = response.a_value() as u8;
        let tid_value = response.b_value() as u8;
        let data1 = response.c_value() as u8;
        let data2 = response.d_value() as u8;
        let data3 = response.e_value() as u8;

        // Check each byte in the response
        Self::check_mid(mid_value, 0x01, &mut mids);
        Self::check_mid(tid_value, 0x21, &mut mids);
        Self::check_mid(data1, 0x41, &mut mids);
        Self::check_mid(data2, 0x61, &mut mids);
        Self::check_mid(data3, 0x81, &mut mids);

        mids
    }

    pub fn test_oxygen_sensor_monitor(
        &mut self,
        bank: BankNumber,
        sensor: SensorNumber,
    ) -> MonitorTest {
        let mid = match (bank, sensor) {
            (BankNumber::Bank1, SensorNumber::Sensor1) => "0601",
            (BankNumber::Bank1, SensorNumber::Sensor2) => "0602",
            (BankNumber::Bank1, SensorNumber::Sensor3) => "0603",
            (BankNumber::Bank1, SensorNumber::Sensor4) => "0604",
            (BankNumber::Bank2, SensorNumber::Sensor1) => "0605",
            (BankNumber::Bank2, SensorNumber::Sensor2) => "0606",
            (BankNumber::Bank2, SensorNumber::Sensor3) => "0607",
            (BankNumber::Bank2, SensorNumber::Sensor4) => "0608",
            _ => {
                return MonitorTest {
                    min: 0,
                    max: 0,
                    value: 0,
                    mid: [0, 0],
                }
            }
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for O2 sensor monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_catalyst_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0621",
            BankNumber::Bank2 => "0622",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for catalyst monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    // Exhaust gas recirculation
    pub fn test_egr_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0631",
            BankNumber::Bank2 => "0632",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for EGR monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    // Variable valve timing
    pub fn test_vvt_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0635",
            BankNumber::Bank2 => "0636",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for VVT monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_evap_monitor(&mut self, leak_size: EvapLeakSize) -> MonitorTest {
        let mid = match leak_size {
            EvapLeakSize::Large => "0639",  // 0.150"
            EvapLeakSize::Medium => "063A", // 0.090"
            EvapLeakSize::Small => "063B",  // 0.040"
            EvapLeakSize::Tiny => "063C",   // 0.020"
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for EVAP monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_purge_flow_monitor(&mut self) -> MonitorTest {
        let mid = "063D";

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for purge flow monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_oxygen_sensor_heater(
        &mut self,
        bank: BankNumber,
        sensor: SensorNumber,
    ) -> MonitorTest {
        let mid = match (bank, sensor) {
            (BankNumber::Bank1, SensorNumber::Sensor1) => "0641",
            (BankNumber::Bank1, SensorNumber::Sensor2) => "0642",
            (BankNumber::Bank1, SensorNumber::Sensor3) => "0643",
            (BankNumber::Bank1, SensorNumber::Sensor4) => "0644",
            (BankNumber::Bank2, SensorNumber::Sensor1) => "0645",
            (BankNumber::Bank2, SensorNumber::Sensor2) => "0646",
            (BankNumber::Bank2, SensorNumber::Sensor3) => "0647",
            (BankNumber::Bank2, SensorNumber::Sensor4) => "0648",
            _ => {
                return MonitorTest {
                    min: 0,
                    max: 0,
                    value: 0,
                    mid: [0, 0],
                }
            }
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for O2 sensor heater monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_heated_catalyst_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0661",
            BankNumber::Bank2 => "0662",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        // The response format for heated catalyst monitor is:
        // A: Test value
        // B: Min limit
        // C: Max limit
        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_secondary_air_monitor(&mut self, id: u8) -> MonitorTest {
        let mid = match id {
            1 => "0671",
            2 => "0672",
            3 => "0673",
            4 => "0674",
            _ => {
                return MonitorTest {
                    min: 0,
                    max: 0,
                    value: 0,
                    mid: [0, 0],
                }
            }
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_fuel_system_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0681",
            BankNumber::Bank2 => "0682",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_boost_pressure_control_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0685",
            BankNumber::Bank2 => "0686",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_nox_absorber_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0690",
            BankNumber::Bank2 => "0691",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_nox_catalyst_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "0698",
            BankNumber::Bank2 => "0699",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_misfire_monitor_general(&mut self) -> MonitorTest {
        let mid = "06A1";

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_misfire_cylinder_monitor(&mut self, cylinder: CylinderNumber) -> MonitorTest {
        let mid = match cylinder {
            CylinderNumber::Cylinder1 => "06A2",
            CylinderNumber::Cylinder2 => "06A3",
            CylinderNumber::Cylinder3 => "06A4",
            CylinderNumber::Cylinder4 => "06A5",
            CylinderNumber::Cylinder5 => "06A6",
            CylinderNumber::Cylinder6 => "06A7",
            CylinderNumber::Cylinder7 => "06A8",
            CylinderNumber::Cylinder8 => "06A9",
            CylinderNumber::Cylinder9 => "06AA",
            CylinderNumber::Cylinder10 => "06AB",
            CylinderNumber::Cylinder11 => "06AC",
            CylinderNumber::Cylinder12 => "06AD",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }

    pub fn test_pm_filter_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        let mid = match bank {
            BankNumber::Bank1 => "06B0",
            BankNumber::Bank2 => "06B1",
        };

        let response = self.query(Command::new_pid(&[
            mid.as_bytes()[0],
            mid.as_bytes()[1],
            mid.as_bytes()[2],
            mid.as_bytes()[3],
        ]));
        if *response.get_payload_size() == 0 {
            return MonitorTest {
                min: 0,
                max: 0,
                value: 0,
                mid: [0, 0],
            };
        }

        MonitorTest {
            value: response.a_value() as u64,
            min: response.b_value() as u64,
            max: response.c_value() as u64,
            mid: [
                mid[2..4].parse().unwrap_or(0),
                mid[4..6].parse().unwrap_or(0),
            ],
        }
    }
}

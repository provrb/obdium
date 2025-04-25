use core::time;
use std::error::Error;
use std::fmt::Display;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::{cmd::Command, pid::Response};

#[derive(Debug)]
pub enum BankNumber {
    Bank1,
    Bank2,
}

#[derive(Debug)]
pub enum OBDError {
    InvalidResponse,
}

impl Display for OBDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "obd error; {}", self);
        Ok(())
    }
}

pub struct OBD {
    connection: Option<TcpStream>,
    // connection: Option<Box<dyn SerialPort>>,
}

impl OBD {
    pub fn new() -> Self {
        Self { connection: None }
    }

    // TODO: do elm327 connection rather than tcp.
    // TCP connection is for testing.
    pub fn connect(&mut self, addr: &str, port: &str) -> bool {
        let conn_addr = format!("{}:{}", addr, port);

        match TcpStream::connect(&conn_addr) {
            Ok(stream) => {
                let _ = stream.set_read_timeout(Some(time::Duration::from_secs(1)));
                self.connection = Some(stream);
                true
            }
            Err(e) => {
                println!("OBD error; connecting to TCP. Address: {conn_addr} : {e}");
                self.connection = None;
                false
            }
        }
    }

    pub fn send_request(&self, req: Command) -> bool {
        let mut stream = match &self.connection {
            Some(stream) => stream,
            None => {
                println!("OBD error; sending request. Connection not active.");
                return false;
            }
        };

        let cmd = Self::append_return_carriage(req.get_command());
        match stream.write(&cmd) {
            Ok(_) => {}
            Err(err) => {
                println!("OBD error; sending request. {err}");
                return false;
            }
        }

        true
    }

    pub fn get_response(&self) -> Option<Response> {
        let mut stream = match &self.connection {
            Some(stream) => stream,
            None => {
                println!("OBD error; getting response. Connection not active.");
                return None;
            }
        };

        let mut buffer = [0u8; 2];
        let mut response = String::new();
        loop {
            let bytes_read = stream.read(&mut buffer).unwrap_or(0);
            if bytes_read <= 0 {
                break;
            }

            let text = String::from_utf8_lossy(&buffer[..bytes_read]);
            response.push_str(&text);
        }

        let ecu_count = response.chars().filter(|c| *c == '\r').count() - 2;
        response = response.replace(" ", "");
        response = response.replace("\r", "");
        response = response.replace(">", "");

        if response.is_empty() {
            return None;
        }

        if response.len() < 2 {
            // Invalid response
            return None;
        }

        let bytes = response.len() / 2; // 2 hex chars = 1 byte
        let parsed = match Self::format_response(&response) {
            Ok(response) => response,
            Err(err) => {
                println!("error when formatting response: {}", err);
                return None;
            }
        };

        let payload_size = (bytes - 2) / ecu_count; // subtract 2 for the request. divide by amount of ecus that responded.
        let no_whitespace = parsed.replace(" ", "");
        let as_bytes = no_whitespace.as_bytes();

        let mut meta_data = Response::default();
        meta_data.ecu_count = ecu_count;
        meta_data.raw_response = Some(parsed.clone());
        meta_data.payload_size = payload_size;
        meta_data.service = [as_bytes[0], as_bytes[1]];
        meta_data.pid = [as_bytes[2], as_bytes[3]];

        meta_data.payload = Some(meta_data.payload_from_response());

        Some(meta_data)
    }

    pub fn format_response(response: &str) -> Result<String, OBDError> {
        let chunks = response
            .as_bytes()
            .chunks(2)
            .map(|pair| std::str::from_utf8(pair).unwrap_or(""))
            .collect::<Vec<&str>>();
        let as_string = chunks.join(" ");
        
        if as_string.contains("NO DA TA") {
            return Err(OBDError::InvalidResponse);
        }
        
        Ok(as_string)
    }

    pub fn rpm(&self) -> f32 {
        let response = match self.query(Command::new(b"010C")) {
            Some(data) => data,
            None => {
                println!("failed to get engine rpm. response is 'None'");
                return 0.0;
            }
        };

        ((256.0 * response.a_value()) + response.b_value()) / 4.0
    }

    pub fn engine_load(&self) -> f32 {
        let response = match self.query(Command::new(b"0104")) {
            Some(data) => data,
            None => {
                println!("failed to get engine load. response is 'None'");
                return 0.0;
            }
        };

        response.a_value() / 2.55
    }

    pub fn coolant_temp(&self) -> f32 {
        let response = match self.query(Command::new(b"0105")) {
            Some(data) => data,
            None => {
                println!("failed to get coolant temp. response is 'None'");
                return 0.0;
            }
        };

        response.a_value() - 40.0
    }

    pub fn short_term_fuel_trim(&self, bank: BankNumber) -> f32 {
        let mut command = Command::default();

        match bank {
            BankNumber::Bank1 => {
                command.set_command(b"0106");
            }
            BankNumber::Bank2 => {
                command.set_command(b"0108");
            }
        }

        let response = match self.query(command) {
            Some(data) => data,
            None => {
                println!("failed to get short term fuel trim. bank: {bank:?}. response is 'None'");
                return 0.0;
            }
        };

        (response.a_value() / 1.28) - 100.0
    }

    pub fn long_term_fuel_trim(&self, bank: BankNumber) -> f32 {
        let mut command = Command::default();

        match bank {
            BankNumber::Bank1 => {
                command.set_command(b"0107");
            }
            BankNumber::Bank2 => {
                command.set_command(b"0109");
            }
        }

        let response = match self.query(command) {
            Some(data) => data,
            None => {
                println!("failed to get long term fuel trim. bank: {bank:?}. response is 'None'");
                return 0.0;
            }
        };

        (response.a_value() / 1.28) - 100.0
    }

    pub fn fuel_pressure(&self) -> f32 {
        let response = match self.query(Command::new(b"010A")) {
            Some(data) => data,
            None => {
                println!("failed to get fuel pressure. response is 'None'");
                return 0.0;
            }
        };

        response.a_value() * 3.0
    }

    pub fn intake_manifold_abs_pressure(&self) -> f32 {
        let response = match self.query(Command::new(b"010B")) {
            Some(data) => data,
            None => {
                println!("failed to get intake manifold absolute pressure. response is 'None'");
                return 0.0;
            }
        };

        response.a_value()
    }

    pub fn vehicle_speed(&self) -> f32 {
        let response = match self.query(Command::new(b"010D")) {
            Some(data) => data,
            None => {
                println!("failed to get vehicle speed. response is 'None'");
                return 0.0;
            }
        };

        response.a_value()
    }

    pub fn timing_advance(&self) -> f32 {
        let response = match self.query(Command::new(b"010E")) {
            Some(data) => data,
            None => {
                println!("failed to get timing advance. response is 'None'");
                return 0.0;
            }
        };

        (response.a_value() / 2.0) - 64.0
    }

    pub fn intake_air_temp(&self) -> f32 {
        let response = match self.query(Command::new(b"010F")) {
            Some(data) => data,
            None => {
                println!("failed to get intake air temperature. response is 'None'");
                return 0.0;
            }
        };

        response.a_value() - 40.0
    }

    pub fn maf_air_flow_rate(&self) -> f32 {
        let response = match self.query(Command::new(b"0110")) {
            Some(data) => data,
            None => {
                println!("failed to get maf sensor air flow rate. response is 'None'");
                return 0.0;
            }
        };

        ((response.a_value() * 256.0) + response.b_value()) / 100.0
    } // Mass airflow sensor

    pub fn throttle_position(&self) -> f32 {
        let response = match self.query(Command::new(b"0111")) {
            Some(data) => data,
            None => {
                println!("failed to get throttle position. response is 'None'");
                return 0.0;
            }
        };

        response.a_value() * (100.0 / 255.0)
    }

    // pub fn oxygen_sensors_present(&self) -> f32 {}

    // pub fn read_oxygen_sensor(&self) -> f32 {}

    // pub fn obd_standards(&self) -> f32 {}

    // pub fn aux_input_status(&self) -> f32 {}

    // pub fn engine_runtime(&self) -> f32 {}

    // pub fn dist_travelled_with_mlt(&self) -> f32 {}

    // pub fn fuel_rail_pressure(&self) -> f32 {}

    // pub fn fuel_rail_guage_pressure(&self) -> f32 {}

    // pub fn o2_sensor_air_fuel_ratio(&self) -> f32 {}

    // pub fn catalyst_temp(&self) -> f32 {}

    // pub fn control_module_voltage(&self) -> f32 {}

    // pub fn cmd_air_fuel_equivalance_ratio(&self) -> f32 {}

    // pub fn relative_throttle_pos(&self) -> f32 {}

    // pub fn ambient_air_temp(&self) -> f32 {}

    // pub fn absolute_throttle_pos(&self) -> f32 {}

    // pub fn time_since_codes_cleared(&self) -> f32 {}

    // pub fn max_values_for(&self) -> f32 {} // fuel-air equivalance ratio, o2 sensor voltage, current, and instake abs pressure

    // pub fn max_air_flow_rate_from_maf(&self) -> f32 {}

    // pub fn fuel_type(&self) -> f32 {}

    // pub fn ethanol_fuel_percentage(&self) -> f32 {}

    // pub fn abs_evap_sys_vapor_pressure(&self) -> f32 {}

    // pub fn evap_sys_vapor_pressure(&self) -> f32 {}

    // pub fn engine_oil_temp(&self) -> f32 {}

    // pub fn fuel_injection_timing(&self) -> f32 {}

    // pub fn engine_fuel_rate(&self) -> f32 {}

    // pub fn emission_requirements(&self) -> f32 {}

    pub fn drivers_demand_engine_torque() {}
    pub fn actual_engine_torque() {}
    pub fn reference_engine_torque() {}
    pub fn engine_percent_torque_data() {}

    pub fn read_mass_air_flow_sensor() {}
    pub fn boost_pressure_control() {}

    pub fn turbocharger_rpm() {}
    pub fn turbocharger_temp() {}

    pub fn exhaust_pressure() {}
    pub fn exhaust_gas_temp() {}

    fn query(&self, request: Command) -> Option<Response> {
        let sent = self.send_request(request);
        if !sent {
            return None;
        }

        self.get_response()
    }

    fn append_return_carriage(byte_string: [u8; 4]) -> Vec<u8> {
        let mut result = byte_string.to_vec();
        result.push(b'\r');
        result
    }
}

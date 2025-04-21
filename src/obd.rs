use core::time;
use std::error::Error;
use std::io::{Read, Write};
use std::{io::ErrorKind, net::TcpStream};

use crate::{cmd::Command, pid::Response};

pub struct OBD {
    connection: Option<TcpStream>,
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

    pub fn send_request(&mut self, req: Command) -> bool {
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

    pub fn get_response(&mut self) -> Option<Response> {
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
            let trimmed = text.trim();
            if trimmed.ends_with(">") {
                break;
            }

            response.push_str(&trimmed);
        }

        if response.is_empty() {
            return None;
        }

        let parsed = Self::parse_response(&response);
        println!("Parsed: {}", parsed);
        Some(Response::new(parsed))
    }

    fn append_return_carriage(byte_string: [u8; 4]) -> Vec<u8> {
        let slice = byte_string.as_slice();
        let r = '\r' as u8;

        let mut result = Vec::with_capacity(slice.len() + 1);
        result.extend_from_slice(slice);
        result.push(r);
        result
    }

    fn parse_response(response: &String) -> String {
        let chunks = response
            .as_bytes()
            .chunks(2)
            .map(|pair| std::str::from_utf8(pair).unwrap_or(""))
            .collect::<Vec<&str>>();
        chunks.join(" ")
    }

    pub fn rpm() {}
    pub fn engine_load() {}
    pub fn coolant_temp() {}
    pub fn short_term_fuel_trim() {}
    pub fn long_term_fuel_trim() {}
    pub fn fuel_pressure() {}
    pub fn intake_manifold_abs_pressure() {}
    pub fn vehicle_speed() {}
    pub fn timing_advance() {}
    pub fn intake_air_temp() {}
    pub fn maf_air_flow_rate() {} // Mass airflow sensor
    pub fn throttle_position() {}
    pub fn oxygen_sensors_present() {}
    pub fn read_oxygen_sensor() {}
    pub fn obd_standards() {}
    pub fn aux_input_status() {}
    pub fn engine_runtime() {}
    pub fn dist_travelled_with_mlt() {}
    pub fn fuel_rail_pressure() {}
    pub fn fuel_rail_guage_pressure() {}
    pub fn o2_sensor_air_fuel_ratio() {}
    pub fn catalyst_temp() {}
    pub fn control_module_voltage() {}
    pub fn cmd_air_fuel_equivalance_ratio() {}
    pub fn relative_throttle_pos() {}
    pub fn ambient_air_temp() {}
    pub fn absolute_throttle_pos() {}
    pub fn time_since_codes_cleared() {}
    pub fn max_values_for() {} // fuel-air equivalance ratio, o2 sensor voltage, current, and instake abs pressure
    pub fn max_air_flow_rate_from_maf() {}
    pub fn fuel_type() {}
    pub fn ethanol_fuel_percentage() {}
    pub fn abs_evap_sys_vapor_pressure() {}
    pub fn evap_sys_vapor_pressure() {}
    pub fn engine_oil_temp() {}
    pub fn fuel_injection_timing() {}
    pub fn engine_fuel_rate() {}
    pub fn emission_requirements() {}

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
}

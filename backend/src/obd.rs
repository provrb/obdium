use serialport::SerialPort;
use sqlite::State;
use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};
use std::str::{self, FromStr};
use std::thread::sleep;
use std::time::Duration;

use crate::cmd::{Command, CommandType};
use crate::response::Response;
use crate::scalar::{Scalar, Unit};
use crate::vin::VIN;
use crate::MODE22_PIDS_DB_PATH;

#[derive(Debug)]
pub enum BankNumber {
    Bank1,
    Bank2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SensorNumber {
    Sensor1,
    Sensor2,
    Sensor3,
    Sensor4,
    Sensor5,
    Sensor6,
    Sensor7,
    Sensor8,
}

#[derive(Debug)]
pub enum Error {
    ConnectionFailed,
    NoConnection,

    InitFailed,

    InvalidResponse,
    InvalidCommand,
    NoData,
    DTCClearFailed,

    ECUUnavailable,
    ELM327WriteError,
    ELM327ReadError,
}

impl Error {
    pub fn as_str(&self) -> &str {
        match self {
            Error::InvalidResponse => "invalid response from ecu.",
            Error::InvalidCommand => "an invalid user command was going to be sent to the ecu.",
            Error::NoConnection => "no serial connection active.",
            Error::NoData => "'NO DATA' received from ECU.",
            Error::ECUUnavailable => "ecu not available.",
            Error::ELM327WriteError => "error writing through serial connection.",
            Error::ELM327ReadError => "error reading through serial connection.",
            Error::ConnectionFailed => "failed to establish connection with elm327.",
            Error::InitFailed => "failed to initialize obd with ecu.",
            Error::DTCClearFailed => "failed to clear diagnostic trouble codes.",
        }
    }
}

impl fmt::Display for Error {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "obd error; {}", self.as_str())
    }
}

pub enum Service {
    Mode01,
    Mode22,
}

#[derive(Default)]
pub struct OBD {
    connection: Option<Box<dyn SerialPort>>,
    elm_version: Option<String>,
    freeze_frame_query: bool,

    pub(crate) record_requests: bool,
    pub(crate) replay_requests: bool,
}

impl OBD {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, port: &str, baud_rate: u32) -> Result<(), Error> {
        if self.replay_requests {
            // No connection required
            return Ok(());
        }

        if self.connection.is_some() {
            return Ok(());
        }

        self.connection = serialport::new(port, baud_rate)
            .timeout(Duration::from_secs(3))
            .open()
            .ok();

        if self.connected() {
            self.init()
        } else {
            Err(Error::ConnectionFailed)
        }
    }

    pub fn connected(&self) -> bool {
        self.connection.is_some()
    }

    pub fn serial_port_name(&self) -> Option<String> {
        match &self.connection {
            Some(connection) => connection.name(),
            None => None,
        }
    }

    pub fn serial_port_baud_rate(&self) -> Option<u32> {
        match &self.connection {
            Some(connection) => connection.baud_rate().ok(),
            None => None,
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        // Initialization commands to send before
        // full communication can be established.
        // Without these, requests will always time out,
        // and the ECU wont understand what we're asking for.
        // Furthermore, causes unexpected behaviour when parsing the response.
        let commands = vec![
            Command::new_at(b"ATZ"),   // Reset all
            Command::new_at(b"ATE0"),  // Echo off
            Command::new_at(b"ATL0"),  // Linefeeds off
            Command::new_at(b"ATH1"),  // Headers on
            Command::new_at(b"ATSP0"), // Automatic protocol
        ];

        for mut command in commands {
            // Cannot proceed with the initialization.
            // Refer to above. Furthermore, if we don't send a command
            // and read the buffer and then get junk values,
            // the program will be messed up.
            // Ensure the intiialization is 100% valid.

            let response = if self.replay_requests {
                self.get_recorded_response(&command)
            } else {
                self.send_command(&mut command)
                    .map_err(|_| Error::InitFailed)?;
                self.get_at_response().map_err(|_| Error::InitFailed)?
            };

            if self.record_requests {
                self.save_request(&command, &response);
            }

            match (command.get_at(), response.formatted_response.as_deref()) {
                (b"ATZ", Some(data)) => {
                    self.elm_version = Some(data.to_owned());
                }
                (_, Some(data)) if data.contains("OK") => {}
                x => {
                    println!("{:?}", x);
                    return Err(Error::InitFailed);
                }
            }
        }

        Ok(())
    }

    /// Toggles whether PID requests are redirected to freeze frame (service 02).
    ///
    /// If `self.freeze_frame_query` is `true`, all service 01 PID requests
    /// will be redirected to service 02 (freeze frame).
    ///
    /// Returns the updated value of `freeze_frame_query`.
    pub fn query_freeze_frame(&mut self, state: bool) {
        self.freeze_frame_query = state
    }

    pub fn read_from_user_input(&mut self) {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        loop {
            let mut input = String::new();

            let _ = stdout.write(b"\n> ");
            let _ = stdout.flush();
            let _ = stdin.read_line(&mut input);

            let message = input.replace("\n", "").replace("\r", "");
            if message == "exit" {
                break;
            }

            if self.send_command(&mut Command::new_arb(&message)).is_err() {
                println!("< error sending message {message}");
                continue;
            }

            if let Ok(response) = self.read_until(b'>') {
                if response.is_empty() {
                    continue;
                }

                let printable_raw = response.escape_default();
                println!("< '{printable_raw}'");
            } else {
                println!("< error receiving response for {message}");
            }
        }
    }

    pub fn send_command(&mut self, req: &mut Command) -> Result<(), Error> {
        // We don't need to send a command since we already
        // know what the respond will be.
        if self.replay_requests {
            return Ok(());
        }

        let stream = match &mut self.connection {
            Some(stream) => stream,
            None => return Err(Error::NoConnection),
        };

        let _ = stream.clear(serialport::ClearBuffer::All);

        let mut cmd = req.as_bytes();
        if cmd.is_empty() {
            return Ok(());
        }

        cmd.push(b'\r');
        stream
            .write_all(&cmd)
            .map_err(|_| Error::ELM327WriteError)?;

        Ok(())
    }

    pub fn get_at_response(&mut self) -> Result<Response, Error> {
        let response = self.read_until(b'>')?;

        let meta_data = Response {
            raw_response: Some(response.clone()),
            formatted_response: Some(response.replace("\r", "")),

            ..Default::default()
        };

        Ok(meta_data)
    }

    pub fn get_pid_response(&mut self) -> Result<Response, Error> {
        let response = self.read_until(b'>')?;
        self.parse_pid_response(&response)
    }

    pub(crate) fn parse_pid_response(&self, raw_response: &str) -> Result<Response, Error> {
        let mut response = raw_response.replace("SEARCHING...", "").replace("\r", "\n");

        let payload_size = self.extract_payload_size(&response);
        let ecu_names = self.extract_ecu_names(&response);
        let escaped = response.clone();

        response = response.replace(" ", "").replace("\n", "").replacen(
            format!("{:02X}", payload_size).as_str(),
            "",
            1,
        );

        self.strip_ecu_names(&mut response, ecu_names.as_slice());

        if response.len() < 2 {
            return Err(Error::InvalidResponse);
        } else if response.contains("NODATA") {
            return Err(Error::NoData);
        }

        let parsed = Self::format_response(&response);
        let no_whitespace = parsed.replace(" ", "");
        let as_bytes = no_whitespace.as_bytes();

        let mut meta_data = Response::no_data();
        meta_data.responding_ecus = ecu_names;
        meta_data.formatted_response = Some(parsed.clone());
        meta_data.raw_response = Some(escaped);
        meta_data.payload_size = payload_size as usize;
        meta_data.service = [as_bytes[0], as_bytes[1]];
        meta_data.payload = Some(meta_data.payload_from_response());

        println!("Raw response: {:?}", meta_data);

        Ok(meta_data)
    }

    pub fn get_service_supported_pids(&mut self, service: &str) -> HashMap<String, Vec<String>> {
        // Service must be 2 characters long
        // An example of a service would be
        // '01' '05' or '09'
        if service.len() != 2 {
            println!("get_service_supported_pids; service ({service}) must be 2 characters long.");
            return HashMap::new();
        }

        let service_as_bytes = service.as_bytes();

        // A hashmap of supported pids for different ECUs
        // Key -> ECU Name
        // Value -> List of supported pids as strings
        let mut supported_pids: HashMap<String, Vec<String>> = HashMap::new();

        // The pids to request service 'service' to get a list of
        // supported pids by the car for that service
        let mut requests = Vec::new();

        if service == "01" {
            requests = vec!["00", "20", "40", "60", "80", "A0", "C0"];
        } else if service == "05" || service == "09" {
            // the request to send to service 05 and 09 to get supported pids from 01-20
            requests = vec!["00"];
        }

        for request_pid in requests {
            let request_pid_bytes = request_pid.as_bytes();
            let command = [
                service_as_bytes[0],
                service_as_bytes[1],
                request_pid_bytes[0],
                request_pid_bytes[1],
            ];
            let response = self.query(Command::new_pid(&command));
            let split = format!("41{}", request_pid);

            let mut parsed: HashMap<String, Vec<String>> = self.parse_supported_pids(
                &response,
                &split,
                i32::from_str_radix(request_pid, 16).unwrap_or_default(),
            );

            for (ecu_name, pids) in parsed.iter_mut() {
                supported_pids
                    .entry(ecu_name.to_string())
                    .and_modify(|existing| existing.extend(pids.clone()))
                    .or_insert(pids.to_vec());
            }
        }
        supported_pids
    }

    // todo: inefficient...
    pub(crate) fn parse_supported_pids(
        &self,
        response: &Response,
        expected_header_split: &str,
        start_pid: i32,
    ) -> HashMap<String, Vec<String>> {
        let sanitized = response.get_payload().unwrap_or_default().replace(" ", "");
        let mut responses: Vec<&str> = sanitized.split(expected_header_split).collect();
        responses = responses
            .iter()
            .filter(|x| !x.is_empty())
            .copied()
            .collect();

        // println!("{:?}", responses);
        // println!("split: {expected_header_split}");

        let mut pid = start_pid + 1;
        let mut respective_pids = HashMap::new();

        // This is a loop because it is possible that
        // the vehicle returns multiple responses from
        // different ECUs, telling us what pids EACH ECU supports.
        for (index, data) in responses.iter().enumerate() {
            let ecu = match response.responding_ecus.get(index) {
                Some(ecu_name) => ecu_name.clone(),
                None => String::default(),
            };
            let mut supported_pids: Vec<String> = Vec::new();

            // Loop through all characters in 'data'
            // Get the character as a number from the hex character 'ch'
            for ch in data.chars() {
                let as_num = u8::from_str_radix(ch.to_string().as_str(), 16).unwrap_or_default();
                let bits = format!("{:04b}", as_num);

                // Iterate through each character in binary representation
                // If bit is 1, that is a supported pid.
                for bit in bits.chars() {
                    if bit == '1' {
                        // value not found
                        supported_pids.push(format!("{pid:02X}"));
                    }

                    pid += 1;
                }
            }

            respective_pids.insert(ecu, supported_pids);
            pid = start_pid + 1;
        }

        respective_pids
    }

    pub(crate) fn extract_ecu_names(&self, response: &str) -> Vec<String> {
        response
            .lines()
            .filter_map(|line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                if split.len() < 3 {
                    return None;
                }

                let ecu_name = split[0].to_string();
                if ecu_name.len() != 3 {
                    // can ids are 3 character hex strings
                    return None;
                }

                Some(ecu_name)
            })
            .collect()
    }

    pub(crate) fn extract_payload_size(&self, response: &str) -> u32 {
        response
            .split_whitespace()
            .nth(1)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .unwrap_or(0)
    }

    pub(crate) fn strip_ecu_names(&self, response: &mut String, ecu_names: &[String]) {
        for ecu_name in ecu_names.iter() {
            *response = response.replace(ecu_name, "")
        }
    }

    pub(crate) fn read_until(&mut self, until: u8) -> Result<String, Error> {
        if self.replay_requests {
            return Ok(String::new());
        }

        let port = match &mut self.connection {
            Some(port) => port,
            None => return Err(Error::NoConnection),
        };

        port.clear(serialport::ClearBuffer::All)
            .map_err(|_| Error::ELM327ReadError)?;

        let mut buffer = [0u8; 1];
        let mut response = String::new();

        loop {
            match port.read(&mut buffer) {
                Ok(1) => {
                    let byte = buffer[0];
                    if byte == until {
                        break;
                    }

                    response.push(byte as char);
                }
                Ok(0) => std::thread::sleep(std::time::Duration::from_millis(10)),
                Ok(_) => break,
                Err(_) => break,
            }
        }

        if response.contains("UNABLE TO CONNECT") {
            println!("unable to connect");
            Err(Error::NoConnection)
        } else {
            Ok(response)
        }
    }

    pub(crate) fn read_iso_tp_response(&mut self) -> Vec<String> {
        // Reference table.
        // Example response that may have multiple frames:
        // (PID 0902 response):
        // 7E8 10 14 49 02 01 4D 41 54 - Frame 1
        // 7E8 21 34 30 33 30 39 36 42 - Frame 2
        // 7E8 22 4E 4C 30 30 30 30 30 - Frame 3
        //
        // 10 - First frame indicator ISO-TP
        // 14 - Total number of data bytes (20)
        // 49 - 4 + 09
        // 02 - response to 02 pid
        // 01 - record 01.
        // 4D 41 54 - payload starts
        // 21 - 1st consecutive frame
        // 22 - 2nd consecutive frame

        // Payload. Array of hex characters
        // (e.g ["4D", "41" ...])
        let mut payload = Vec::new();

        // Parsing
        let mut response = self.read_until(b'>').unwrap_or_default();
        response = response.replace("SEARCHING...", "");

        let ecu_names = self.extract_ecu_names(&response);
        self.strip_ecu_names(&mut response, ecu_names.as_slice());

        let stack_frames: Vec<&str> = response
            .split('\r')
            .filter(|l| !l.trim().is_empty())
            .collect();

        // Parse each stack frame
        for frame in stack_frames {
            let clean = frame.trim_start();
            let bytes: Vec<&str> = clean.split_whitespace().collect();
            if bytes.len() < 2 {
                continue;
            }

            for byte in bytes.iter() {
                match byte {
                    &"10" => {
                        // First frame
                        payload.extend(bytes[5..].iter().map(|&s| s.to_string()));
                    }
                    &"21" | &"22" | &"23" | &"24" => {
                        // ["22", "4E", "4C", "30", "30", "30", "30", "30"]
                        // We insert:
                        // ["4E", "4C", "30", "30", "30", "30", "30"]
                        payload.extend(bytes[1..].iter().map(|&s| s.to_string()));
                    }
                    _ => {
                        // unhandled or one frame
                    }
                }
            }
        }
        payload
    }

    pub fn get_vin(&mut self) -> Option<VIN> {
        match self.send_command(&mut Command::new_pid(b"0902")) {
            Ok(()) => (),
            Err(_) => return None,
        }

        // Convert hex payload to ASCII string
        let mut vin = String::new();
        for byte in self.read_iso_tp_response() {
            vin.push(
                u8::from_str_radix(&byte, 16)
                    .map(|s| s as char)
                    .expect("call to u8::from_str_radix failed on str '{byte}'"),
            );
        }

        match VIN::new(&vin) {
            Ok(vin) => Some(vin),
            Err(err) => {
                println!("error when getting vin {vin}. {err}");
                None
            }
        }
    }

    pub(crate) fn format_response(response: &str) -> String {
        let chunks = response
            .as_bytes()
            .chunks(2)
            .map(|pair| std::str::from_utf8(pair).unwrap_or(""))
            .collect::<Vec<&str>>();

        chunks.join(" ")
    }

    pub(crate) fn query(&mut self, mut request: Command) -> Response {
        if self.freeze_frame_query && *request.command_type() == CommandType::PIDCommand {
            let pid = request.get_pid();
            if pid.starts_with(b"01") {
                request.set_pid(&[b'0', b'2', pid[2], pid[3]]);
            }
        }

        match self.send_command(&mut request) {
            Ok(_) => (),
            Err(err) => {
                println!(
                    "{}\tAT: '{}' - PID: '{}' ",
                    err,
                    String::from_utf8_lossy(request.get_at()),
                    String::from_utf8(request.get_pid().to_vec()).unwrap_or_default()
                );
                return Response::default();
            }
        };

        let response = if self.replay_requests {
            self.get_recorded_response(&request)
        } else {
            self.get_pid_response().unwrap_or(Response::no_data())
        };

        if self.record_requests {
            self.save_request(&request, &response);
        }

        response
    }

    pub fn get_protocol(&mut self) -> Result<String, Error> {
        let mut request = Command::new_at(b"AT DP");
        self.send_command(&mut request)?;

        let response = if self.replay_requests {
            self.get_recorded_response(&request)
        } else {
            self.get_at_response().unwrap_or(Response::no_data())
        };

        if self.record_requests {
            self.save_request(&request, &response);
        }

        response.formatted_response.ok_or(Error::InvalidResponse)
    }

    /// Test and run Mode 22 pids from
    /// /data/model-pids.sqlite
    pub fn test_mode_22_pids(&mut self, vin: &VIN) {
        // This does not work when replaying requests
        if self.replay_requests {
            return;
        }

        // Get a Mode 22 pids for the model from the vin
        // Run them all, see if the output is valid.
        // Calculate equation and display
        // Sleep for a short period of time.
        // Repeat
        let model = match vin.get_engine_manufacturer() {
            Ok(em) => em,
            Err(_) => return,
        };

        // connect to mode 22 database
        let con = match sqlite::Connection::open(MODE22_PIDS_DB_PATH) {
            Ok(con) => con,
            Err(err) => {
                println!("when connecting to mode22 database: {err}");
                return;
            }
        };

        let query = "SELECT * FROM vehicle_pids WHERE model = ?";
        let mut statement = match con.prepare(query) {
            Ok(statement) => statement,
            Err(err) => {
                println!("when sanitizing statement {query}: {err}");
                return;
            }
        };

        match statement.bind((1, model.as_str())) {
            Ok(_) => {}
            Err(err) => {
                println!("when binding model '{}' to query {query}: {err}", model);
                return;
            }
        };

        while let Ok(State::Row) = statement.next() {
            let pid = statement
                .read::<String, _>("pid")
                .expect("reading column pid");
            let equation = statement
                .read::<String, _>("equation")
                .expect("reading column equation");
            let unit = statement
                .read::<String, _>("unit")
                .expect("reading column unit");
            let description = statement
                .read::<String, _>("description")
                .expect("reading description");

            let command = Command::new_arb(&pid);
            self.query(command).map_no_data(|response| {
                match self.calculate_dynamic_equation(&equation, &unit, &response) {
                    Ok(value) => {
                        println!("successfully calculated pid {pid}. equation: {equation}. unit {unit}");
                        println!("description: {description}");
                        println!("calculated: {value}");
                        value
                    },
                    Err(err) => {
                        println!("error trying to calculate pid {pid}. equation: {equation}. unit {unit}");
                        println!("description: {description}");
                        println!("error: {err}");
                        Scalar::no_data()
                    }
                }
            });

            sleep(Duration::from_millis(500));
        }
    }

    pub(crate) fn calculate_dynamic_equation(
        &mut self,
        equation: &str,
        unit: &str,
        response: &Response,
    ) -> Result<Scalar, Box<dyn std::error::Error>> {
        use evalexpr::*;

        if *response.get_payload_size() == 0 {
            return Ok(Scalar::no_data());
        }

        // TODO: fix this ugly code.
        let mut context: HashMapContext<DefaultNumericTypes> = HashMapContext::new();
        if equation.contains("A") {
            context.set_value("A".into(), Value::from_float(response.a_value() as f64))?;
        }
        if equation.contains("B") {
            context.set_value("B".into(), Value::from_float(response.b_value() as f64))?;
        }
        if equation.contains("C") {
            context.set_value("C".into(), Value::from_float(response.c_value() as f64))?;
        }
        if equation.contains("D") {
            context.set_value("D".into(), Value::from_float(response.d_value() as f64))?;
        }
        if equation.contains("E") {
            context.set_value("E".into(), Value::from_float(response.e_value() as f64))?;
        }

        match eval_float_with_context(equation, &context) {
            Ok(res) => Ok(Scalar::new(
                res as f32,
                Unit::from_str(unit).unwrap_or(Unit::Unknown),
            )),
            Err(err) => {
                println!("when evaluating dynamic equation {equation}. {err}");
                Err(Box::new(err))
            }
        }
    }
}

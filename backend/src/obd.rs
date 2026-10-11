use serialport::ErrorKind::{Io, NoDevice};
use serialport::SerialPort;

use sqlite::State;

use std::collections::HashMap;

use std::io::{Read, Write};
use std::str::{self, FromStr};
use std::thread::sleep;

use std::time::Duration;

use thiserror::Error;

use crate::cmd::{Command, CommandType};
use crate::response::Response;

use crate::scalar::{Scalar, Unit, UnitPreferences};
use crate::vin::VIN;

use crate::MODE22_PIDS_DB_PATH;


#[derive(Clone, Debug)]
pub struct ModuleTarget {
    pub name: &'static str,
    pub tx: &'static str,
    pub rx: &'static str,
}

pub const BMW_F30_MODULES: &[ModuleTarget] = &[
    ModuleTarget { name: "DME", tx: "7E0", rx: "7E8" },
    ModuleTarget { name: "EGS", tx: "7E1", rx: "7E9" },
];

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to establish connection with adapter")]
    ConnectionFailed,
    #[error("Attempted to perform a read/write action via serial connection, but no serial connection to adapter is active")]
    NoConnection,
    #[error("Failed to initialize connection via adapter sending command: {0}")]
    InitFailedSendingCommand(String),
    #[error("Could not initialize connection via adapter - Failed reading response")]
    InitFailedReadingResponse,
    #[error("No response containing 'OK' received from adapter while initializing connection")]
    InitFailedNoOKReceived,
    #[error("Invalid OBD protocol given while trying to connect. Expected number between 0-9, got: {0} ")]
    InvalidProtocol(u8),
    #[error("Received invalid or mangled response to command via serial connection")]
    InvalidResponse,
    #[error(
        "Received response explicitly containing 'NODATA' thus no meaningful data was received"
    )]
    NoData,
    #[error("No positive response received after sending OBD request to clear DTCs. Expected positive response '44' but got '{0}'")]
    DTCClearFailed(String),
    #[error(
        "Error occured during I/O operation via serial communication. Error details (if any): {0}"
    )]
    AdapterReadWriteError(String),
}

pub enum Service {
    Mode01,
    Mode22,
}

// Fake serial port used for simulating.
// Specifically demo mode.
struct DummySerialPort;

impl SerialPort for DummySerialPort {
    fn name(&self) -> Option<String> {
        Some("DEMO MODE".to_string())
    }

    fn baud_rate(&self) -> serialport::Result<u32> {
        Ok(0)
    }

    fn data_bits(&self) -> serialport::Result<serialport::DataBits> {
        Ok(serialport::DataBits::Eight)
    }

    fn flow_control(&self) -> serialport::Result<serialport::FlowControl> {
        Ok(serialport::FlowControl::None)
    }

    fn parity(&self) -> serialport::Result<serialport::Parity> {
        Ok(serialport::Parity::None)
    }

    fn stop_bits(&self) -> serialport::Result<serialport::StopBits> {
        Ok(serialport::StopBits::One)
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(0)
    }

    fn set_timeout(&mut self, _timeout: Duration) -> serialport::Result<()> {
        Ok(())
    }

    fn write_request_to_send(&mut self, _level: bool) -> serialport::Result<()> {
        Ok(())
    }

    fn write_data_terminal_ready(&mut self, _level: bool) -> serialport::Result<()> {
        Ok(())
    }

    fn read_clear_to_send(&mut self) -> serialport::Result<bool> {
        Ok(true)
    }

    fn read_data_set_ready(&mut self) -> serialport::Result<bool> {
        Ok(true)
    }

    fn read_ring_indicator(&mut self) -> serialport::Result<bool> {
        Ok(false)
    }

    fn read_carrier_detect(&mut self) -> serialport::Result<bool> {
        Ok(false)
    }

    fn bytes_to_read(&self) -> serialport::Result<u32> {
        Ok(0)
    }

    fn bytes_to_write(&self) -> serialport::Result<u32> {
        Ok(0)
    }

    fn clear(&self, _buffer: serialport::ClearBuffer) -> serialport::Result<()> {
        Ok(())
    }

    fn try_clone(&self) -> serialport::Result<Box<dyn SerialPort>> {
        Ok(Box::new(DummySerialPort))
    }

    fn set_baud_rate(&mut self, _baud_rate: u32) -> serialport::Result<()> {
        Ok(())
    }

    fn set_data_bits(&mut self, _data_bits: serialport::DataBits) -> serialport::Result<()> {
        Ok(())
    }

    fn set_flow_control(
        &mut self,
        _flow_control: serialport::FlowControl,
    ) -> serialport::Result<()> {
        Ok(())
    }

    fn set_parity(&mut self, _parity: serialport::Parity) -> serialport::Result<()> {
        Ok(())
    }

    fn set_stop_bits(&mut self, _stop_bits: serialport::StopBits) -> serialport::Result<()> {
        Ok(())
    }

    fn set_break(&self) -> serialport::Result<()> {
        Ok(())
    }

    fn clear_break(&self) -> serialport::Result<()> {
        Ok(())
    }
}

impl Read for DummySerialPort {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

impl Write for DummySerialPort {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Default)]
pub struct OBD {
    connection: Option<Box<dyn SerialPort>>,
    elm_version: Option<String>,
    freeze_frame_query: bool,
    protocol: u8,
    pub(crate) requests_path: String,
    pub(crate) record_requests: bool,
    pub(crate) replay_requests: bool,
    pub(crate) unit_preferences: UnitPreferences,
}

impl OBD {
    pub fn new() -> Self {
        Self {
            requests_path: "./data/requests.json".to_string(),
            ..Default::default()
        }
    }

    pub fn connect(&mut self, port: &str, baud_rate: u32, protocol: u8) -> Result<(), Error> {
        if port == "DEMO MODE" {
            self.replay_requests = true;
            self.connection = Some(Box::new(DummySerialPort));
            return Ok(());
        }
        self.replay_requests = false;
        self.record_requests = false;
        if self.connection.is_some() {
            return Ok(());
        }
        self.connection = serialport::new(port, baud_rate)
            .timeout(Duration::from_secs(5))
            .open()
            .ok();
        if self.is_connected() {
            let initialized = self.init();
            let mut command = match protocol {
                0 => Command::new_at(b"ATSP0"),
                1 => Command::new_at(b"ATSP1"),
                2 => Command::new_at(b"ATSP2"),
                3 => Command::new_at(b"ATSP3"),
                4 => Command::new_at(b"ATSP4"),
                5 => Command::new_at(b"ATSP5"),
                6 => Command::new_at(b"ATSP6"),
                7 => Command::new_at(b"ATSP7"),
                8 => Command::new_at(b"ATSP8"),
                9 => Command::new_at(b"ATSP9"),
                _ => return Err(Error::InvalidProtocol(protocol)),
            };
            let _ = self.send_command(&mut command);
            let _ = self.get_at_response();
            self.protocol = protocol;
            if protocol == 6 || protocol == 0 {
                let _ = self.prepare_bmw_328();
            }
            let _ = self.query(Command::new_pid(b"0100"));
            initialized
        } else {
            Err(Error::ConnectionFailed)
        }
    }

    pub fn disconnect(&mut self) {
        if let Some(connection) = self.connection.take() {
            drop(connection);
            self.connection = None;
        }
    }

    pub fn is_connected(&self) -> bool {
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

    pub fn get_open_serial_ports() -> Vec<(String, u32)> {
        let ports = match serialport::available_ports() {
            Ok(p) => p,
            Err(err) => {
                println!("error getting available serial ports: {err}");
                return Vec::new();
            }
        };
        const BAUD_RATES: [u32; 6] = [38400, 9600, 115200, 57600, 230400, 4800];
        let handles: Vec<_> = ports
            .into_iter()
            .map(|port| {
                std::thread::spawn(move || {
                    let port_name = port.port_name;
                    for baud_rate in BAUD_RATES.iter() {
                        let Ok(mut con) = serialport::new(&port_name, *baud_rate)
                            .timeout(Duration::from_millis(500))
                            .open()
                        else {
                            continue;
                        };
                        let _ = con.clear(serialport::ClearBuffer::All);
                        if con.write_all(b"ATZ\r").is_err() {
                            continue;
                        }
                        std::thread::sleep(Duration::from_millis(500));
                        let _ = con.clear(serialport::ClearBuffer::All);
                        if con.write_all(b"ATI\r").is_err() {
                            continue;
                        }
                        let mut buffer: [u8; 256] = [0u8; 256];
                        let mut total_read = 0;
                        loop {
                            match con.read(&mut buffer[total_read..]) {
                                Ok(n) if n > 0 => total_read += n,
                                Ok(_) => break,
                                Err(e)
                                    if e.kind() == std::io::ErrorKind::TimedOut
                                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                                {
                                    break
                                }
                                Err(_) => break,
                            }
                        }
                        let response = String::from_utf8_lossy(&buffer[..total_read]);
                        let cleaned = response.trim_matches(|c: char| !c.is_ascii_graphic());
                        println!(
                            "- response: `{}` for port: {}",
                            cleaned.escape_debug(),
                            port_name
                        );
                        if cleaned.ends_with('>') || cleaned.contains("ELM") {
                            return Some((port_name, *baud_rate));
                        }
                    }
                    None
                })
            })
            .collect();
        let open_serial_ports: Vec<(String, u32)> = handles
            .into_iter()
            .filter_map(|h| h.join().ok().flatten())
            .collect();
        println!("Ports: {:?}", open_serial_ports);
        open_serial_ports
    }

    pub fn init(&mut self) -> Result<(), Error> {
        // Initialization commands to send before
        // full communication can be established.
        // Without these, requests will always time out,
        // and the ECU wont understand what we're asking for.
        // Furthermore, causes unexpected behaviour when parsing the response.
        let commands = vec![
            Command::new_at(b"ATZ"),  // Reset all
            Command::new_at(b"ATE0"), // Echo off
            Command::new_at(b"ATL0"), // Linefeeds off
            Command::new_at(b"ATH1"), // Headers on
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
                    .map_err(|_| Error::InitFailedSendingCommand(command.as_string()))?;
                self.get_at_response()?
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
                    return Err(Error::InitFailedNoOKReceived);
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
        if self.replay_requests {
            return Ok(());
        }
        let stream = match &mut self.connection {
            Some(stream) => stream,
            None => return Err(Error::NoConnection),
        };
        let _ = stream.set_timeout(Duration::from_secs(2));
        let _ = stream.clear(serialport::ClearBuffer::All);
        let mut cmd = req.as_bytes();
        if cmd.is_empty() {
            return Ok(());
        }
        cmd.push(b'\r');
        match stream.write_all(&cmd) {
            Ok(()) => Ok(()),
            Err(_) => {
                self.connection = None;
                Err(Error::NoConnection)
            }
        }
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
        println!("ecu raw: {}", response.escape_debug());
        self.parse_pid_response(&response)
    }

    pub(crate) fn parse_pid_response(&self, raw_response: &str) -> Result<Response, Error> {
        let mut response = raw_response.replace("SEARCHING...", "").replace("\r", "\n");
        let payload_size = OBD::extract_payload_size(&response);
        let ecu_names = OBD::extract_ecu_names(&response);
        let escaped = response.clone();
        response = response.replace(" ", "").replace("\n", "").replacen(
            format!("{:02X}", payload_size).as_str(),
            "",
            1,
        );
        OBD::strip_ecu_names(&mut response, ecu_names.as_slice());
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
            println!(
                "supported-pid raw {}{}: {}",
                service,
                request_pid,
                response.raw_response.clone().unwrap_or_default().escape_debug()
            );
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
            for byte_str in data.as_bytes().chunks(2).flat_map(std::str::from_utf8) {
                let as_num = u8::from_str_radix(byte_str, 16).unwrap_or_default();
                // Iterate through each character in binary representation
                // If bit is 1, that is a supported pid.
                for i in 0..8 {
                    if (as_num & (1 << (7 - i))) != 0 {
                        supported_pids.push(format!("{:02X}", pid));
                    }
                    pid += 1;
                }
            }
            respective_pids.insert(ecu, supported_pids);
            pid = start_pid + 1;
        }
        respective_pids
    }

    pub fn extract_ecu_names(response: &str) -> Vec<String> {
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

    pub fn extract_payload_size(response: &str) -> u32 {
        response
            .split_whitespace()
            .nth(1)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .unwrap_or(0)
    }

    pub(crate) fn strip_ecu_names(response: &mut String, ecu_names: &[String]) {
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
        let mut response = self.read_until(b'>').unwrap_or_default();
        response = response.replace("SEARCHING...", "");
        let stack_frames: Vec<&str> = response
            .split('\r')
            .filter(|l| !l.trim().is_empty())
            .collect();
        let mut primary_ecu: Option<String> = None;
        for frame in stack_frames {
            let clean = frame.trim();
            let bytes: Vec<&str> = clean.split_whitespace().collect();
            if bytes.len() < 3 {
                continue;
            }
            let ecu_id = bytes[0].to_string();
            if primary_ecu.is_none() {
                primary_ecu = Some(ecu_id.clone());
            }
            if Some(ecu_id) != primary_ecu {
                continue;
            }
            let pci = match u8::from_str_radix(bytes[1], 16) {
                Ok(b) => b,
                Err(_) => continue,
            };
            let frame_type = pci >> 4;
            match frame_type {
                0x0 => {
                    let length = (pci & 0x0F) as usize;
                    if bytes.len() >= 2 + length {
                        payload.extend(bytes[2..2 + length].iter().map(|&s| s.to_string()));
                    }
                }
                0x1 if bytes.len() >= 4 => {
                    payload.extend(bytes[3..].iter().map(|&s| s.to_string()));
                }
                0x2 if bytes.len() >= 2 => {
                    payload.extend(bytes[2..].iter().map(|&s| s.to_string()));
                }
                _ => {}
            }
        }
        payload
    }

    pub fn get_vin(&mut self) -> Option<VIN> {
        let response = self.query(Command::new_pid(b"0902"));
        let raw = response.raw_response.clone().unwrap_or_default();
        println!("vin raw: {}", raw.escape_debug());
        let vin = Self::vin_from_iso_tp(&raw);
        println!("vin assembled: {vin}");
        match VIN::new(&vin) {
            Ok(vin) => Some(vin),
            Err(err) => {
                println!("error when getting vin {vin}. {err}");
                None
            }
        }
    }

    fn vin_from_iso_tp(raw: &str) -> String {
        let mut payload: Vec<u8> = Vec::new();
        for line in raw.split(|c| c == '\r' || c == '\n') {
            let bytes: Vec<&str> = line.split_whitespace().collect();
            if bytes.len() < 3 {
                continue;
            }
            let Ok(pci) = u8::from_str_radix(bytes[1], 16) else {
                continue;
            };
            let frame_type = pci >> 4;
            let data = match frame_type {
                0x0 => &bytes[2..],
                0x1 => &bytes[3..],
                0x2 => &bytes[2..],
                _ => continue,
            };
            for token in data {
                if let Ok(byte) = u8::from_str_radix(token, 16) {
                    payload.push(byte);
                }
            }
        }
        let start = payload.windows(2).position(|pair| pair == [0x49, 0x02]).unwrap_or(0);
        payload.iter().skip(start + 3).filter(|b| b.is_ascii_alphanumeric()).take(17).map(|b| *b as char).collect()
    }

    pub(crate) fn format_response(response: &str) -> String {
        let chunks = response
            .as_bytes()
            .chunks(2)
            .map(|pair| std::str::from_utf8(pair).unwrap_or(""))
            .collect::<Vec<&str>>();
        chunks.join(" ")
    }

    pub fn query(&mut self, mut request: Command) -> Response {
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

    pub fn set_unit_preferences(&mut self, preferences: UnitPreferences) {
        self.unit_preferences = preferences;
    }

    pub fn get_protocol_name(&mut self) -> Result<String, Error> {
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

    pub fn get_protocol_number(&self) -> u8 {
        self.protocol
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

    pub fn calculate_dynamic_equation(
        &mut self,
        equation: &str,
        unit: &str,
        response: &Response,
    ) -> Result<Scalar, Box<dyn std::error::Error>> {
        use evalexpr::*;
        if *response.get_payload_size() == 0 {
            return Ok(Scalar::no_data());
        }
        let equation = equation.to_uppercase();
        // TODO: fix this ugly code.
        let mut context: HashMapContext<DefaultNumericTypes> = HashMapContext::new();
        let variables = [
            ("A", response.a_value()),
            ("B", response.b_value()),
            ("C", response.c_value()),
            ("D", response.d_value()),
            ("E", response.e_value()),
        ];
        for (variable, value) in variables {
            if equation.contains(variable) {
                context.set_value(variable.into(), Value::from_float(value as f64))?;
            }
        }
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
        match eval_float_with_context(&equation, &context) {
            Ok(res) => Ok(Scalar::new(
                res as f32,
                Unit::from_str(unit).unwrap_or(Unit::Unknown),
                Some(self.unit_preferences),
            )),
            Err(err) => {
                println!("when evaluating dynamic equation {equation}. {err}");
                Err(Box::new(err))
            }
        }
    }

    pub fn prepare_bmw_328(&mut self) -> Result<(), Error> {
        if self.replay_requests {
            return Ok(());
        }
        for cmd in [
            "ATE0",
            "ATL0",
            "ATS1",
            "ATH1",
            "ATCAF1",
            "ATCFC1",
            "ATAL",
            "ATAT1",
            "ATSTFF",
            "ATR1",
            "ATAR",
            "ATFCSM0",
        ] {
            self.at_ok(cmd)?;
        }
        let probes = [("6", "7DF"), ("6", "7E0"), ("7", "18DB33F1"), ("7", "18DA10F1")];
        for (protocol, header) in probes {
            self.at_ok(&format!("ATSP{protocol}"))?;
            self.at_ok(&format!("ATSH{header}"))?;
            let raw = self.raw_obd("0100");
            println!("probe protocol {protocol} header {header}: {}", raw.escape_debug());
            let compact = raw.replace(" ", "").replace("\r", "").replace("\n", "");
            if compact.contains("4100") {
                println!("locked onto protocol {protocol} header {header}");
                let _ = self.at_ok("STCFCPC");
                if header.ends_with("7E0") || header == "7DF" {
                    let _ = self.at_ok("STCFCPA 7E0,7E8");
                    let _ = self.at_ok("STCFCPA 7E1,7E9");
                }
                return Ok(());
            }
        }
        println!("no BMW header answered 0100. ignition must be on, engine can be off.");
        Ok(())
    }

    fn raw_obd(&mut self, cmd: &str) -> String {
        let mut command = Command::new_arb(cmd);
        if self.send_command(&mut command).is_err() {
            return String::new();
        }
        self.read_until(b'>').unwrap_or_default()
    }

    pub fn select_module(&mut self, module: &ModuleTarget) -> Result<(), Error> {
        self.at_ok("ATAR")?;
        self.at_ok("ATFCSM0")?;
        self.at_ok(&format!("ATSH{}", module.tx))?;
        Ok(())
    }

    pub fn restore_functional(&mut self) -> Result<(), Error> {
        self.at_ok("ATAR")?;
        self.at_ok("ATSH7DF")?;
        self.at_ok("ATFCSM0")?;
        Ok(())
    }

    fn at_ok(&mut self, cmd: &str) -> Result<(), Error> {
        for attempt in 0..3 {
            let mut command = Command::new_arb(cmd);
            self.send_command(&mut command)?;
            let response = self.get_at_response()?;
            let text = response.formatted_response.unwrap_or_default();
            println!("adapter `{cmd}` -> {}", text.escape_debug());
            if text.contains("OK") || text.contains("ELM") || text.contains("STN") {
                return Ok(());
            }
            if text.contains("STOPPED") || text.contains("SEARCHING") || text.is_empty() {
                std::thread::sleep(Duration::from_millis(300));
                let _ = self.sync_prompt();
                continue;
            }
            if attempt == 2 {
                println!("adapter rejected `{cmd}`: {text}");
                return Err(Error::InitFailedNoOKReceived);
            }
        }
        Err(Error::InitFailedNoOKReceived)
    }

    fn sync_prompt(&mut self) -> Result<(), Error> {
        let stream = match &mut self.connection {
            Some(stream) => stream,
            None => return Err(Error::NoConnection),
        };
        let _ = stream.write_all(b"\r");
        let _ = self.read_until(b'>');
        Ok(())
    }

    /// Mode 03/07/0A plus UDS mode 19, per module. This is the Carly-style pass.
    /// Mode 03 alone is only the emissions list.
    pub fn read_bmw_codes(&mut self) -> Vec<String> {
        if self.replay_requests {
            return Vec::new();
        }
        let _ = self.at_ok("ATAR");
        let _ = self.at_ok("ATFCSM0");
        let _ = self.at_ok("ATAL");
        let mut codes = Vec::new();

        let physical = [
            ("DME", "7E0"),
            ("EGS", "7E1"),
            ("DSC", "7E2"),
            ("ABS", "7E3"),
            ("EPS", "7E4"),
        ];
        for (name, header) in physical {
            let _ = self.at_ok(&format!("ATSH{header}"));
            for service in ["03", "07", "0A"] {
                let raw = self.raw_obd(service);
                println!("{name} mode {service}: {}", raw.escape_debug());
                codes.extend(Self::decode_sae_dtcs(name, service, &raw));
            }
            for uds in ["1902FF", "19020C"] {
                let raw = self.raw_obd(uds);
                println!("{name} {uds}: {}", raw.escape_debug());
                codes.extend(Self::decode_uds_dtcs(name, &raw));
            }
        }

        // BMW extended addressing. Tester 6F1, ECU address in the first data byte.
        let extended = [
            ("DME", "12", "612"),
            ("EGS", "18", "618"),
            ("DSC", "29", "629"),
            ("FEM", "40", "640"),
            ("KOMBI", "60", "660"),
        ];
        let _ = self.at_ok("ATSH6F1");
        let _ = self.at_ok("ATFCSH6F1");
        let _ = self.at_ok("ATFCSM1");
        for (name, addr, rx) in extended {
            let _ = self.at_ok(&format!("ATCEA{addr}"));
            let _ = self.at_ok(&format!("ATCRA{rx}"));
            let _ = self.at_ok(&format!("ATFCSD{addr}300000"));
            let raw = self.raw_obd("1902FF");
            println!("{name} ext {addr}: {}", raw.escape_debug());
            codes.extend(Self::decode_uds_dtcs(name, &raw));
        }
        let _ = self.at_ok("ATCEA");
        let _ = self.at_ok("ATAR");
        let _ = self.at_ok("ATFCSM0");
        let _ = self.at_ok("ATSH7DF");

        codes.sort();
        codes.dedup();
        println!("bmw codes: {codes:?}");
        codes
    }

    fn decode_sae_dtcs(module: &str, service: &str, raw: &str) -> Vec<String> {
        let bytes = Self::hex_bytes(raw);
        let mut out = Vec::new();
        let mut i = 0;
        while i + 1 < bytes.len() {
            if bytes[i] == 0x43 || bytes[i] == 0x47 || bytes[i] == 0x4A {
                i += 1;
                if i >= bytes.len() {
                    break;
                }
                let count = bytes[i] as usize;
                if count > 0 && count < 8 && i + 1 + count * 2 <= bytes.len() {
                    i += 1;
                    for _ in 0..count {
                        if bytes[i] == 0 && bytes[i + 1] == 0 {
                            i += 2;
                            continue;
                        }
                        out.push(format!(
                            "{module} mode {service}: {}",
                            Self::sae_code(bytes[i], bytes[i + 1])
                        ));
                        i += 2;
                    }
                    continue;
                }
            }
            i += 1;
        }
        out
    }

    fn decode_uds_dtcs(module: &str, raw: &str) -> Vec<String> {
        let bytes = Self::hex_bytes(raw);
        let mut out = Vec::new();
        let mut i = 0;
        while i + 3 < bytes.len() {
            if bytes[i] == 0x59 && bytes[i + 1] == 0x02 {
                i += 3; // 59 02, then status availability mask
                while i + 3 < bytes.len() {
                    if bytes[i] == 0 && bytes[i + 1] == 0 && bytes[i + 2] == 0 {
                        break;
                    }
                    let code = if bytes[i + 2] == 0 {
                        Self::sae_code(bytes[i], bytes[i + 1])
                    } else {
                        format!("{:02X}{:02X}{:02X}", bytes[i], bytes[i + 1], bytes[i + 2])
                    };
                    out.push(format!(
                        "{module} mode 19: {code} status {:02X}",
                        bytes[i + 3]
                    ));
                    i += 4;
                }
                break;
            }
            i += 1;
        }
        out
    }

    fn sae_code(a: u8, b: u8) -> String {
        let class = ["P", "C", "B", "U"][(a >> 6) as usize];
        format!("{class}{:01X}{:01X}{:02X}", (a >> 4) & 0x03, a & 0x0F, b)
    }

    fn hex_bytes(raw: &str) -> Vec<u8> {
        raw.split_whitespace()
            .filter_map(|token| u8::from_str_radix(token, 16).ok())
            .collect()
    }


    /// Clears emissions codes (mode 04) and UDS codes (14 FF FF FF) on each module.
    /// A mode 0A / 4A code is permanent and will come back until the monitor passes.
    pub fn clear_bmw_codes(&mut self) -> Vec<String> {
        if self.replay_requests {
            return Vec::new();
        }
        let _ = self.at_ok("ATAR");
        let _ = self.at_ok("ATFCSM0");
        let mut results = Vec::new();
        for (name, header) in [("DME", "7E0"), ("EGS", "7E1"), ("DSC", "7E2")] {
            let _ = self.at_ok(&format!("ATSH{header}"));
            let mode04 = self.raw_obd("04");
            println!("{name} clear mode 04: {}", mode04.escape_debug());
            results.push(format!("{name} mode 04: {}", mode04.replace('\r', " ").replace('\n', " ")));
            let uds = self.raw_obd("14FFFFFF");
            println!("{name} clear mode 14: {}", uds.escape_debug());
            results.push(format!("{name} mode 14: {}", uds.replace('\r', " ").replace('\n', " ")));
        }
        let _ = self.at_ok("ATSH7DF");
        results
    }

}

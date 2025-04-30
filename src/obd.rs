use serialport::SerialPort;
use std::fmt::Display;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use crate::cmd::CommandType;
use crate::{cmd::Command, response::Response};

#[derive(Debug)]
pub enum BankNumber {
    Bank1,
    Bank2,
}

#[derive(Debug)]
pub enum OxygenSensor {
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
pub enum OBDError {
    ConnectionFailed,
    NoConnection,

    InitFailed,

    InvalidResponse,
    InvalidCommand,
    ECUUnavailable,
    ELM327WriteError,
}

impl Display for OBDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "obd error; ");
        match *self {
            OBDError::InvalidResponse => writeln!(f, "invalid response from ecu."),
            OBDError::InvalidCommand => writeln!(
                f,
                "an invalid user command was going to be sent to the ecu."
            ),
            OBDError::NoConnection => writeln!(f, "no serial connection active."),
            OBDError::ECUUnavailable => writeln!(f, "ecu not available."),
            OBDError::ELM327WriteError => writeln!(f, "error writing through serial connection."),
            OBDError::ConnectionFailed => {
                writeln!(f, "failed to establish connection with elm327.")
            }
            OBDError::InitFailed => writeln!(f, "failed to initialize obd with ecu."),
        }
    }
}

pub struct OBD {
    // connection: Option<TcpStream>,
    connection: Option<Box<dyn SerialPort>>,

    // ECU name to process data for
    // When none. Use data from the first ecu that responds
    // and ecu data is discarded
    relevant_ecu: Option<String>,
}

impl OBD {
    pub fn new() -> Self {
        Self {
            connection: None,
            relevant_ecu: None,
        }
    }

    pub fn connect(&mut self, port: &str, baud_rate: u32) -> Result<(), OBDError> {
        if self.connection.is_some() {
            return Ok(());
        }

        self.connection = match serialport::new(port, baud_rate)
            .timeout(Duration::from_secs(10))
            .open()
        {
            Ok(port) => Some(port),
            Err(_) => return Err(OBDError::ConnectionFailed),
        };

        self.init()
    }

    pub fn init(&mut self) -> Result<(), OBDError> {
        let commands = vec![
            Command::new_at(b"ATZ"),
            Command::new_at(b"ATE0"),
            Command::new_at(b"ATL0"),
            Command::new_at(b"ATH1"),
            Command::new_at(b"ATSP0"),
        ];

        for command in commands {
            thread::sleep(Duration::from_millis(10));
            match self.send_command(&command) {
                Ok(()) => {}
                Err(err) => {
                    println!(
                        "when sending AT command: {} - {}",
                        err,
                        String::from_utf8_lossy(command.get_at())
                    );
                    return Err(err);
                }
            }

            match self.get_at_response() {
                Ok(response) => {
                    println!(
                        "{} response: {}",
                        String::from_utf8_lossy(command.get_at()),
                        response.raw_response.unwrap_or_default()
                    );
                }
                Err(err) => {
                    println!("when receiving AT command: {}", err,);
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    pub fn use_default_ecu(&mut self) {
        self.relevant_ecu = Some("default".to_owned())
    }

    // If 'ecu_name' isn't found in a response a
    // warnign message will be printed and the
    // first ecu will be used for data in that feature.
    pub fn use_ecu(&mut self, ecu_name: String) {
        self.relevant_ecu = Some(ecu_name)
    }

    // todo: inefficient...
    pub fn get_supported_pids(&mut self) -> Vec<[u8; 2]> {
        let response = self.query(Command::new_pid(b"0100")).unwrap_or_default();
        let binding = response.get_payload().unwrap_or_default().replace(" ", "");
        let split: Vec<&str> = binding.split("4100").collect();
        println!("bind: {split:?}");
        let mut pid = 1;
        let mut supported_pids: Vec<String> = Vec::new();
        for response in split {
            for ch in response.chars() {
                let as_num = u8::from_str_radix(ch.to_string().as_str(), 16).unwrap();
                let bits = format!("{:04b}", as_num);
                println!("Bits for {as_num} ({ch}): {bits}");
                for bit in bits.chars() {
                    print!("\t{bit} {pid} - ");
                    if bit == '1' {
                        // value not found
                        supported_pids.push(format!("{pid:02X}"));
                        println!("set")
                    } else {
                        println!("not")
                    }
                    
                    pid+=1;
                }
            }
            pid = 1;
        }
        supported_pids.sort();
        supported_pids.dedup();
        println!("{supported_pids:?}");

        Vec::new()
    }

    pub fn send_command(&mut self, req: &Command) -> Result<(), OBDError> {
        let stream = match &mut self.connection {
            Some(stream) => stream,
            None => return Err(OBDError::NoConnection),
        };

        let _ = stream.clear(serialport::ClearBuffer::All);

        let mut cmd = match req.command_type() {
            CommandType::PIDCommand => req.get_pid().to_vec(),
            CommandType::ATCommand => req.get_at().to_vec(),
            CommandType::ServiceQuery => req.get_svc().to_vec(),
            _ => return Err(OBDError::InvalidCommand),
        };

        cmd.push(b'\r');

        if stream.write_all(&cmd).is_err() {
            return Err(OBDError::ELM327WriteError);
        }

        Ok(())
    }

    pub fn get_at_response(&mut self) -> Result<Response, OBDError> {
        let mut response: String = match self.read_until(b'>') {
            Ok(response) => response,
            Err(err) => {
                println!("when reading AT response - {} ", err);
                return Err(err);
            }
        };

        response = response.replace("\r", "");

        let mut meta_data = Response::default();
        meta_data.raw_response = Some(response);

        Ok(meta_data)
    }

    pub fn get_pid_response(&mut self) -> Result<Response, OBDError> {
        let mut response: String = match self.read_until(b'>') {
            Ok(response) => response,
            Err(err) => {
                println!("when reading pid response - {} ", err);
                return Err(err);
            }
        };

        response = response.replace("SEARCHING...", "").replace("\r", "\n");

        let mut payload_size: u32 = 0;
        let ecu_names: Vec<String> = response
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

                payload_size = u32::from_str_radix(split[1], 16).unwrap_or(0);

                Some(ecu_name)
            })
            .collect();
        let ecu_count = ecu_names.len();
        if ecu_count == 0 {
            return Ok(Response::default());
        }

        println!("{response}");
        response = response
            .replace(" ", "")
            .replace("\n", "")
            .replace(format!("{:02X}", payload_size).as_str(), "");

        // remove ecu names
        for ecu_name in ecu_names.iter() {
            response = response.replace(ecu_name.as_str(), "")
        }

        println!("\t{response}, {}", response.len());
        println!("{ecu_names:?}");
        println!("{payload_size}");

        if response.len() < 2 {
            return Err(OBDError::InvalidResponse);
        } else if response.contains("NODATA") {
            return Ok(Response::default());
        }

        let parsed = Self::format_response(&response);
        let no_whitespace = parsed.replace(" ", "");
        let as_bytes = no_whitespace.as_bytes();
        let mut meta_data = Response::default();
        meta_data.ecu_count = ecu_count;
        meta_data.raw_response = Some(parsed.clone());
        meta_data.payload_size = payload_size as usize;
        meta_data.service = [as_bytes[0], as_bytes[1]];
        meta_data.pid = [as_bytes[2], as_bytes[3]];
        meta_data.payload = Some(meta_data.payload_from_response());

        Ok(meta_data)
    }

    pub(crate) fn read_until(&mut self, until: u8) -> Result<String, OBDError> {
        let port = match &mut self.connection {
            Some(port) => port,
            None => return Err(OBDError::NoConnection),
        };

        let _ = port.clear(serialport::ClearBuffer::All);

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

        Ok(response)
    }

    pub(crate) fn format_response(response: &str) -> String {
        let chunks = response
            .as_bytes()
            .chunks(2)
            .map(|pair| std::str::from_utf8(pair).unwrap_or(""))
            .collect::<Vec<&str>>();
        let as_string = chunks.join(" ");

        as_string
    }

    pub(crate) fn query(&mut self, request: Command) -> Option<Response> {
        match self.send_command(&request) {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "{}\tAT: '{}' - PID: '{}' ",
                    err,
                    String::from_utf8_lossy(request.get_at()),
                    String::from_utf8(request.get_pid().to_vec()).unwrap_or_default()
                );
                return None;
            }
        };

        match self.get_pid_response() {
            Ok(response) => Some(response),
            Err(err) => {
                println!(
                    "{}\tAT: '{}' - PID: '{}' ",
                    err,
                    String::from_utf8_lossy(request.get_at()),
                    String::from_utf8(request.get_pid().to_vec()).unwrap_or_default()
                );
                return None;
            }
        }
    }
}

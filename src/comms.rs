use std::borrow::Cow;

pub struct PID {
    pub pid_hex: [u8;2],
    pub service_num: [u8;2],
    command: [u8; 4],         // service_num + pid_hex
    response: Option<String>, // Hex Response from ECU
    bytes: u8,                // How many bytes in the response
}

impl PID {
    pub fn new() -> Self {
        Self {
            pid_hex: [0;2],
            service_num: [0;2],
            command: [0; 4],
            response: None,
            bytes: 0,
        }
    }

    pub fn cmd(&self) -> String {
        let command = [self.service_num, self.pid_hex].concat();
        println!("{:?}", command);
        String::from_utf8_lossy(&command).to_string()
    }

    // TODO:
    fn a_value() -> f32 {
        0f32
    }

    fn b_value() -> f32 {
        0f32
    }

    fn c_value() -> f32 {
        0f32
    }

    fn d_value() -> f32 {
        0f32
    }
}

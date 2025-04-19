use crate::cmd::Command;

pub struct PID {
    command: Command,
    response: Option<String>, // Hex Response from ECU
    bytes: u8,                // How many bytes in the response
}

impl PID {
    pub fn new() -> Self {
        Self {
            command: Command::default(),
            response: None,
            bytes: 0,
        }
    }

    pub fn set_command(&mut self, cmd: &[u8; 4]) -> bool {
        self.command.set_command(cmd)
    }

    pub fn get_command(&self) -> String {
        let command = self.command.get_command();
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

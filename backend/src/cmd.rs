use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum CommandType {
    PIDCommand,
    ATCommand,
    ServiceQuery,
    Arbitrary,
    Default,
}

impl Default for CommandType {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug)]
pub struct Command {
    command_type: CommandType,
    pid_command: [u8; 4],
    at_command: &'static [u8],
    svc_command: [u8; 2],
    arbitrary_message: String,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            command_type: CommandType::Default,
            pid_command: [0u8; 4],
            at_command: &[],
            svc_command: [0u8; 2],
            arbitrary_message: String::new(),
        }
    }
}

impl Command {
    pub fn new_pid(command: &[u8; 4]) -> Self {
        Self {
            command_type: CommandType::PIDCommand,
            pid_command: *command,
            ..Default::default()
        }
    }

    pub fn new_at(at_command: &'static [u8]) -> Self {
        Self {
            command_type: CommandType::ATCommand,
            at_command,
            ..Default::default()
        }
    }

    pub fn new_svc(svc_command: &[u8; 2]) -> Self {
        Self {
            command_type: CommandType::ServiceQuery,
            svc_command: *svc_command,
            ..Default::default()
        }
    }

    pub fn new_arb(arbitrary_msg: &str) -> Self {
        Self {
            command_type: CommandType::Arbitrary,
            arbitrary_message: arbitrary_msg.to_owned(),
            ..Default::default()
        }
    }

    pub fn set_pid(&mut self, command: &[u8; 4]) {
        if self.command_type == CommandType::Default {
            self.command_type = CommandType::PIDCommand;
        }

        if self.command_type != CommandType::PIDCommand {
            return;
        }

        self.pid_command = command.to_owned();
        self.at_command = &[];
    }

    pub fn set_at(&mut self, at_command: &'static [u8]) -> bool {
        if self.command_type == CommandType::Default {
            self.command_type = CommandType::ATCommand;
        }

        if self.command_type != CommandType::ATCommand {
            return false;
        }

        if at_command.len() < 3 {
            println!("at_command to set is invalid. length less than 3.");
            return false;
        }

        self.pid_command = [0u8; 4];
        self.at_command = at_command;
        true
    }

    pub fn set_svc(&mut self, command: &[u8; 2]) {
        if self.command_type == CommandType::Default {
            self.command_type = CommandType::ServiceQuery;
        }

        if self.command_type != CommandType::ServiceQuery {
            return;
        }
        self.svc_command = *command;
    }

    pub fn get_pid(&self) -> [u8; 4] {
        self.pid_command
    }

    pub fn get_at(&self) -> &[u8] {
        self.at_command
    }

    pub fn get_svc(&self) -> [u8; 2] {
        self.svc_command
    }

    pub fn get_msg(&self) -> String {
        self.arbitrary_message.clone()
    }

    pub fn as_string(&self) -> String {
        match String::from_utf8(self.as_bytes()) {
            Ok(string) => string,
            Err(err) => panic!("UTF-8 error converting command to string. {err}"),
        }
    }

    /// Get the command as a Vector of bytes.
    /// e.g: if at_command is in use, return it as Vec<u8>
    pub fn as_bytes(&self) -> Vec<u8> {
        match self.command_type() {
            CommandType::PIDCommand => self.get_pid().to_vec(),
            CommandType::ATCommand => self.get_at().to_vec(),
            CommandType::ServiceQuery => self.get_svc().to_vec(),
            CommandType::Arbitrary => self.get_msg().as_bytes().to_vec(),
            _ => Vec::new(),
        }
    }

    pub(crate) fn command_type(&self) -> &CommandType {
        &self.command_type
    }
}

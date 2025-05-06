#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum CommandType {
    PIDCommand,
    ATCommand,
    ServiceQuery,
    Arbitrary,
    Default,
    Unknown,
}

impl Default for CommandType {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Default, Debug)]
pub struct Command {
    command_type: CommandType,
    pid_command: [u8; 4],
    at_command: &'static [u8],
    svc_command: [u8; 2],
    arbitrary_message: String,
}

impl Command {
    pub fn new_pid(command: &[u8; 4]) -> Self {
        Command {
            command_type: CommandType::PIDCommand,
            pid_command: *command,
            at_command: &[],
            svc_command: [0u8; 2],
            arbitrary_message: String::default(),
        }
    }

    pub fn new_at(at_command: &'static [u8]) -> Self {
        let mut s = Self::default();
        s.command_type = CommandType::ATCommand;
        if s.set_at(at_command) {
            return s;
        }
        Command::default()
    }

    pub fn new_svc(svc_command: &[u8; 2]) -> Self {
        Command {
            command_type: CommandType::ServiceQuery,
            pid_command: [0u8; 4],
            at_command: &[],
            svc_command: *svc_command,
            arbitrary_message: String::default(),
        }
    }

    pub(crate) fn new_arb(arbitrary_msg: &str) -> Self {
        Command {
            command_type: CommandType::Arbitrary,
            pid_command: [0u8; 4],
            at_command: &[],
            svc_command: [0u8; 2],
            arbitrary_message: arbitrary_msg.to_owned(),
        }
    }

    pub fn command_type(&self) -> CommandType {
        self.command_type
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
            self.command_type = CommandType::Default;
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
}

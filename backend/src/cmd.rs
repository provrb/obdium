use serde::{Deserialize, Serialize};

/// The type of command to send via OBD
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub(crate) enum CommandType {
    /// Regular PID command, e.g 010C
    PIDCommand,
    /// An ELM AT command, e.g ATZ
    ATCommand,
    /// A OBD service query, e.g 09
    ServiceQuery,
    /// An arbitrary message that doesn't fall into any other specific command category
    /// e.g, this is used for Mode 22 PIDs because PIds are strictly 4 characters (the service # and PID #), but 
    /// an arbitrary command payload can be any length, useful for 6 character mode 22 pids (22xxyy)
    Arbitrary,
    #[default]
    Default,
}

/// The main Command abstraction layer to send over and use with the `OBD` struct 
#[derive(Debug, Default)]
pub struct Command {
    command_type: CommandType,
    /// If command_type is PIDCommand, contains the 4 character PID 
    pid_command: [u8; 4],
    /// If command_type is ATCommand, contains the ATxx command
    at_command: &'static [u8],
    /// If command_type is ServiceQuery, contans the 2 character service #/mode #
    svc_command: [u8; 2],
    /// If command_type is Arbitrary, contains a variable length string
    arbitrary_message: String,
}

impl Command {
    /// Create a new generic PID command and returns the `Command``
    /// struct with command_type: PIDCommand.
    /// 
    /// `command` - The 4 character PID command, e.g: 010C (Engine RPM PID) 
    pub fn new_pid(command: &[u8; 4]) -> Self {
        Self {
            command_type: CommandType::PIDCommand,
            pid_command: *command,
            ..Default::default()
        }
    }

    /// Create a new AT ELM command and returns the `Command`
    /// struct with command_type: ATCommand.
    /// 
    /// `at_command` - The variable length AT command, e.g: ATZ
    pub fn new_at(at_command: &'static [u8]) -> Self {
        Self {
            command_type: CommandType::ATCommand,
            at_command,
            ..Default::default()
        }
    }

    /// Create a new service query command and returns the `Command`
    /// struct with command_type: ServiceQuqery.
    /// 
    /// `svc_command` - The 2 character number of the service/mode to query, e.g 09
    pub fn new_svc(svc_command: &[u8; 2]) -> Self {
        Self {
            command_type: CommandType::ServiceQuery,
            svc_command: *svc_command,
            ..Default::default()
        }
    }

    /// Create a new arbitrary command and returns the `Command`
    /// struct with command_type: Arbitrary
    /// 
    /// `arbitrary_msg` - The string to store in the `arbitrary_message` field of the command
    pub fn new_arb(arbitrary_msg: &str) -> Self {
        Self {
            command_type: CommandType::Arbitrary,
            arbitrary_message: arbitrary_msg.to_owned(),
            ..Default::default()
        }
    }

    /// If this command is of command_type PIDCommand or Default,
    /// set the underlying pid_command field to `command`
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

    /// If this command is of command_type ATCommand or Default,
    /// set the underlying at_command field to `command`
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

    /// If this command is of command_type ServiceQuery or Default,
    /// set the underlying svc_command field to `command`
    pub fn set_svc(&mut self, command: &[u8; 2]) {
        if self.command_type == CommandType::Default {
            self.command_type = CommandType::ServiceQuery;
        }

        if self.command_type != CommandType::ServiceQuery {
            return;
        }
        self.svc_command = *command;
    }

    /// Get the underlying `pid_command` field. 
    /// Note: Ensure this command's command_type is PIDCommand to ensure you get
    /// the data you're looking for.
    pub fn get_pid(&self) -> [u8; 4] {
        self.pid_command
    }

    /// Get the underlying `at_command` field. 
    /// Note: Ensure this command's command_type is ATCommand to ensure you get
    /// the data you're looking for.
    pub fn get_at(&self) -> &[u8] {
        self.at_command
    }

    /// Get the underlying `svc_command` field. 
    /// Note: Ensure this command's command_type is ServiceQuery mand to ensure you get
    /// the data you're looking for.
    pub fn get_svc(&self) -> [u8; 2] {
        self.svc_command
    }

    /// Get the underlying `arbitrary_message` field. 
    /// Note: Ensure this command's command_type is Arbitrary to ensure you get
    /// the data you're looking for.
    pub fn get_msg(&self) -> String {
        self.arbitrary_message.clone()
    }

    /// Get the underlying, active command content as a string.
    /// Regardless of the command_type, this function will return the correct populated command field content
    /// as a string.
    /// This is the simplest and most correct way to get a `Command` struct's "command"
    pub fn as_string(&self) -> String {
        match String::from_utf8(self.as_bytes()) {
            Ok(string) => string,
            Err(err) => panic!("UTF-8 error converting command to string. {err}"),
        }
    }

    /// Get the occupied command field content as a Vector of bytes.
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

    /// Get the type of this command
    pub(crate) fn command_type(&self) -> &CommandType {
        &self.command_type
    }
}

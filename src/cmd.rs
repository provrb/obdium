#[derive(Default, Debug)]
pub struct Command {
    pid_command: [u8; 4],
    at_command: &'static [u8],
}

impl Command {
    pub fn new_pid(command: &[u8; 4]) -> Self {
        Command {
            pid_command: *command,
            at_command: &[],
        }
    }

    pub fn new_at(at_command: &'static [u8]) -> Self {
        Command {
            pid_command: [0u8; 4],
            at_command: at_command,
        }
    }

    pub fn set_at(&mut self, at_command: &'static [u8]) -> bool {
        if at_command.len() < 3 {
            println!("at_command to set is invalid. length less than 3.");
            return false;
        }

        self.pid_command = [0u8; 4];
        self.at_command = at_command;
        true
    }

    pub fn get_at(&self) -> &[u8] {
        self.at_command
    }

    pub fn set_pid(&mut self, command: &[u8; 4]) {
        self.pid_command = command.to_owned();
        self.at_command = &[];
    }

    pub fn get_pid(&self) -> [u8; 4] {
        self.pid_command
    }
}

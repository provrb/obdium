#[derive(Default)]
pub struct Command {
    pid_hex: [u8; 2],
    service_num: [u8; 2],
    command: [u8; 4],
}

impl Command {
    pub fn new(command: &[u8; 4]) -> Self {
        todo!()
    }

    pub fn set_command(&mut self, command: &[u8; 4]) -> bool {
        if !self.is_valid_command(&command) {
            return false;
        }

        self.command = command.to_owned();
        return true;
    }

    pub fn get_command(&self) -> [u8; 4] {
        self.command
    }

    pub fn is_valid_command(&self, command: &[u8; 4]) -> bool {
        let service = self.service_from_command(command);
        if !service.starts_with(b"0") {
            return false;
        }

        for c in service {
            println!("char: {}", c);
        }

        return true;
    }

    fn service_from_command(&self, command: &[u8; 4]) -> [u8; 2] {
        [command[0], command[1]]
    }
}

use std::io::{self, BufRead};

use obdium::cmd::Command;
use obdium::obd::{BankNumber, OBD};

fn main() -> std::io::Result<()> {
    let mut obd = OBD::new();
    // if !obd.connect("127.0.0.1", "5054") {
    //     return Err(std::io::Error::new(
    //         std::io::ErrorKind::Other,
    //         "Couldn't connect to TCP",
    //     ));
    // }

    if !obd.connect("/dev/ttyUSB0", 38400) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Couldn't connect to ELM327",
        ));
    }

    obd.init();
    obd.coolant_temp();

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        
        handle.read_line(&mut buffer)?;

        if let Some(connection) = &mut obd.connection {
            OBD::send_string(buffer.as_bytes(), connection);
        }
    }

    Ok(())
}

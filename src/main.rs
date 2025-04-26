use std::io::{self, BufRead};
use std::time::Duration;

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
    std::thread::sleep(Duration::from_secs(1));
    obd.coolant_temp();
    std::thread::sleep(Duration::from_secs(1));
    obd.fuel_pressure();
    std::thread::sleep(Duration::from_secs(1));
    obd.engine_load();

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

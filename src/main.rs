use obdium::obd::{BankNumber, OBDError, OBD};
use std::time::Duration;

fn main() -> Result<(), OBDError> {
    let mut obd = OBD::new();
    // obd.connect("COM4", 38400)?;
    obd.connect("/dev/ttyUSB0", 38400)?;

    std::thread::sleep(Duration::from_secs(1));
    println!("Coolant temperature: {}C", obd.coolant_temp());
    println!("Engine load: {}%", obd.engine_load());
    println!("Fuel pressure: {}kPa", obd.fuel_pressure());
    println!("RPM: {}", obd.rpm());
    println!("Short term fuel trim (Bank1): {}", obd.short_term_fuel_trim(BankNumber::Bank1));
    println!("Short term fuel trim (Bank2): {}", obd.short_term_fuel_trim(BankNumber::Bank2));
    

    Ok(())
}

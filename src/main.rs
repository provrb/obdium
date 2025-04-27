use obdium::obd::{OBDError, OBD};
use std::time::Duration;

fn main() -> Result<(), OBDError> {
    let mut obd = OBD::new();
    obd.connect("COM4", 38400)?;

    std::thread::sleep(Duration::from_secs(1));
    println!("Coolant temperature: {}C", obd.coolant_temp());
    println!("Engine load: {}%", obd.engine_load());
    println!("Fuel pressure: {}kPa", obd.fuel_pressure());

    Ok(())
}

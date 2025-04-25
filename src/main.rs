use obdium::cmd::Command;
use obdium::obd::{BankNumber, OBD};

fn main() -> std::io::Result<()> {
    let mut obd = OBD::new();
    if !obd.connect("/dev/ttyUSB0", 38400) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Couldn't connect to ELM327",
        ));
    }

    // println!("RPM: {}", obd.rpm());
    // println!("Engine load: {}%", obd.engine_load());
    println!("Coolant temperature: {}°C", obd.coolant_temp());
    // println!("\nShort term fuel trim: ");
    // println!("Bank 1: {}%", obd.short_term_fuel_trim(BankNumber::Bank1));
    // println!("Bank 2: {}%", obd.short_term_fuel_trim(BankNumber::Bank2));

    // println!("\nLong term fuel trim: ");
    // println!("Bank 1: {}%", obd.long_term_fuel_trim(BankNumber::Bank1));
    // println!("Bank 2: {}%", obd.long_term_fuel_trim(BankNumber::Bank2));

    // println!("Fuel Pressure: {}kPa", obd.fuel_pressure());

    Ok(())
}

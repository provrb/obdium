use obdium::obd::{Error, OBD};

fn main() -> Result<(), Error> {
    // Connecting via ELM327
    // OBD::connect takes two parameters
    // Port and baud_rate
    // Usually baud_rate is 38400.
    let mut obd = OBD::new();
    obd.connect("COM4", 38400)?;

    // Get a list of supported pids from each ECU
    // Hashmap.
    // ECU Name -> { SupportedPids }
    obd.get_service_supported_pids("01");

    // Get a list of diagnostic trouble codes
    // Contains the category, code and description
    let codes = obd.get_trouble_codes();
    for code in codes {
        println!("{}\n", code);
    }

    // Misc information
    println!("OBD standard: {}", obd.obd_standards());
    println!("Auxiliary input status: {}", obd.aux_input_status());
    println!("Control module voltage: {}V", obd.control_module_voltage());
    println!(
        "Distance traveled with malfunction indicator lamp: {}km",
        obd.distance_traveled_with_mil()
    );
    println!(
        "TIme run with malfunction indicator lamp: {}s",
        obd.time_run_with_mil()
    );

    Ok(())
}

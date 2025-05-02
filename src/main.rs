use obdium::{
    obd::{BankNumber, OBDError, OxygenSensor, OBD},
    pid::diagnostics::MILStatus,
};
use std::time::Duration;

fn main() -> Result<(), OBDError> {
    let mut obd = OBD::new();
    //obd.connect("COM4", 38400)?;
    //obd.connect("/dev/ttyUSB0", 38400)?;
    obd.connect("/dev/pts/3", 38400)?;

    std::thread::sleep(Duration::from_secs(1));//obd.connect("/dev/ttyUSB0", 38400)?;

    println!("\n{} DIAGNOSTICS {}", "=".repeat(24), "=".repeat(24));
    let supported_pids =  obd.get_supported_pids();

    println!("Supported pids for service 01 [01-E0] ({}):", supported_pids.len());
    for supported_pid in supported_pids.iter() {
        println!("\tPID: {supported_pid}");
    }

    println!(
        "Check engine light: {}",
        if obd.get_mil_status() == MILStatus::On {
            "On"
        } else {
            "Off"
        }
    );
    println!("Number of trouble codes: {}", obd.get_num_trouble_codes());

    let codes = obd.get_trouble_codes();
    for code in codes {
        println!("{}\n", code);
    }

    println!("OBD standard: {:?}", obd.obd_standards());
    println!("Auxiliary input status: {:?}", obd.aux_input_status());
    println!(
        "Distance traveled with malfunction indicator lamp: {}km",
        obd.distance_traveled_with_mil()
    );
    println!("Control module voltage: {}V", obd.control_module_voltage());
    println!(
        "Time since codes cleared: {}s",
        obd.time_since_codes_cleared()
    );
    println!("Engine speed: {}RPM", obd.rpm());
    println!("Engine load: {}%", obd.engine_load());
    println!("Coolant temperature: {}C", obd.coolant_temp());
    println!("Engine fuel rate: {}L/h", obd.engine_fuel_rate());
    println!("Engine runtime: {}s", obd.engine_runtime());
    println!("Engine mileage: {}km", obd.engine_mileage());
    println!("Engine oil temperature: {}°C", obd.engine_oil_temp());
    println!(
        "Drivers demand engine torque: {}%",
        obd.drivers_demand_engine_torque()
    );
    println!("Actual engine torque: {}%", obd.actual_engine_torque());
    println!(
        "Reference engine torque: {}%",
        obd.reference_engine_torque()
    );

    let percent_torque_data = obd.engine_percent_torque_data();
    println!("Engine percent torque data:");
    println!("\tIdle: {}%", percent_torque_data.0);
    println!("\tEngine point 1: {}%", percent_torque_data.1);
    println!("\tEngine point 2: {}%", percent_torque_data.2);
    println!("\tEngine point 3: {}%", percent_torque_data.3);
    println!("\tEngine point 4: {}%", percent_torque_data.4);

    println!("\n{} FUEL SYSTEM {}", "=".repeat(24), "=".repeat(24));

    println!("Short term fuel trim:");
    println!("\tBank 1: {}", obd.short_term_fuel_trim(BankNumber::Bank1));
    println!("\tBank 2: {}", obd.short_term_fuel_trim(BankNumber::Bank2));

    println!("Long term fuel trim:");
    println!("\tBank 1: {}", obd.long_term_fuel_trim(BankNumber::Bank1));
    println!("\tBank 2: {}", obd.long_term_fuel_trim(BankNumber::Bank2));

    println!("Fuel pressure: {}kPa", obd.fuel_pressure());
    println!("Fuel tank level: {}%", obd.fuel_tank_level());
    println!("Fuel rail pressure: {}kPa", obd.fuel_rail_pressure());
    println!(
        "Fuel rail gauge pressure: {}kPa",
        obd.fuel_rail_guage_pressure()
    );
    println!("Fuel type: {:?}", obd.fuel_type());
    println!(
        "Ethanol fuel percentage: {}%",
        obd.ethanol_fuel_percentage()
    );
    println!("Fuel injection timing: {}°", obd.fuel_injection_timing());

    println!("\n{} SENSOR DATA {}", "=".repeat(24), "=".repeat(24));
    println!("Vehicle speed: {}km/h", obd.vehicle_speed());
    println!(
        "Timing advance: {}° before top-dead-center",
        obd.timing_advance()
    );
    println!("Intake air temperature: {}°C", obd.intake_air_temp());
    println!("Mass air-flow sensor rate: {}g/s", obd.maf_air_flow_rate());
    println!("Ambient air temperature: {}°C", obd.ambient_air_temp());
    println!(
        "Maximum air-flow rate from mass air-flow sensor: {}g/s",
        obd.max_air_flow_rate_from_maf()
    );
    println!("Throttle position: {}%", obd.throttle_position());

    // Read oxcygen sensors 1-8
    let sensors = [
        OxygenSensor::Sensor1,
        OxygenSensor::Sensor2,
        OxygenSensor::Sensor3,
        OxygenSensor::Sensor4,
        OxygenSensor::Sensor5,
        OxygenSensor::Sensor6,
        OxygenSensor::Sensor7,
        OxygenSensor::Sensor8,
    ];

    for sensor in sensors.iter() {
        let mut sensor_data = obd.read_oxygen_sensor(sensor);
        println!("Oxygen sensor {:?}:", sensor);
        println!("\tVoltage: {}v", sensor_data.0);
        println!("\tShort term fuel trim: {}", sensor_data.1);

        sensor_data = obd.o2_sensor_air_fuel_ratio(sensor);
        println!("\tAir-fuel equivalance ratio: {}", sensor_data.0);
        println!("\tVoltage: {}", sensor_data.1);
    }

    let maf = obd.read_mass_air_flow_sensor();
    println!("Mass air flow sensor:");
    println!("\tSensor A: {}g/s", maf.0);
    println!("\tSensor B: {}g/s", maf.1);

    let max_values_for = obd.max_values_for();
    println!("Maximum values for:");
    println!("\tFuel-air equivalance ratio: {}", max_values_for.0);
    println!("\tOxygen sensor voltage: {}V", max_values_for.1);
    println!("\tOxygen sensor current: {}mA", max_values_for.2);
    println!(
        "\tIntake manifold absolute pressure: {}kPa",
        max_values_for.3
    );

    println!(
        "Relative throttle position: {}%",
        obd.relative_throttle_pos()
    );

    Ok(())
}

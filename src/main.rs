use obdium::obd::{BankNumber, OBDError, SensorNumber, Service, OBD};
use obdium::vin::parser::VIN;

fn main() -> Result<(), OBDError> {
    let vin = VIN::new("KL4CJASB6JB660929");
    let wmi = vin.get_wmi();
    let key = vin.as_key();
    let wmi_id = *vin.get_wmi_id().unwrap();
    let model_year = vin.get_model_year().unwrap() as i64;
    let schema_id = vin.get_schema_id(wmi_id, model_year).unwrap();
    let vehicle_type_id = vin.get_vehicle_type_id(wmi).unwrap();
    let make_id = vin.get_make_id(wmi).unwrap();
    let model_id = vin.get_model_id(schema_id).unwrap();
    let vspec_schema_id = vin.get_vspec_schema_id(model_id, make_id).unwrap();
    let vspec_pattern_id = vin
        .get_vspec_pattern_id(vspec_schema_id, schema_id)
        .unwrap();
    println!("Model year: {}", model_year);
    println!("WMI: {}", wmi);
    println!("WMI ID: {}", wmi_id);
    println!("Key: {}", key);
    println!("Truck type id: {}", vin.get_truck_type_id(wmi).unwrap());
    println!(
        "Vehicle type id: {}",
        vin.get_vehicle_type_id(wmi).unwrap()
    );
    println!(
        "Schema ID: {}",
        vin.get_schema_id(wmi_id, model_year).unwrap()
    );
    println!("Engine model: {}", vin.get_engine_model(schema_id).unwrap());
    println!(
        "Cylinder count: {}",
        vin.get_cylinder_count(schema_id).unwrap()
    );
    println!(
        "Engine displacement (L): {}",
        vin.get_engine_displacement(schema_id).unwrap()
    );
    println!("Fuel type: {}", vin.get_fuel_type(schema_id).unwrap());
    println!(
        "Valve train design: {}",
        vin.get_valve_train_design(schema_id).unwrap()
    );
    println!(
        "Fuel delivery type: {}",
        vin.get_fuel_delivery_type(schema_id).unwrap()
    );
    println!("Turbo: {}", vin.has_turbo(schema_id).unwrap());
    println!(
        "Engine manufacturer: {}",
        vin.get_engine_manufacturer(schema_id).unwrap()
    );
    println!(
        "Vehicle model: {}",
        vin.get_vehicle_model(schema_id).unwrap()
    );
    println!("Vehicle Make: {}", vin.get_vehicle_make(make_id).unwrap());
    println!(
        "Vehicle type: {}",
        vin.get_vehicle_type(vehicle_type_id).unwrap()
    );
    println!("Plant city: {}", vin.get_plant_city(schema_id).unwrap());
    println!(
        "Plant country: {}",
        vin.get_plant_country(schema_id).unwrap()
    );
    println!("Body class: {}", vin.get_body_class(schema_id).unwrap());
    println!("Vehicle spec schema id: {}", vspec_schema_id);
    println!("Vehicle spec pattern id: {}", vspec_pattern_id);
    println!("ABS: {}", vin.abs_availablility(vspec_pattern_id).unwrap());
    println!(
        "Airbag locations curtain: {}",
        vin.airbag_locations_curtain(schema_id).unwrap()
    );
    println!(
        "Airbag locations front: {}",
        vin.airbag_locations_front(schema_id).unwrap()
    );
    println!(
        "Airbag locations knee: {}",
        vin.airbag_locations_knee(schema_id).unwrap()
    );
    println!(
        "Airbag locations side: {}",
        vin.airbag_locations_side(schema_id).unwrap()
    );
    println!(
        "Transmission style: {}",
        vin.get_transmission_style(vspec_pattern_id).unwrap()
    );
    println!(
        "Steering location: {}",
        vin.get_steering_location(vspec_pattern_id).unwrap()
    );
    println!(
        "Keyless ignition: {}",
        vin.keyless_ignition_availability(vspec_pattern_id).unwrap()
    );
    println!("Drive type: {}", vin.get_drive_type(schema_id).unwrap());
    println!(
        "Axle count: {}",
        vin.get_axle_count(vspec_pattern_id).unwrap()
    );
    println!("Brake system: {}", vin.get_brake_system(schema_id).unwrap());
    println!(
        "ESC: {}",
        vin.electronic_stability_control(vspec_pattern_id).unwrap()
    );
    println!(
        "Traction control: {}",
        vin.traction_control(vspec_pattern_id).unwrap()
    );
    println!(
        "Auto-reverse system: {}",
        vin.windows_auto_reverse(vspec_pattern_id).unwrap()
    );
    println!(
        "Gross vehicle weight rating: {}",
        vin.get_vehicle_weight_rating(schema_id).unwrap()
    );
    println!(
        "Plant company: {}",
        vin.get_plant_company(schema_id).unwrap()
    );
    println!("Plant state: {}", vin.get_plant_state(schema_id).unwrap());
    println!(
        "Top speed: {}MPH",
        vin.get_vehicle_top_speed(vspec_pattern_id).unwrap()
    );
    println!(
        "Front wheel size: {}in",
        vin.get_front_wheel_size(vspec_pattern_id).unwrap()
    );
    println!(
        "Rear wheel size: {}in",
        vin.get_rear_wheel_size(vspec_pattern_id).unwrap()
    );
    println!(
        "Dynamic brake support: {}",
        vin.dynamic_brake_support(vspec_pattern_id).unwrap()
    );
    println!(
        "Backup camera: {}",
        vin.backup_camera(vspec_pattern_id).unwrap()
    );
    println!(
        "Automatic crash notification: {}",
        vin.automatic_crash_notification(vspec_pattern_id).unwrap()
    );
    println!(
        "Daytime running light: {}",
        vin.daytime_running_light(vspec_pattern_id).unwrap()
    );
    println!(
        "Semi-automatic headlamp beam switching: {}",
        vin.semiauto_headlamp_beam_switching(vspec_pattern_id)
            .unwrap()
    );
    println!(
        "Tranmission speeds: {}",
        vin.get_transmission_speeds(vspec_pattern_id).unwrap()
    );
    println!(
        "Vehicle base price: ${}",
        vin.get_vehicle_base_price(vspec_pattern_id).unwrap()
    );
    println!("Trim: {}", vin.vehicle_trim(schema_id).unwrap());
    println!("Seatbelt type: {}", vin.seatbelt_type(schema_id).unwrap());
    println!(
        "Number of rows: {}",
        vin.number_of_rows(vspec_pattern_id).unwrap()
    );
    println!(
        "Number of seats: {}",
        vin.number_of_seats(vspec_pattern_id).unwrap()
    );

    let mut obd = OBD::new();
    obd.connect("COM4", 38400)?;

    println!("\n{} DIAGNOSTICS {}", "=".repeat(24), "=".repeat(24));
    let supported_pids = obd.get_service_supported_pids("01");

    println!("Supported pids for ECUs");
    for (ecu_name, pids) in supported_pids.iter() {
        print!("ECU {ecu_name}:");
        for (index, pid) in pids.iter().enumerate() {
            if index % 10 == 0 {
                print!("\n\t");
            }

            print!("{pid} ");
            if index == pids.len() - 1 {
                println!()
            }
        }
    }

    println!("Check engine light: {}", obd.check_engine_light());
    println!("Number of trouble codes: {}", obd.get_num_trouble_codes());

    let codes = obd.get_trouble_codes();
    for code in codes {
        println!("{}\n", code);
    }

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
    println!(
        "Time since codes cleared: {}s",
        obd.time_since_codes_cleared()
    );
    println!(
        "Warm-ups since codes cleared: {}",
        obd.warm_ups_since_codes_cleared()
    );

    println!("\n{} ENGINE {}", "=".repeat(24), "=".repeat(24));
    println!("Engine type: {}", obd.get_engine_type());
    println!("Engine speed: {}RPM", obd.rpm());
    println!("Engine load: {}%", obd.engine_load());

    let coolant_temp = obd.coolant_temp_sensors();
    println!("Coolant temperature: {}°C", obd.coolant_temp());
    println!(
        "Coolant temperatue from sensors - Sensor 1: {}°C - Sensor 2: {}°C ",
        coolant_temp.0, coolant_temp.1
    );
    println!("Engine fuel rate: {}L/h", obd.engine_fuel_rate());
    println!("Engine runtime: {}s", obd.engine_runtime());
    println!("Engine runtime (diesel): {}s", obd.engine_runtime_diesel());
    println!("Engine mileage: {}km", obd.engine_mileage());

    let oil_temp = obd.engine_oil_temp_sensors();
    println!(
        "Engine oil temperature (Mode 1): {}°C",
        obd.engine_oil_temp(Service::Mode01)
    );
    println!(
        "Engine oil temperature (Mode 22): {}°C",
        obd.engine_oil_temp(Service::Mode22)
    );
    println!(
        "Engine oil temperatue from sensors - Sensor 1: {}°C - Sensor 2: {}°C ",
        oil_temp.0, oil_temp.1
    );
    println!("Engine oil pressure: {}kPa", obd.engine_oil_pressure()); // Mode 22 PID

    println!(
        "Drivers demand engine torque: {}%",
        obd.drivers_demand_engine_torque()
    );
    println!("Actual engine torque: {}%", obd.actual_engine_torque());
    println!(
        "Reference engine torque: {}Nm",
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

    let fuel_system_status = obd.fuel_system_status();
    println!("Fuel system status:");
    println!("Fuel system 1: {:?}", fuel_system_status.0);
    println!("Fuel system 2: {:?}", fuel_system_status.1);

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
    println!("Commanded EVAP purge: {}%", obd.commanded_evap_purge());
    println!(
        "EVAP system vapor pressure: {}Pa",
        obd.evap_system_vapor_pressure()
    );
    println!("Cylinder fuel rate: {}mg/stroke", obd.cylinder_fuel_rate());

    println!("\n{} SENSOR DATA {}", "=".repeat(24), "=".repeat(24));
    println!("Vehicle speed: {}km/h", obd.vehicle_speed());
    println!(
        "Timing advance: {}° before top-dead-center",
        obd.timing_advance()
    );

    println!("Throttle position: {}%", obd.throttle_position());
    println!(
        "Relative throttle position: {}%",
        obd.relative_throttle_pos()
    );
    println!(
        "Absolute throttle position B: {}%",
        obd.abs_throttle_position_b()
    );
    println!(
        "Absolute throttle position C: {}%",
        obd.abs_throttle_position_c()
    );
    println!(
        "Accelerator pedal position D: {}%",
        obd.acc_pedal_position_d()
    );
    println!(
        "Accelerator pedal position E: {}%",
        obd.acc_pedal_position_e()
    );
    println!(
        "Accelerator pedal position F: {}%",
        obd.acc_pedal_position_f()
    );

    // Read oxcygen sensors 1-8
    let sensors = [
        SensorNumber::Sensor1,
        SensorNumber::Sensor2,
        SensorNumber::Sensor3,
        SensorNumber::Sensor4,
        SensorNumber::Sensor5,
        SensorNumber::Sensor6,
        SensorNumber::Sensor7,
        SensorNumber::Sensor8,
    ];

    let mut data = Vec::new();

    for (i, sensor) in sensors.iter().enumerate() {
        let (voltage1, trim) = obd.read_oxygen_sensor(sensor);
        let (afr, voltage2) = obd.o2_sensor_air_fuel_ratio(sensor);
        data.push((i + 1, voltage1, trim, afr, voltage2));
    }

    println!("\n{} AIR DATA {}", "=".repeat(24), "=".repeat(24));

    println!("Oxygen Sensors:");
    println!(
        "{:<7} {:>11} {:>20} {:>20}",
        "Sensor", "Voltage", "Short Term Trim (%)", "AFR / Voltage"
    );

    for (sensor_id, v1, trim, afr, v2) in data {
        println!(
            "{:<7} {:>10.3}v {:>20.2} {:>12.3} / {:<7.3}",
            sensor_id, v1, trim, afr, v2
        );
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

    println!("Intake air temperature: {}°C", obd.intake_air_temp());
    println!("Mass air-flow sensor rate: {}g/s", obd.maf_air_flow_rate());
    println!("Ambient air temperature: {}°C", obd.ambient_air_temp());
    println!(
        "Maximum air-flow rate from mass air-flow sensor: {}g/s",
        obd.max_air_flow_rate_from_maf()
    );

    println!("Secondary air status: {:?}", obd.secondary_air_status());
    println!(
        "Absolute barometric pressure: {}kPa",
        obd.abs_barometric_pressure()
    );
    Ok(())
}

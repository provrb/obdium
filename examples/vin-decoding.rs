use obdium::vin::parser::VIN;

fn main() {
    // Create the VIN object.
    // A vin can only be assigned once per VIN object.
    let vin = VIN::new("KL4CJASB6JB660929");

    // Get the 'World Manufcaturer Identifier' from the VIN.
    // The first 3 characters of the vin. Contains
    // information about the region, vehicle type, and country.
    let wmi = vin.get_wmi();

    // Internal represenation of the WMI as an ID.
    // Non-developers do not need to worry about this.
    let wmi_id = vin.get_wmi_id().unwrap();

    // Get the year the vehicle was created.
    let model_year = vin.get_model_year().unwrap();

    // ID representation of the VIN number
    // used internally for SQLite querys to VPIC database.
    let schema_id = vin.get_vin_schema_id().unwrap();

    // ID's that can be used to retrieve specific vehicle specifications
    // Used internally as well
    let vspec_schema_id = vin.get_vspec_schema_id().unwrap();
    let vspec_pattern_id = vin.get_vspec_pattern_id().unwrap();

    println!("Model year: {}", model_year);
    println!("WMI: {}", wmi);
    println!("WMI ID: {}", wmi_id);
    println!("Truck type id: {}", vin.get_truck_type_id().unwrap());
    println!("Vehicle type id: {}", vin.get_vehicle_type_id().unwrap());
    println!("Schema ID: {}", schema_id);
    println!("Engine model: {}", vin.get_engine_model().unwrap());
    println!("Cylinder count: {}", vin.get_cylinder_count().unwrap());
    println!(
        "Engine displacement (L): {}",
        vin.get_engine_displacement().unwrap()
    );
    println!("Fuel type: {}", vin.get_fuel_type().unwrap());
    println!(
        "Valve train design: {}",
        vin.get_valve_train_design().unwrap()
    );
    println!(
        "Fuel delivery type: {}",
        vin.get_fuel_delivery_type().unwrap()
    );
    println!("Turbo: {}", vin.has_turbo().unwrap());
    println!(
        "Engine manufacturer: {}",
        vin.get_engine_manufacturer().unwrap()
    );
    println!("Vehicle model: {}", vin.get_vehicle_model().unwrap());
    println!("Vehicle Make: {}", vin.get_vehicle_make().unwrap());
    println!("Vehicle type: {}", vin.get_vehicle_type().unwrap());
    println!("Plant city: {}", vin.get_plant_city().unwrap());
    println!("Plant country: {}", vin.get_plant_country().unwrap());
    println!("Body class: {}", vin.get_body_class().unwrap());
    println!("Vehicle spec schema id: {}", vspec_schema_id);
    println!("Vehicle spec pattern id: {}", vspec_pattern_id);
    println!("ABS: {}", vin.abs_availablility(vspec_pattern_id).unwrap());
    println!(
        "Airbag locations curtain: {}",
        vin.airbag_locations_curtain().unwrap()
    );
    println!(
        "Airbag locations front: {}",
        vin.airbag_locations_front().unwrap()
    );
    println!(
        "Airbag locations knee: {}",
        vin.airbag_locations_knee().unwrap()
    );
    println!(
        "Airbag locations side: {}",
        vin.airbag_locations_side().unwrap()
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
    println!("Drive type: {}", vin.get_drive_type().unwrap());
    println!(
        "Axle count: {}",
        vin.get_axle_count(vspec_pattern_id).unwrap()
    );
    println!("Brake system: {}", vin.get_brake_system().unwrap());
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
        vin.get_vehicle_weight_rating().unwrap()
    );
    println!("Plant company: {}", vin.get_plant_company().unwrap());
    println!("Plant state: {}", vin.get_plant_state().unwrap());
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
    println!("Trim: {}", vin.vehicle_trim().unwrap());
    println!("Seatbelt type: {}", vin.seatbelt_type().unwrap());
    println!(
        "Number of rows: {}",
        vin.number_of_rows(vspec_pattern_id).unwrap()
    );
    println!(
        "Number of seats: {}",
        vin.number_of_seats(vspec_pattern_id).unwrap()
    );
}

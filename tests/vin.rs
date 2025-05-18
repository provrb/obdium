use obdium::vin::parser::VIN;

// This shouldn't be changed.
// All test outputs rely on accurate and reliabled tested outcomes for
// this specific VIN.
// Changing this will result in tests failing.
const VIN_STRING: &'static str = "KL4CJASB6JB660929";

#[test]
fn database_connect() {
    let vin = VIN::new(VIN_STRING);

    assert!(
        vin.test_database_connection(),
        "database_connect: couldn't connect to database"
    )
}

#[test]
fn wmi_prefix() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.get_wmi(),
        "KL4",
        "get_wmi: provided incorrect wmi for vin: {VIN_STRING}. expected KL4"
    )
}

#[test]
fn wmi_id() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.get_wmi_id().unwrap(),
        2069,
        "get_wmi_id: provided incorrect wmi id for wmi. expected 2069"
    )
}

#[test]
fn truck_type_id() {
    let vin = VIN::new(VIN_STRING);
    let wmi = vin.get_wmi();

    assert_eq!(
        vin.get_truck_type_id().unwrap(),
        0,
        "get_truck_type_id: provided incorrect truck type id for wmi: {wmi}. expected 0"
    )
}

#[test]
fn vehicle_type_id() {
    let vin = VIN::new(VIN_STRING);
    let wmi = vin.get_wmi();

    assert_eq!(
        vin.get_vehicle_type_id().unwrap(),
        7,
        "get_vehicle_type_id: provided incorrect vehicle type id for wmi: {wmi}. expected 7"
    )
}

#[test]
fn model_year() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.get_model_year().unwrap(),
        2018,
        "get_model_year: provided incorrect model year for vin: {VIN_STRING}. expected 2018"
    )
}

#[test]
fn vin_key() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.as_key(),
        "CJASB|JB660929",
        "as_key: provided incorrect key for vin: {VIN_STRING}. expected CJASB|JB660929"
    )
}

#[test]
fn schema_id() {
    let vin = VIN::new(VIN_STRING);
    let wmi = vin.get_wmi();
    let wmi_id = vin.get_wmi_id().unwrap();

    assert_eq!(
        vin.get_vin_schema_id().unwrap(),
        15103,
        "get_schema_id: provided incorrect id. wmi_id: {wmi_id}, wmi: {wmi}, vin: {VIN_STRING}. expected 15103."
    )
}

#[test]
fn engine_model() {
    let vin = VIN::new(VIN_STRING);
    let wmi = vin.get_wmi();
    let wmi_id = vin.get_wmi_id().unwrap();

    assert_eq!(
        vin.get_engine_model().unwrap(),
        "LUV: MFI, Variable Valve Timing, ALUM, E85 MAX",
        "engine_model: provided incorrect engine model. wmi_id: {wmi_id}, wmi: {wmi}, vin: {VIN_STRING}."
    )
}

#[test]
fn cylinder_count() {
    let vin = VIN::new(VIN_STRING);
    let wmi = vin.get_wmi();
    let wmi_id = vin.get_wmi_id().unwrap();

    assert_eq!(
        vin.get_cylinder_count().unwrap(),
        4,
        "cylinder_count: provided incorrect cylinder count model. wmi_id: {wmi_id}, wmi: {wmi}, vin: {VIN_STRING}. expected 4."
    )
}

#[test]
fn transmission_style() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.get_transmission_style().unwrap(),
        "Automatic",
        "transmission_style: incorrect value. expected 'Automatic'"
    );
}

#[test]
fn steering_location() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.get_steering_location().unwrap(),
        "Left-Hand Drive (LHD)",
        "steering_location: incorrect value. expected 'Left'"
    );
}

#[test]
fn abs_availability() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.abs_availablility().unwrap(),
        "Standard",
        "abs_availability: incorrect value. expected 'Standard'"
    );
}

#[test]
fn keyless_ignition() {
    let vin = VIN::new(VIN_STRING);

    assert_eq!(
        vin.keyless_ignition_availability().unwrap(),
        "Standard",
        "keyless_ignition: incorrect value. expected 'Standard'"
    );
}

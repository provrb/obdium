use std::net::TcpStream;

struct OBD {
    connection: TcpStream,
}

impl OBD {
    pub fn connect() {}
    pub fn send_request() {}

    pub fn rpm() {}
    pub fn engine_load() {}
    pub fn coolant_temp() {}
    pub fn short_term_fuel_trim() {}
    pub fn long_term_fuel_trim() {}
    pub fn fuel_pressure() {}
    pub fn intake_manifold_abs_pressure() {}
    pub fn vehicle_speed() {}
    pub fn timing_advance() {}
    pub fn intake_air_temp() {}
    pub fn maf_air_flow_rate() {} // Mass airflow sensor
    pub fn throttle_position() {}
    pub fn oxygen_sensors_present() {}
    pub fn read_oxygen_sensor() {}
    pub fn obd_standards() {}
    pub fn aux_input_status() {}
    pub fn engine_runtime() {}
    pub fn dist_travelled_with_mlt() {}
    pub fn fuel_rail_pressure() {}
    pub fn fuel_rail_guage_pressure() {}
    pub fn o2_sensor_air_fuel_ratio() {}
    pub fn catalyst_temp() {}
    pub fn control_module_voltage() {}
    pub fn cmd_air_fuel_equivalance_ratio() {}
    pub fn relative_throttle_pos() {}
    pub fn ambient_air_temp() {}
    pub fn absolute_throttle_pos() {}
    pub fn time_since_codes_cleared() {}
    pub fn max_values_for() {} // fuel-air equivalance ratio, o2 sensor voltage, current, and instake abs pressure
}
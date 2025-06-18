pub mod events;

use obdium::OBD;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{EventHandler, Window};

pub static ACTIVE_OBD: Lazy<Mutex<Option<Arc<Mutex<OBD>>>>> = Lazy::new(|| Mutex::new(None));
pub static USER_COMMAND_LISTENER: Lazy<Mutex<Option<EventHandler>>> =
    Lazy::new(|| Mutex::new(None));
pub static READINESS_TESTS_LISTENER: Lazy<Mutex<Option<EventHandler>>> =
    Lazy::new(|| Mutex::new(None));
pub(crate) static CUSTOM_PIDS_TRACKED: Lazy<Mutex<Vec<CustomPid>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub fn unlisten_events(window: &Arc<Window>) {
    {
        let mut handler = USER_COMMAND_LISTENER.lock().unwrap();
        if let Some(id) = handler.take() {
            window.unlisten(id);
        }
    }
    {
        let mut handler = READINESS_TESTS_LISTENER.lock().unwrap();
        if let Some(id) = handler.take() {
            window.unlisten(id);
        }
    }
}

/// Structs that are used as payloads
/// between frontend and backend.

/// Very brief vehicle information
/// No detailed information- use VehicleInfoExtended
/// with the VIN struct for that instead.
#[derive(Serialize, Deserialize, Clone)]
struct VehicleInfo {
    vin: String,
    make: String,
    model: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CustomPid {
    pub name: String,
    pub mode: String,
    pub pid: String,
    pub unit: String,
    pub command: String,
    pub equation: String,
}

/// All relevant information about a
/// vehicle.
///
/// Essential when the frontend
/// requests vehicle info about a VIN but
/// can't use the VIN struct and it's methods.
#[derive(Serialize, Deserialize, Clone, Default)]
struct VehicleInfoExtended {
    error_msg: String,
    vin: String,
    make: String,
    model: String,
    model_year: String,
    engine_model: String,
    cylinder_count: String,
    axel_count: String,
    traction_control: String,
    plant_country: String,
    plant_city: String,
    plant_state: String,
    semi_auto_headlamp_beam_switching: String,
    dynamic_brake_support: String,
    airbag_locations_knee: String,
    airbag_locations_side: String,
    drive_type: String,
    fuel_type: String,
    fuel_delivery_type: String,
    engine_manufacturer: String,
    anti_lock_braking_system: String,
    transmission_style: String,
    steering_location: String,
    keyless_ignition: String,
    top_speed: String,
    daytime_running_light: String,
    window_auto_reverse: String,
    airbag_locations_front: String,
    front_wheel_size: String,
    rear_wheel_size: String,
    automatic_crash_notification: String,
    trim: String,
    transmission_speeds: String,
    vehicle_base_price: String,
    number_of_rows: String,
    number_of_seats: String,
    brake_system: String,
    engine_displacement: String,
    gross_vehicle_weight_rating: String,
    airbag_locations_curtain: String,
    backup_camera: String,
    body_style: String,
    vehicle_manufacturer: String,
}

/// Tells the frontend the serial port
/// connection status.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ConnectionStatus {
    connected: bool,
    message: String,
    serial_port: String,
    baud_rate: u32,
    protocol: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConnectPaylod {
    serial_port: String,
    baud_rate: u32,
    protocol: u8,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Setting {
    t_id: String,
    checked: bool,

    // Optional data to send
    // Used for record_requests file pat
    data: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Dtc {
    category: String,
    description: String,
    name: String,
    permanant: bool,
    location: String,
}

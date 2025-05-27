// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use obdium::obd::OBD;
use obdium::{dicts::PID_INFOS, vin::VIN};
use serde::{Deserialize, Serialize};
use stats::{
    critical_frequency_calls, frequent_calls, high_frequency_calls, less_frequent_calls,
    once_calls, oxygen_sensors,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::sleep,
    time::Duration,
};
use tauri::{async_runtime::spawn, Manager, Window};

mod stats;

#[derive(Serialize, Deserialize, Clone)]
struct VehicleInfo {
    vin: String,
    make: String,
    model: String,
}

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

#[derive(Serialize, Deserialize, Clone)]
struct ConnectionStatus {
    connected: bool,
    message: String,
    serial_port: String,
}

fn track_data(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    critical_frequency_calls(window, obd);
    high_frequency_calls(window, obd);
    frequent_calls(window, obd);
    less_frequent_calls(window, obd);
    oxygen_sensors(window, obd);
    once_calls(window, obd);
}

fn send_vehicle_details(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd = Arc::clone(obd);
    let window = Arc::clone(window);
    spawn(async move {
        let mut obd = obd.lock().unwrap();

        // send the vin and vehicle details to the frontend
        match obd.get_vin() {
            Some(vin) => {
                let make = match vin.get_vehicle_make() {
                    Ok(make) => make,
                    Err(err) => {
                        println!("failed to resolve vehicle make from vin: {}", vin.get_vin());
                        println!("error: {err}");
                        "??".to_string()
                    }
                };

                let model = match vin.get_vehicle_model() {
                    Ok(model) => model,
                    Err(err) => {
                        println!(
                            "failed to resolve vehicle model from vin: {}",
                            vin.get_vin()
                        );
                        println!("error: {err}");
                        "??".to_string()
                    }
                };

                let v_info = VehicleInfo {
                    vin: vin.get_vin().to_string(),
                    make,
                    model,
                };

                window.emit("vehicle-details", v_info).unwrap();
            }
            None => {
                println!("error: getting vin. vin is none.");
            }
        };
    });
}

fn listen_send_pids(window: &Arc<Window>) {
    let window_arc = Arc::new(window.clone());
    let window_clone = Arc::clone(&window_arc);
    window_clone.listen("get-pids", move |_| {
        println!("received event");
        let _ = window_arc.emit("update-pids", PID_INFOS);
    });
}

fn send_connection_status(window: &Window, obd: &OBD, message: String, connected: bool) {
    let port = obd.serial_port_name().unwrap_or_default();
    let conn_status = ConnectionStatus {
        connected,
        message,
        serial_port: port,
    };

    let _ = window.emit("connection-status", conn_status);
}

fn connect_obd(window: &Window) -> Option<OBD> {
    // Try to connect obd
    let mut obd = OBD::new();

    match obd.connect("COM4", 38400) {
        Ok(()) => {
            let band = obd.serial_port_baud_rate().unwrap_or_default();
            let port = obd.serial_port_name().unwrap_or_default();

            send_connection_status(
                window,
                &obd,
                format!("Connected to port {port} on {band} band"),
                true,
            );

            Some(obd)
        }
        Err(error) => {
            send_connection_status(
                window,
                &obd,
                format!("Failed to connect. Error: {error}"),
                false,
            );

            None
        }
    }
}

// TODO: This is an eye sore.
fn listen_decode_vin(window: &Window) {
    let window_arc = Arc::new(window.clone());
    let window_clone = Arc::clone(&window_arc);
    window_clone.listen("decode-vin", move |event| {
        let vin = match VIN::new(event.payload().unwrap().replace("\"", "")) {
            Ok(vin) => vin,
            Err(err) => {
                // emit error
                let v_info = VehicleInfoExtended {
                    error_msg: format!("{}", err),
                    ..Default::default()
                };

                let _ = window_arc.emit("decode-vin", v_info);
                return;
            }
        };

        let v_info = VehicleInfoExtended {
            error_msg: "Decoded successfully.".to_string(),
            vin: vin.get_vin().to_string(),
            make: vin.get_vehicle_make().unwrap_or("N/A".into()),
            model: vin.get_vehicle_model().unwrap_or("N/A".into()),
            model_year: vin.get_model_year().unwrap_or(-1).to_string(),
            engine_model: vin.get_engine_model().unwrap_or("N/A".into()),
            cylinder_count: vin.get_cylinder_count().unwrap_or(-1).to_string(),
            axel_count: vin.get_axle_count().unwrap_or(-1).to_string(),
            traction_control: vin.traction_control().unwrap_or("N/A".into()),
            plant_country: vin.get_plant_country().unwrap_or("N/A".into()),
            plant_city: vin.get_plant_city().unwrap_or("N/A".into()),
            plant_state: vin.get_plant_state().unwrap_or("N/A".into()),
            semi_auto_headlamp_beam_switching: vin
                .semiauto_headlamp_beam_switching()
                .unwrap_or("N/A".into()),
            dynamic_brake_support: vin.dynamic_brake_support().unwrap_or("N/A".into()),
            airbag_locations_knee: vin.airbag_locations_knee().unwrap_or("N/A".into()),
            airbag_locations_side: vin.airbag_locations_side().unwrap_or("N/A".into()),
            body_style: vin.get_body_class().unwrap_or("N/A".into()),
            fuel_type: vin.get_fuel_type().unwrap_or("N/A".into()),
            fuel_delivery_type: vin.get_fuel_delivery_type().unwrap_or("N/A".into()),
            engine_manufacturer: vin.get_engine_manufacturer().unwrap_or("N/A".into()),
            anti_lock_braking_system: vin.abs_availablility().unwrap_or("N/A".into()),
            transmission_style: vin.get_transmission_style().unwrap_or("N/A".into()),
            steering_location: vin.get_steering_location().unwrap_or("N/A".into()),
            keyless_ignition: vin.keyless_ignition_availability().unwrap_or("N/A".into()),
            top_speed: vin.get_vehicle_top_speed().unwrap_or(-1).to_string(),
            daytime_running_light: vin.daytime_running_light().unwrap_or("N/A".into()),
            window_auto_reverse: vin.windows_auto_reverse().unwrap_or("N/A".into()),
            airbag_locations_front: vin.airbag_locations_front().unwrap_or("N/A".into()),
            front_wheel_size: vin.get_front_wheel_size().unwrap_or(-1).to_string(),
            rear_wheel_size: vin.get_rear_wheel_size().unwrap_or(-1).to_string(),
            automatic_crash_notification: vin
                .automatic_crash_notification()
                .unwrap_or("N/A".into()),
            trim: vin.vehicle_trim().unwrap_or("N/A".into()),
            transmission_speeds: vin.get_transmission_speeds().unwrap_or(-1).to_string(),
            vehicle_base_price: vin.get_vehicle_base_price().unwrap_or(-1.0).to_string(),
            number_of_rows: vin.number_of_rows().unwrap_or(-1).to_string(),
            number_of_seats: vin.number_of_seats().unwrap_or(-1).to_string(),
            brake_system: vin.get_brake_system().unwrap_or("N/A".into()),
            engine_displacement: vin.get_engine_displacement().unwrap_or(-1.0).to_string(),
            gross_vehicle_weight_rating: vin.get_vehicle_weight_rating().unwrap_or("N/A".into()),
            airbag_locations_curtain: vin.airbag_locations_curtain().unwrap_or("N/A".into()),
            backup_camera: vin.backup_camera().unwrap_or("N/A".into()),
            drive_type: vin.get_drive_type().unwrap_or("N/A".into()),
            vehicle_manufacturer: vin.get_vehicle_manufacturer().unwrap_or("N/A".into()),
        };

        let _ = window_arc.emit("decode-vin", v_info);
    });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let frontend_ready = Arc::new(AtomicBool::new(false));

            spawn(async move {
                listen_decode_vin(&window);

                // Detect when the frontend is loaded
                let frontend_ready_listener = Arc::clone(&frontend_ready);
                window.listen("frontend-loaded", move |_| {
                    frontend_ready_listener.store(true, Ordering::SeqCst);
                });

                let window_arc = Arc::new(window);
                listen_send_pids(&window_arc);

                while !frontend_ready.load(Ordering::SeqCst) {
                    sleep(Duration::from_millis(100));
                }


                let obd = connect_obd(&window_arc);
                if let Some(obd) = obd {
                    // Arc's
                    let obd = Arc::new(Mutex::new(obd));

                    // Usually called once
                    send_vehicle_details(&window_arc, &obd);

                    // Live tracking data
                    track_data(&window_arc, &obd);

                    // spawn thread to keep checking if obd disconnects
                    spawn(async move {
                        loop {
                            sleep(Duration::from_secs(1));
                            let obd = obd.lock().unwrap();
                            if !obd.connected() {
                                send_connection_status(
                                    &window_arc,
                                    &obd,
                                    "Connection dropped".to_string(),
                                    false,
                                );
                                break;
                            }
                        }
                    });
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

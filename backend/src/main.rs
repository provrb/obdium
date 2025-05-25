// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use obdium::dicts::PID_INFOS;
use obdium::obd::OBD;
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

fn send_pids(window: &Arc<Window>) {
    let window = Arc::clone(window);
    spawn(async move {
        for pid in PID_INFOS {
            let _ = window.emit("add-pid-info", pid);
        }
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let frontend_ready = Arc::new(AtomicBool::new(false));

            spawn(async move {
                // Detect when the frontend is loaded
                let frontend_ready_listener = Arc::clone(&frontend_ready);
                window.listen("frontend-loaded", move |_| {
                    frontend_ready_listener.store(true, Ordering::SeqCst);
                });

                while !frontend_ready.load(Ordering::SeqCst) {
                    sleep(Duration::from_millis(100));
                }

                let window_arc = Arc::new(window);
                send_pids(&window_arc);

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

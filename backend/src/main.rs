// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
                let v_info = VehicleInfo {
                    vin: vin.get_vin().to_string(),
                    make: vin.get_vehicle_make().unwrap_or("??".to_string()),
                    model: vin.get_vehicle_model().unwrap_or("??".to_string()),
                };

                window.emit("vehicle-details", v_info).unwrap();
            }
            None => {
                println!("error: getting vin. vin is none.");
            }
        };
    });
}

fn connect_obd(window: &Window) -> Option<OBD> {
    // Try to connect obd
    let mut obd = OBD::new();

    match obd.connect("COM4", 38400) {
        Ok(()) => {
            let port = obd.serial_port_name().unwrap_or_default();
            let band = obd.serial_port_baud_rate().unwrap_or_default();
            let conn_status = ConnectionStatus {
                connected: true,
                message: format!("Connected to port {port} on {band} band"),
                serial_port: port,
            };
            let _ = window.emit("connection-status", conn_status);
            Some(obd)
        }
        Err(error) => {
            let conn_status = ConnectionStatus {
                connected: false,
                message: format!("Failed to connect. Error: {error}"),
                serial_port: "".to_string(),
            };
            let _ = window.emit("connection-status", conn_status);
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

                let obd = connect_obd(&window);
                if let Some(obd) = obd {
                    // Arc's
                    let obd = Arc::new(Mutex::new(obd));
                    let window = Arc::new(window);
                    send_vehicle_details(&window, &obd);
                    track_data(&window, &obd);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

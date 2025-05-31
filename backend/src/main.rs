// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bridge;
mod stats;

use bridge::events::{
    do_send_connection_status, listen_connect_elm, listen_decode_vin,
    listen_send_ports,
};
use obdium::OBD;
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

fn track_data(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    critical_frequency_calls(window, obd);
    high_frequency_calls(window, obd);
    frequent_calls(window, obd);
    less_frequent_calls(window, obd);
    oxygen_sensors(window, obd);
    once_calls(window, obd);
}

fn connect_obd(window: &Window, port: String, baud_rate: u32, protocol: u8) -> Option<OBD> {
    // Try to connect obd
    let mut obd = OBD::new();

    match obd.connect(&port, baud_rate, protocol) {
        Ok(()) => {
            let band = obd.serial_port_baud_rate().unwrap_or_default();
            let port = obd.serial_port_name().unwrap_or_default();

            do_send_connection_status(
                window,
                &obd,
                format!("Connected to port {port} on {band} band"),
                true,
            );

            Some(obd)
        }
        Err(error) => {
            do_send_connection_status(
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
                listen_decode_vin(&window);

                // Detect when the frontend is loaded
                let frontend_ready_listener = Arc::clone(&frontend_ready);
                window.listen("frontend-loaded", move |_| {
                    frontend_ready_listener.store(true, Ordering::SeqCst);
                });

                let window_arc = Arc::new(window);

                while !frontend_ready.load(Ordering::SeqCst) {
                    sleep(Duration::from_millis(100));
                }

                listen_send_ports(&window_arc);
                listen_connect_elm(&window_arc);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

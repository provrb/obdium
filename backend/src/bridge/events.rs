use super::{ConnectPaylod, ConnectionStatus, Dtc, Setting, VehicleInfo, VehicleInfoExtended};
/// Events to bridge the frontend with
/// the backend.
///
/// Includes listening for requests from the frontend
/// to perform actions
use crate::{connect_obd, track_data, OBD};
use obdium::dicts::PID_INFOS;
use obdium::scalar::UnitPreferences;
use obdium::vin::VIN;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{async_runtime::spawn, Window};
use tokio::time::sleep;

// "Listen" events
//
// Where the backend "listens" for events from the frontend
// All listen events are prefixed with 'listen'

pub fn listen_send_pids(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    std::thread::sleep(Duration::from_secs(2));

    let window_arc = Arc::new(window.clone());
    let pids = {
        let mut obd = obd.lock().unwrap();
        obd.get_service_supported_pids("01")
    };

    println!("Supported pids list: \n{:?}", pids);

    let supported_pids: Vec<&String> = { pids.values().flatten().collect() };

    let mut supported_pids_info = PID_INFOS.to_owned();
    supported_pids_info
        .iter_mut()
        .for_each(|pid| pid.supported = supported_pids.contains(&&pid.pid.to_string()));
    supported_pids_info.sort_by(|a, b| b.supported.cmp(&a.supported));

    if supported_pids_info.is_empty() {
        supported_pids_info = PID_INFOS.to_vec();
    }

    window.listen("get-pids", move |_| {
        let _ = window_arc.emit("update-pids", &supported_pids_info);
    });
}

pub fn listen_send_readiness_test(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let window_arc = Arc::new(window.clone());
    let obd_arc = Arc::clone(obd);
    let window_clone = Arc::clone(&window_arc);
    window_clone.listen("get-readiness-tests", move |_| {
        let mut obd = obd_arc.lock().unwrap();
        let tests = [
            obd.get_common_tests_status().to_vec(),
            obd.get_advanced_tests_status().to_vec(),
        ]
        .concat();
        let _ = window_arc.emit("update-readiness-tests", tests);
    });
}

pub fn listen_send_ports(window: &Arc<Window>) {
    let window_arc = Arc::new(window.clone());
    let window_clone = Arc::clone(&window_arc);
    window_clone.listen("get-serial-ports", move |_| {
        let _ = window_arc.emit("update-serial-ports", OBD::get_open_serial_port());
    });
}

// TODO: This is an eye sore.
pub fn listen_decode_vin(window: &Window) {
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

pub fn listen_connect_elm(window: &Arc<Window>) {
    let window_arc = Arc::new(window.clone());
    let window_clone = Arc::clone(&window_arc);
    window_clone.listen("connect-elm", move |event| {
        let payload = event.payload().unwrap_or("");
        let connect_payload: ConnectPaylod = match serde_json::from_str(payload) {
            Ok(p) => p,
            Err(e) => {
                println!("Failed to parse connect payload: {e}");
                return;
            }
        };

        let obd = connect_obd(
            &window_arc,
            connect_payload.serial_port,
            connect_payload.baud_rate,
            connect_payload.protocol,
        );

        if let Some(obd) = obd {
            // Arc's
            let obd = Arc::new(Mutex::new(obd));

            // Usually called once
            do_send_vehicle_details(&window_arc, &obd);

            // Live tracking data
            track_data(&window_arc, &obd);

            // spawn thread to keep checking if obd disconnects
            let window_arc_clone = Arc::clone(&window_arc);
            let obd_clone = Arc::clone(&obd);
            spawn(async move {
                loop {
                    sleep(Duration::from_secs(1)).await;
                    let obd = obd_clone.lock().unwrap();
                    if !obd.is_connected() {
                        do_send_connection_status(
                            &window_arc_clone,
                            &obd,
                            "Connection dropped".to_string(),
                            false,
                        );
                        break;
                    }
                }
            });

            // Listen for disconnect-elm event outside of async block to avoid lifetime issues
            let window_arc_for_listen = Arc::clone(&window_arc);
            let obd_for_listen = Arc::clone(&obd);
            window_arc_for_listen.listen("disconnect-elm", {
                let window_arc_for_listen = Arc::clone(&window_arc_for_listen);
                move |_| {
                    if let Ok(mut obd) = obd_for_listen.lock() {
                        do_send_connection_status(
                            &window_arc_for_listen,
                            &obd,
                            "Connection dropped".to_string(),
                            false,
                        );
                        obd.disconnect();
                    }
                }
            });

            listen_set_unit_preferences(&window_arc, &obd);
            listen_send_connection_status(&window_arc, &obd);
            listen_change_obd_settings(&window_arc, &obd);
            listen_send_pids(&window_arc, &obd);
            listen_send_readiness_test(&window_arc, &obd);
            listen_send_dtcs(&window_arc, &obd);
            listen_clear_dtcs(&window_arc, &obd);
        }
    });
}

pub fn listen_change_obd_settings(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd_arc = Arc::clone(obd);
    window.listen("settings-changed", move |event| {
        let payload = match event.payload() {
            Some(payload) => payload,
            None => return,
        };

        let setting: Setting = match serde_json::from_str(payload) {
            Ok(p) => p,
            Err(e) => {
                println!("Failed to parse setting payload: {e}");
                return;
            }
        };

        let mut obd = obd_arc.lock().unwrap();
        match setting.t_id.as_str() {
            "record-responses" => {
                if let Some(path) = setting.data {
                    println!("record response has a path: {path}");
                    obd.record_requests(setting.checked, path);
                } else {
                    obd.record_requests(setting.checked, "./data/requests.json".into());
                }
            }
            "replay-responses" => obd.replay_requests(setting.checked),
            "use-freeze-fram" => obd.query_freeze_frame(setting.checked),
            _ => (),
        }
    });
}

pub fn listen_send_dtcs(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd_arc = Arc::clone(obd);
    let window_arc = Arc::clone(window);
    window.listen("get-dtcs", move |_| {
        let mut obd = obd_arc.lock().unwrap();
        std::thread::sleep(Duration::from_secs(1));

        let codes = [obd.get_trouble_codes(), obd.get_permanant_trouble_codes()].concat();
        println!("codes: {:?}", codes);

        // create serializable DTC struct from TroubleCode struct
        let serialized: Vec<Dtc> = codes
            .into_iter()
            .map(|dtc| Dtc {
                name: dtc.dtc,
                category: dtc.category.system_letter().to_string(),
                description: dtc.description,
                permanant: dtc.permanant,
                location: dtc.category.as_str().to_string(),
            })
            .collect();

        let _ = window_arc.emit("update-dtcs", serialized);
    });
}

pub fn listen_clear_dtcs(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd_arc = Arc::clone(obd);
    window.listen("clear-dtcs", move |_| {
        let mut obd = obd_arc.lock().unwrap();
        let _ = obd.clear_trouble_codes();
    });
}

pub fn listen_send_connection_status(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd_arc = Arc::clone(obd);
    let window_arc = Arc::clone(window);
    window.listen("get-connection-status", move |_| {
        let obd = obd_arc.lock().unwrap();
        do_send_connection_status(&window_arc, &obd, "".into(), obd.is_connected());
    });
}

pub fn listen_set_unit_preferences(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd_arc = Arc::clone(obd);
    window.listen("set-unit-preferences", move |event| {
        let payload = match event.payload() {
            Some(payload) => payload,
            None => return,
        };
        println!("Setting unit preference to {}", payload);

        let mut obd = obd_arc.lock().unwrap();
        let unit_preferences: UnitPreferences = match serde_json::from_str(payload) {
            Ok(unit_preferences) => unit_preferences,
            Err(err) => {
                println!("error serializing UnitPreferences from frontned: {err}");
                return;
            }
        };

        obd.set_unit_preferences(unit_preferences);
    });
}

// Calls to the frontend from the backend
// where the backend umprompty tells the
// frontend to do something.
//
// In this case, the frontend would be listening for these events.

pub fn do_send_vehicle_details(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd = Arc::clone(obd);
    let window = Arc::clone(window);
    spawn(async move {
        let mut obd = obd.lock().unwrap();
        std::thread::sleep(Duration::from_secs(2));

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

pub fn do_send_connection_status(window: &Window, obd: &OBD, message: String, connected: bool) {
    let port = obd.serial_port_name().unwrap_or_default();
    let baud = obd.serial_port_baud_rate().unwrap_or_default();
    let conn_status = ConnectionStatus {
        connected,
        message,
        serial_port: port,
        baud_rate: baud,
        protocol: obd.get_protocol_number(),
    };

    let _ = window.emit("connection-status", conn_status);
}

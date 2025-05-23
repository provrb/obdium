// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{Arc, Mutex}, thread::{sleep}, time::Duration};
use std::sync::atomic::{AtomicBool, Ordering};

use obdium::{
    obd::OBD, scalar::Scalar, vin::VIN, BankNumber, SensorNumber, Service
};
use serde::{Deserialize, Serialize};
use tauri::{async_runtime::spawn, Manager, Window};

#[derive(Serialize, Deserialize, Clone)]
struct Card {
    name: String,
    unit: String,
    value: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct VehicleInfo {
    vin: String,
    make: String,
    model: String,
}

#[tauri::command]
fn update_card<T>(window: &Window, name: T, scalar: Scalar)
where
    T: Into<String> + std::fmt::Debug,
{
    let card = Card {
        name: name.into(),
        unit: scalar.unit.as_str().to_string().to_uppercase(),
        value: scalar.value.round(),
    };

    window.emit("update-card", card).unwrap();
}

fn track_data(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    // Very High frequency calls
    {
        let window = Arc::clone(&window);
        let obd = Arc::clone(&obd);
        spawn(async move {
            loop {
                sleep(Duration::from_millis(100));
                
                let mut obd = obd.lock().unwrap();
                let rpm = obd.rpm();
                let engine_load = obd.engine_load();
                let throttle_pos = obd.throttle_position();
                let relative_throttle_pos = obd.relative_throttle_pos();
                let abs_throttle_pos_b = obd.abs_throttle_position_b();
                let abs_throttle_pos_c = obd.abs_throttle_position_c();
                let acc_pedal_pos_d = obd.acc_pedal_position_d();
                let acc_pedal_pos_e = obd.acc_pedal_position_e();
                let acc_pedal_pos_f = obd.acc_pedal_position_f();
                drop(obd);

                update_card(&window, "Engine Speed", rpm);
                update_card(&window, "Engine Load", engine_load);
                update_card(&window, "Throttle Pos.", throttle_pos);
                update_card(&window, "Relative Throttle Pos.", relative_throttle_pos);
                update_card(&window, "Abs. Throttle Pos. (D)", abs_throttle_pos_b);
                update_card(&window, "Abs. Throttle Pos. (C)", abs_throttle_pos_c);
                update_card(&window, "Accelerator Pedal Pos. (D)", acc_pedal_pos_d);
                update_card(&window, "Accelerator Pedal Pos. (E)", acc_pedal_pos_e);
                update_card(&window, "Accelerator Pedal Pos. (F)", acc_pedal_pos_f);
            }
        });
    }

    // High frequency calls
    {
        let window = Arc::clone(&window);
        let obd = Arc::clone(&obd);
        spawn(async move {
            loop {
                sleep(Duration::from_millis(200));
                let mut obd = obd.lock().unwrap();
                let vehicle_speed = obd.vehicle_speed();
                let engine_fuel_rate = obd.engine_fuel_rate();
                let engine_oil_pressure = obd.engine_oil_pressure();
                let drivers_demand_engine_torque = obd.drivers_demand_engine_torque();
                let actual_engine_torque = obd.actual_engine_torque();
                let ref_engine_torque = obd.reference_engine_torque();
                let engine_torque_data = obd.engine_percent_torque_data();
                let fuel_pressure = obd.fuel_pressure();
                let fuel_rail_pressure = obd.fuel_rail_pressure();
                let fuel_rail_gauge_pressure = obd.fuel_rail_guage_pressure();
                let cylinder_fuel_rate = obd.cylinder_fuel_rate();
                let maf_air_flow_rate = obd.maf_air_flow_rate();
                let max_air_flow_rate_maf = obd.max_air_flow_rate_from_maf();
                let timing_advance = obd.timing_advance();
                let boost_guage_pressure = obd.boost_guage_pressure();
                let turbocharger_rpm = obd.turbocharger_rpm();
                drop(obd);

                update_card(&window, "Vehicle Speed", vehicle_speed);
                update_card(&window, "Engine Fuel Rate", engine_fuel_rate);
                update_card(&window, "Engine Oil Pressure", engine_oil_pressure);
                update_card(&window, "Drivers Demand Engine Torque", drivers_demand_engine_torque);
                update_card(&window, "Actual Engine Torque", actual_engine_torque);
                update_card(&window, "Reference Engine Torque", ref_engine_torque);
                update_card(&window, "Fuel Pressure", fuel_pressure);
                update_card(&window, "Fuel Rail Pressure", fuel_rail_pressure);
                update_card(&window, "Fuel Rail Gauge Pressure", fuel_rail_gauge_pressure);
                update_card(&window, "Cylinder Fuel Rate", cylinder_fuel_rate);
                update_card(&window, "MAF Airflow Rate", maf_air_flow_rate);
                update_card(&window, "MAF Maximum Airflow Rate", max_air_flow_rate_maf);
                update_card(&window, "Timing Advance", timing_advance);
                update_card(&window, "Boost Gauge Pressure", boost_guage_pressure);
                update_card(&window, "Turbocharger RPM", turbocharger_rpm);
                update_card(&window, "Idle Engine Torque", engine_torque_data.0);
                update_card(&window, "Engine Point 1 Torque", engine_torque_data.1);
                update_card(&window, "Engine Point 2 Torque", engine_torque_data.2);
                update_card(&window, "Engine Point 3 Torque", engine_torque_data.3);
                update_card(&window, "Engine Point 4 Torque", engine_torque_data.4);
            }
        });
    }

    // Frequent calls
    {
        let window = Arc::clone(&window);
        let obd = Arc::clone(&obd);
        spawn(async move {
            loop {
                sleep(Duration::from_millis(500));

                let mut obd = obd.lock().unwrap();
                let stft_bank_1 = obd.short_term_fuel_trim(&BankNumber::Bank1);
                let stft_bank_2 = obd.short_term_fuel_trim(&BankNumber::Bank2);

                let o2_sensor_1 = obd.read_oxygen_sensor(&SensorNumber::Sensor1);
                let o2_sensor_2 = obd.read_oxygen_sensor(&SensorNumber::Sensor2);
                let o2_sensor_3 = obd.read_oxygen_sensor(&SensorNumber::Sensor3);
                let o2_sensor_4 = obd.read_oxygen_sensor(&SensorNumber::Sensor4);
                let o2_sensor_5 = obd.read_oxygen_sensor(&SensorNumber::Sensor5);
                let o2_sensor_6 = obd.read_oxygen_sensor(&SensorNumber::Sensor6);
                let o2_sensor_7 = obd.read_oxygen_sensor(&SensorNumber::Sensor7);
                let o2_sensor_8 = obd.read_oxygen_sensor(&SensorNumber::Sensor8);

                let o2_sensor_af_ratio_1 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor1);
                let o2_sensor_af_ratio_2 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor2);
                let o2_sensor_af_ratio_3 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor3);
                let o2_sensor_af_ratio_4 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor4);
                let o2_sensor_af_ratio_5 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor5);
                let o2_sensor_af_ratio_6 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor6);
                let o2_sensor_af_ratio_7 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor7);
                let o2_sensor_af_ratio_8 = obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor8);

                drop(obd);

                update_card(&window, "Short Term Fuel Trim (Bank 1)", stft_bank_1);
                update_card(&window, "Short Term Fuel Trim (Bank 2)", stft_bank_2);
                update_card(&window, "O2 Sensor (1) AFR", o2_sensor_af_ratio_1.0);
                update_card(&window, "O2 Sensor (1) Voltage (2)", o2_sensor_af_ratio_1.1);
                update_card(&window, "O2 Sensor (1) Voltage (1)", o2_sensor_1.0);
                update_card(&window, "O2 Sensor (1) STFT", o2_sensor_1.1);

                update_card(&window, "O2 Sensor (2) AFR", o2_sensor_af_ratio_2.0);
                update_card(&window, "O2 Sensor (2) Voltage (2)", o2_sensor_af_ratio_2.1);
                update_card(&window, "O2 Sensor (2) Voltage (1)", o2_sensor_2.0);
                update_card(&window, "O2 Sensor (2) STFT", o2_sensor_2.1);
                
                update_card(&window, "O2 Sensor (3) AFR", o2_sensor_af_ratio_3.0);
                update_card(&window, "O2 Sensor (3) Voltage (2)", o2_sensor_af_ratio_3.1);
                update_card(&window, "O2 Sensor (3) Voltage (1)", o2_sensor_3.0);
                update_card(&window, "O2 Sensor (3) STFT", o2_sensor_3.1);
                
                update_card(&window, "O2 Sensor (4) AFR", o2_sensor_af_ratio_4.0);
                update_card(&window, "O2 Sensor (4) Voltage (2)", o2_sensor_af_ratio_4.1);
                update_card(&window, "O2 Sensor (4) Voltage (1)", o2_sensor_4.0);
                update_card(&window, "O2 Sensor (4) STFT", o2_sensor_4.1);
                
                update_card(&window, "O2 Sensor (5) AFR", o2_sensor_af_ratio_5.0);
                update_card(&window, "O2 Sensor (5) Voltage (2)", o2_sensor_af_ratio_5.1);
                update_card(&window, "O2 Sensor (5) Voltage (1)", o2_sensor_5.0);
                update_card(&window, "O2 Sensor (5) STFT", o2_sensor_5.1);
                
                update_card(&window, "O2 Sensor (6) AFR", o2_sensor_af_ratio_6.0);
                update_card(&window, "O2 Sensor (6) Voltage (2)", o2_sensor_af_ratio_6.1);
                update_card(&window, "O2 Sensor (6) Voltage (1)", o2_sensor_6.0);
                update_card(&window, "O2 Sensor (6) STFT", o2_sensor_6.1);
                
                update_card(&window, "O2 Sensor (7) AFR", o2_sensor_af_ratio_7.0);
                update_card(&window, "O2 Sensor (7) Voltage (2)", o2_sensor_af_ratio_7.1);
                update_card(&window, "O2 Sensor (7) Voltage (1)", o2_sensor_7.0);
                update_card(&window, "O2 Sensor (7) STFT", o2_sensor_7.1);
                
                update_card(&window, "O2 Sensor (8) AFR", o2_sensor_af_ratio_8.0);
                update_card(&window, "O2 Sensor (8) Voltage (2)", o2_sensor_af_ratio_8.1);
                update_card(&window, "O2 Sensor (8) Voltage (1)", o2_sensor_8.0);
                update_card(&window, "O2 Sensor (8) STFT", o2_sensor_8.1);
            }
        });
    }

    // Less frequent calls
    {
        let window = Arc::clone(&window);
        let obd = Arc::clone(&obd);
        spawn(async move {
            loop {
                sleep(Duration::from_secs(1));

                let mut obd = obd.lock().unwrap();
                let ltft_bank_1 = obd.long_term_fuel_trim(&BankNumber::Bank1);
                let ltft_bank_2 = obd.long_term_fuel_trim(&BankNumber::Bank2);
                let coolant_temp = obd.coolant_temp();
                let coolant_temp_sensors = obd.coolant_temp_sensors();
                let engine_oil_temp = obd.engine_oil_temp(Service::Mode01);
                let engine_oil_temp_ext = obd.engine_oil_temp(Service::Mode22);
                let engine_oil_temp_sensors = obd.engine_oil_temp_sensors();
                let commanded_egr = obd.commanded_egr();
                let egr_error = obd.egr_error();
                
                let catalyst_temp_b1_s1 = obd.catalyst_temp(BankNumber::Bank1, SensorNumber::Sensor1);
                let catalyst_temp_b1_s2 = obd.catalyst_temp(BankNumber::Bank1, SensorNumber::Sensor2);
                let catalyst_temp_b2_s1 = obd.catalyst_temp(BankNumber::Bank2, SensorNumber::Sensor1);
                let catalyst_temp_b2_s2 = obd.catalyst_temp(BankNumber::Bank2, SensorNumber::Sensor2);

                let barometric_pressure = obd.abs_barometric_pressure();
                let ambient_air_temp = obd.ambient_air_temp();
                let max_values_for = obd.max_values_for();
                let fuel_injection_timing = obd.fuel_injection_timing();
                let commanded_evap_purge = obd.commanded_evap_purge();
                let evap_sys_vapor_pressure = obd.evap_system_vapor_pressure();
                let control_mod_voltage = obd.control_module_voltage();
                let engine_runtime = obd.engine_runtime();

                drop(obd);

                update_card(&window, "Maximum AFR Value", max_values_for.0);
                update_card(&window, "Maximum O2 Sensor Voltage", max_values_for.1);
                update_card(&window, "Maximum O2 Sensor Current", max_values_for.2);
                update_card(&window, "Maximum Intake Abs. Pressure", max_values_for.3);
                update_card(&window, "Long Term Fuel Trim (Bank 1)", ltft_bank_1);
                update_card(&window, "Long Term Fuel Trim (Bank 2)", ltft_bank_2);
                update_card(&window, "Coolant Temp.", coolant_temp);
                update_card(&window, "Coolant Temp. (Sensors: A)", coolant_temp_sensors.0);
                update_card(&window, "Coolant Temp. (Sensors: B)", coolant_temp_sensors.1);
                update_card(&window, "Engine Oil Temp. (Mode 01)", engine_oil_temp);
                update_card(&window, "Engine Oil Temp. (Mode 22)", engine_oil_temp_ext);
                update_card(&window, "Engine Oil Temp. (Sensors: A)", engine_oil_temp_sensors.0);
                update_card(&window, "Engine Oil Temp. (Sensors: B)", engine_oil_temp_sensors.1);
                update_card(&window, "Commanded EGR", commanded_egr);
                update_card(&window, "EGR Error", egr_error);
                update_card(&window, "Catalyst Temp. (Bank 1: Sensor 1)", catalyst_temp_b1_s1);
                update_card(&window, "Catalyst Temp. (Bank 1: Sensor 2)", catalyst_temp_b1_s2);
                update_card(&window, "Catalyst Temp. (Bank 2: Sensor 1)", catalyst_temp_b2_s1);
                update_card(&window, "Catalyst Temp. (Bank 2: Sensor 2)", catalyst_temp_b2_s2);
                update_card(&window, "Absolute Barometric Pressure", barometric_pressure);
                update_card(&window, "Ambient Air Temp.", ambient_air_temp);
                update_card(&window, "Fuel Injection Timing", fuel_injection_timing);
                update_card(&window, "Commanded EVAP Purge", commanded_evap_purge);
                update_card(&window, "EVAP System Vapor Pressure", evap_sys_vapor_pressure);
                update_card(&window, "Control Module Voltage", control_mod_voltage);
                update_card(&window, "Engine Runtime (Session)", engine_runtime);
            }
        });
    }

    // Rare calls
    {
        let window = Arc::clone(&window);
        let obd = Arc::clone(&obd);
        spawn(async move {
            loop {
                sleep(Duration::from_secs(5));

                let mut obd = obd.lock().unwrap();
                let warm_ups = obd.warm_ups_since_codes_cleared();
                let dist_since = obd.distance_traveled_since_codes_cleared();
                let dist_with = obd.distance_traveled_with_mil();
                let time_since = obd.time_since_codes_cleared();
                let time_with = obd.time_run_with_mil();
                let odometer = obd.odometer();
                let ethanol_fuel_percent = obd.ethanol_fuel_percentage();
                
                drop(obd);

                update_card(&window, "Warm-Ups Since Codes Cleared", warm_ups);
                update_card(&window, "Dist. Since Codes Cleared", dist_since);
                update_card(&window, "Dist. With Check Engine Light", dist_with);
                update_card(&window, "Time Since Codes Cleared", time_since);
                update_card(&window, "Time With Check Engine Light", time_with);
                update_card(&window, "Odometer", odometer);
                update_card(&window, "Ethanol Fuel Percentage", ethanol_fuel_percent);
            }
        });
    }
}

fn send_vehicle_details(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let obd = Arc::clone(&obd);
    let window = Arc::clone(&window);
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {            
            let window = app.get_window("main").unwrap();

            let frontend_ready = Arc::new(AtomicBool::new(false));
            spawn(async move {
                let frontend_ready_listener = Arc::clone(&frontend_ready);
                let _events = window.listen("frontend-loaded", move |_| {
                    frontend_ready_listener.store(true, Ordering::SeqCst);
                });

                while !frontend_ready.load(Ordering::SeqCst) {
                    sleep(Duration::from_millis(100));
                }

                let mut obd = OBD::new();
                obd.connect("COM4", 38400).unwrap();

                // Arc's
                let obd = Arc::new(Mutex::new(obd));
                let window = Arc::new(window);

                track_data(&window, &obd);
                send_vehicle_details(&window, &obd);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

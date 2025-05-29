use obdium::{scalar::Scalar, BankNumber};
use obdium::{SensorNumber, Service, OBD};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::{async_runtime::spawn, Window};
use tokio::time;

macro_rules! update {
    ($win:expr, $($name:expr => $val:expr),* $(,)?) => {
        $(
            update_card($win, $name, $val);
        )*
    };
}

#[derive(Serialize, Deserialize, Clone)]
struct Card {
    name: String,
    unit: String,
    value: f32,
}

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

pub fn critical_frequency_calls(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let window = Arc::clone(window);
    let obd = Arc::clone(obd);
    spawn(async move {
        let mut interval = time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;

            let mut obd = obd.lock().unwrap();
            if !obd.connected() {
                break;
            }

            let rpm = obd.rpm();
            let turbocharger_rpm = obd.turbocharger_rpm();
            let vehicle_speed = obd.vehicle_speed();

            drop(obd);

            update!(
                &window,
                "Vehicle Speed" => vehicle_speed,
                "Engine Speed" => rpm,
                "Turbocharger RPM" => turbocharger_rpm,
            );
        }
    });
}

pub fn high_frequency_calls(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let window = Arc::clone(window);
    let obd = Arc::clone(obd);

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        let mut cycle = 0;
        loop {
            interval.tick().await;
            cycle = (cycle + 1) % 5;

            match cycle {
                0 => {
                    // Cycle 1
                    let (
                        maf_air_flow_rate,
                        engine_fuel_rate,
                        engine_oil_pressure,
                        drivers_demand_engine_torque,
                        actual_engine_torque,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        if !obd.connected() {
                            break;
                        }

                        (
                            obd.maf_air_flow_rate(),
                            obd.engine_fuel_rate(),
                            obd.engine_oil_pressure(),
                            obd.drivers_demand_engine_torque(),
                            obd.actual_engine_torque(),
                        )
                    };

                    update!(
                        &window,
                        "MAF Airflow Rate" => maf_air_flow_rate,
                        "Engine Fuel Rate" => engine_fuel_rate,
                        "Engine Oil Pressure" => engine_oil_pressure,
                        "Drivers Demand Engine Torque" => drivers_demand_engine_torque,
                        "Actual Engine Torque" => actual_engine_torque,
                    );
                }
                1 => {
                    // Cycle 2
                    let (
                        engine_load,
                        ref_engine_torque,
                        fuel_pressure,
                        fuel_rail_pressure,
                        fuel_rail_gauge_pressure,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.engine_load(),
                            obd.reference_engine_torque(),
                            obd.fuel_pressure(),
                            obd.fuel_rail_pressure(),
                            obd.fuel_rail_guage_pressure(),
                        )
                    };

                    update!(
                        &window,
                        "Engine Load" => engine_load,
                        "Reference Engine Torque" => ref_engine_torque,
                        "Fuel Pressure" => fuel_pressure,
                        "Fuel Rail Pressure" => fuel_rail_pressure,
                        "Fuel Rail Gauge Pressure" => fuel_rail_gauge_pressure,
                    );
                }
                2 => {
                    // Cycle 3
                    let (
                        cylinder_fuel_rate,
                        max_air_flow_rate_maf,
                        timing_advance,
                        boost_guage_pressure,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.cylinder_fuel_rate(),
                            obd.max_air_flow_rate_from_maf(),
                            obd.timing_advance(),
                            obd.boost_guage_pressure(),
                        )
                    };

                    update!(
                        &window,
                        "Cylinder Fuel Rate" => cylinder_fuel_rate,
                        "MAF Maximum Airflow Rate" => max_air_flow_rate_maf,
                        "Timing Advance" => timing_advance,
                        "Boost Gauge Pressure" => boost_guage_pressure,
                    );
                }
                3 => {
                    // Cycle 4
                    let (
                        throttle_pos,
                        rel_throttle_pos,
                        abs_throttle_b,
                        abs_throttle_c,
                        accel_pedal_d,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.throttle_position(),
                            obd.relative_throttle_pos(),
                            obd.abs_throttle_position_b(),
                            obd.abs_throttle_position_c(),
                            obd.acc_pedal_position_d(),
                        )
                    };

                    update!(
                        &window,
                        "Throttle Pos." => throttle_pos,
                        "Relative Throttle Pos." => rel_throttle_pos,
                        "Abs. Throttle Pos. (D)" => abs_throttle_b,
                        "Abs. Throttle Pos. (C)" => abs_throttle_c,
                        "Accelerator Pedal Pos. (D)" => accel_pedal_d,
                    );
                }
                4 => {
                    // Cycle 5
                    let (acc_pedal_pos_e, acc_pedal_pos_f, engine_torque_data) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.acc_pedal_position_e(),
                            obd.acc_pedal_position_f(),
                            obd.engine_percent_torque_data(),
                        )
                    };

                    update!(
                        &window,
                        "Idle Engine Torque" => engine_torque_data.0,
                        "Engine Point 1 Torque" => engine_torque_data.1,
                        "Engine Point 2 Torque" => engine_torque_data.2,
                        "Engine Point 3 Torque" => engine_torque_data.3,
                        "Engine Point 4 Torque" => engine_torque_data.4,
                        "Accelerator Pedal Pos. (E)" => acc_pedal_pos_e,
                        "Accelerator Pedal Pos. (F)" => acc_pedal_pos_f,
                    );
                }
                _ => unreachable!(),
            }
        }
    });
}

pub fn frequent_calls(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let window = Arc::clone(window);
    let obd = Arc::clone(obd);
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(4));
        let mut cycles = 0;
        loop {
            interval.tick().await;

            cycles = (cycles + 1) % 2;
            match cycles {
                0 => {
                    let stft_bank_1 = {
                        let mut obd = obd.lock().unwrap();
                        if !obd.connected() {
                            break;
                        }
                        obd.short_term_fuel_trim(&BankNumber::Bank1)
                    };

                    update!(&window, "Short Term Fuel Trim (Bank 1)" => stft_bank_1);
                }
                1 => {
                    let stft_bank_2 = {
                        let mut obd = obd.lock().unwrap();
                        obd.short_term_fuel_trim(&BankNumber::Bank2)
                    };

                    update!(
                        &window,
                        "Short Term Fuel Trim (Bank 2)" => stft_bank_2,
                    );
                }
                _ => unreachable!(),
            }
        }
    });
}

pub fn less_frequent_calls(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let window = Arc::clone(window);
    let obd = Arc::clone(obd);
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3));
        let mut cycles = 0;
        loop {
            interval.tick().await;
            cycles = (cycles + 1) % 4;

            match cycles {
                0 => {
                    let (
                        ltft_bank_1,
                        ltft_bank_2,
                        coolant_temp,
                        coolant_temp_sensors,
                        engine_oil_temp,
                        engine_oil_temp_ext,
                        engine_oil_temp_sensors,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        if !obd.connected() {
                            break;
                        }
                        (
                            obd.long_term_fuel_trim(&BankNumber::Bank1),
                            obd.long_term_fuel_trim(&BankNumber::Bank2),
                            obd.coolant_temp(),
                            obd.coolant_temp_sensors(),
                            obd.engine_oil_temp(Service::Mode01),
                            obd.engine_oil_temp(Service::Mode22),
                            obd.engine_oil_temp_sensors(),
                        )
                    };

                    update!(
                        &window,
                        "Long Term Fuel Trim (Bank 1)" => ltft_bank_1,
                        "Long Term Fuel Trim (Bank 2)" => ltft_bank_2,
                        "Coolant Temp." => coolant_temp,
                        "Engine Oil Temp. (Mode 22)" => engine_oil_temp_ext,
                        "Engine Oil Temp. (Mode 01)" => engine_oil_temp,
                        "Coolant Temp. (Sensors: A)" => coolant_temp_sensors.0,
                        "Coolant Temp. (Sensors: B)" => coolant_temp_sensors.1,
                        "Engine Oil Temp. (Sensors: A)" => engine_oil_temp_sensors.0,
                        "Engine Oil Temp. (Sensors: B)" => engine_oil_temp_sensors.1,
                    );
                }
                1 => {
                    let (
                        commanded_egr,
                        egr_error,
                        catalyst_temp_b1_s1,
                        catalyst_temp_b1_s2,
                        catalyst_temp_b2_s1,
                        catalyst_temp_b2_s2,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.commanded_egr(),
                            obd.egr_error(),
                            obd.catalyst_temp(BankNumber::Bank1, SensorNumber::Sensor1),
                            obd.catalyst_temp(BankNumber::Bank1, SensorNumber::Sensor2),
                            obd.catalyst_temp(BankNumber::Bank2, SensorNumber::Sensor1),
                            obd.catalyst_temp(BankNumber::Bank2, SensorNumber::Sensor2),
                        )
                    };

                    update!(
                        &window,
                        "Catalyst Temp. (Bank 1: Sensor 1)" => catalyst_temp_b1_s1,
                        "Catalyst Temp. (Bank 1: Sensor 2)" => catalyst_temp_b1_s2,
                        "Catalyst Temp. (Bank 2: Sensor 1)" => catalyst_temp_b2_s1,
                        "Catalyst Temp. (Bank 2: Sensor 2)" => catalyst_temp_b2_s2,
                        "Commanded EGR" => commanded_egr,
                        "EGR Error" => egr_error,
                    );
                }
                2 => {
                    let (
                        barometric_pressure,
                        ambient_air_temp,
                        max_values_for,
                        fuel_injection_timing,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.abs_barometric_pressure(),
                            obd.ambient_air_temp(),
                            obd.max_values_for(),
                            obd.fuel_injection_timing(),
                        )
                    };

                    update!(
                        &window,
                        "Absolute Barometric Pressure" => barometric_pressure,
                        "Ambient Air Temp." => ambient_air_temp,
                        "Fuel Injection Timing" => fuel_injection_timing,
                        "Maximum AFR Value" => max_values_for.0,
                        "Maximum O2 Sensor Voltage" => max_values_for.1,
                        "Maximum O2 Sensor Current" => max_values_for.2,
                        "Maximum Intake Abs. Pressure" => max_values_for.3,
                    );
                }
                3 => {
                    let (
                        commanded_evap_purge,
                        evap_sys_vapor_pressure,
                        control_mod_voltage,
                        engine_runtime,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.commanded_evap_purge(),
                            obd.evap_system_vapor_pressure(),
                            obd.control_module_voltage(),
                            obd.engine_runtime(),
                        )
                    };

                    update!(
                        &window,
                        "Commanded EVAP Purge" => commanded_evap_purge,
                        "EVAP System Vapor Pressure" =>evap_sys_vapor_pressure,
                        "Control Module Voltage" => control_mod_voltage,
                        "Engine Runtime (Session)" => engine_runtime,
                    );
                }
                _ => unreachable!(),
            }
        }
    });
}

pub fn oxygen_sensors(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    let window = Arc::clone(window);
    let obd = Arc::clone(obd);
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(6));
        let mut cycles = 0;
        loop {
            interval.tick().await;
            cycles = (cycles + 1) % 4;

            match cycles {
                0 => {
                    let (o2_sensor_1, o2_sensor_2, o2_sensor_3, o2_sensor_4) = {
                        let mut obd = obd.lock().unwrap();
                        if !obd.connected() {
                            break;
                        }
                        (
                            obd.read_oxygen_sensor(&SensorNumber::Sensor1),
                            obd.read_oxygen_sensor(&SensorNumber::Sensor2),
                            obd.read_oxygen_sensor(&SensorNumber::Sensor3),
                            obd.read_oxygen_sensor(&SensorNumber::Sensor4),
                        )
                    };

                    update!(
                        &window,
                        "O2 Sensor (1) Voltage (1)" => o2_sensor_1.0,
                        "O2 Sensor (1) STFT" => o2_sensor_1.1,
                        "O2 Sensor (2) Voltage (1)" => o2_sensor_2.0,
                        "O2 Sensor (2) STFT" => o2_sensor_2.1,
                        "O2 Sensor (3) Voltage (1)" => o2_sensor_3.0,
                        "O2 Sensor (3) STFT" => o2_sensor_3.1,
                        "O2 Sensor (4) Voltage (1)" => o2_sensor_4.0,
                        "O2 Sensor (4) STFT" => o2_sensor_4.1,
                    );
                }
                1 => {
                    let (
                        o2_sensor_af_ratio_1,
                        o2_sensor_af_ratio_2,
                        o2_sensor_af_ratio_3,
                        o2_sensor_af_ratio_4,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor1),
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor2),
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor3),
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor4),
                        )
                    };

                    update!(
                        &window,
                        "O2 Sensor (1) AFR" => o2_sensor_af_ratio_1.0,
                        "O2 Sensor (1) Voltage (2)" => o2_sensor_af_ratio_1.1,
                        "O2 Sensor (2) AFR" => o2_sensor_af_ratio_2.0,
                        "O2 Sensor (2) Voltage (2)" => o2_sensor_af_ratio_2.1,
                        "O2 Sensor (3) AFR" => o2_sensor_af_ratio_3.0,
                        "O2 Sensor (3) Voltage (2)" => o2_sensor_af_ratio_3.1,
                        "O2 Sensor (4) AFR" => o2_sensor_af_ratio_4.0,
                        "O2 Sensor (4) Voltage (2)" => o2_sensor_af_ratio_4.1,
                    );
                }
                2 => {
                    let (o2_sensor_5, o2_sensor_6, o2_sensor_7, o2_sensor_8) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.read_oxygen_sensor(&SensorNumber::Sensor5),
                            obd.read_oxygen_sensor(&SensorNumber::Sensor6),
                            obd.read_oxygen_sensor(&SensorNumber::Sensor7),
                            obd.read_oxygen_sensor(&SensorNumber::Sensor8),
                        )
                    };

                    update!(
                        &window,
                        "O2 Sensor (5) Voltage (1)" => o2_sensor_5.0,
                        "O2 Sensor (5) STFT" => o2_sensor_5.1,
                        "O2 Sensor (6) Voltage (1)" => o2_sensor_6.0,
                        "O2 Sensor (6) STFT" => o2_sensor_6.1,
                        "O2 Sensor (7) Voltage (1)" => o2_sensor_7.0,
                        "O2 Sensor (7) STFT" => o2_sensor_7.1,
                        "O2 Sensor (8) Voltage (1)" => o2_sensor_8.0,
                        "O2 Sensor (8) STFT" => o2_sensor_8.1,
                    );
                }
                3 => {
                    let (
                        o2_sensor_af_ratio_5,
                        o2_sensor_af_ratio_6,
                        o2_sensor_af_ratio_7,
                        o2_sensor_af_ratio_8,
                    ) = {
                        let mut obd = obd.lock().unwrap();
                        (
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor5),
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor6),
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor7),
                            obd.read_oxygen_sensor_abcd(&SensorNumber::Sensor8),
                        )
                    };

                    update!(
                        &window,
                            "O2 Sensor (5) AFR" => o2_sensor_af_ratio_5.0,
                            "O2 Sensor (5) Voltage (2)" => o2_sensor_af_ratio_5.1,
                            "O2 Sensor (6) AFR" => o2_sensor_af_ratio_6.0,
                            "O2 Sensor (6) Voltage (2)" => o2_sensor_af_ratio_6.1,
                            "O2 Sensor (7) AFR" => o2_sensor_af_ratio_7.0,
                            "O2 Sensor (7) Voltage (2)" => o2_sensor_af_ratio_7.1,
                            "O2 Sensor (8) AFR" => o2_sensor_af_ratio_8.0,
                            "O2 Sensor (8) Voltage (2)" => o2_sensor_af_ratio_8.1,
                    );
                }
                _ => unreachable!(),
            }
        }
    });
}

pub fn once_calls(window: &Arc<Window>, obd: &Arc<Mutex<OBD>>) {
    // Once calls
    let window = Arc::clone(window);
    let obd = Arc::clone(obd);
    spawn(async move {
        let mut obd = obd.lock().unwrap();
        let warm_ups = obd.warm_ups_since_codes_cleared();
        let dist_since = obd.distance_traveled_since_codes_cleared();
        let dist_with = obd.distance_traveled_with_mil();
        let time_since = obd.time_since_codes_cleared();
        let time_with = obd.time_run_with_mil();
        let odometer = obd.odometer();
        let ethanol_fuel_percent = obd.ethanol_fuel_percentage();

        update!(
            &window,
                "Warm-Ups Since Codes Cleared" => warm_ups,
                "Dist. Since Codes Cleared" => dist_since,
                "Dist. With Check Engine Light" => dist_with,
                "Time Since Codes Cleared" => time_since,
                "Time With Check Engine Light" => time_with,
                "Odometer" => odometer,
                "Ethanol Fuel Percentage" => ethanol_fuel_percent,
        );
    });
}

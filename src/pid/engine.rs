use crate::cmd::Command;
use crate::obd::SensorNumber;
use crate::obd::Service;
use crate::obd::OBD;
use std::fmt;

pub enum ValveTrainDesign {
    CamlessValveActuation = 1,
    DualOverheadCam,
    OverheadValve,
    SingleOverheadCam,
    Unknown,
}

impl ValveTrainDesign {
    pub fn from_u8(from: u8) -> Self {
        match from {
            1u8 => Self::CamlessValveActuation,
            2u8 => Self::DualOverheadCam,
            3u8 => Self::OverheadValve,
            4u8 => Self::SingleOverheadCam,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ValveTrainDesign::CamlessValveActuation => "Camless Valve Actuation (CVA)",
            ValveTrainDesign::DualOverheadCam => "Dual Overhead Cam (DOHC)",
            ValveTrainDesign::OverheadValve => "Overhead Valve (OHV)",
            ValveTrainDesign::SingleOverheadCam => "Single Overhead Cam (SOHC)",
            ValveTrainDesign::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for ValveTrainDesign {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(PartialEq, Eq)]
pub enum EngineType {
    SparkIgnition,
    CompressionIgnition,
}

impl EngineType {
    pub fn as_str(&self) -> &str {
        match self {
            EngineType::SparkIgnition => "Internal Combustion Engine",
            EngineType::CompressionIgnition => "Compression Ignition Engine",
        }
    }
}

impl fmt::Display for EngineType {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl OBD {
    pub fn rpm(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010C"));
        ((256.0 * response.a_value()) + response.b_value()) / 4.0
    }

    pub fn engine_load(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0104"));
        response.a_value() / 2.55
    }

    pub fn coolant_temp(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0105"));
        response.a_value() - 40.0
    }

    pub fn coolant_temp_sensors(&mut self) -> (f32, f32) {
        let response = self.query(Command::new_pid(b"0166"));
        let sensors_supported = self.sensors_supported_for(response.a_value() as u8);
        let mut coolant_temp = (0f32, 0f32);

        if sensors_supported.contains(&SensorNumber::Sensor1) {
            coolant_temp.0 = response.b_value() - 40.0;
        }

        if sensors_supported.contains(&SensorNumber::Sensor2) {
            coolant_temp.1 = response.c_value() - 40.0;
        }

        coolant_temp
    }

    pub fn engine_fuel_rate(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"019D"));
        response.a_value()
    }

    pub fn engine_runtime(&mut self) -> f32 {
        if self.get_engine_type() == EngineType::CompressionIgnition {
            return self.engine_runtime_diesel();
        }

        let response = self.query(Command::new_pid(b"011F"));
        (256.0 * response.a_value()) + response.b_value()
    }

    pub fn engine_runtime_diesel(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"017F"));
        let b = response.b_value();
        let c = response.c_value();
        let d = response.d_value();
        let e = response.e_value();
        let b_power = f32::powf(2f32, 24f32);
        let c_power = f32::powf(2f32, 16f32);
        let d_power = f32::powf(2f32, 8f32);

        (b * b_power) + (c * c_power) + (d * d_power) + e
    }

    pub fn engine_mileage(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"01A6"));
        let a = response.a_value();
        let b = response.b_value();
        let c = response.c_value();
        let d = response.d_value();

        let a_power = f32::powf(2f32, 24f32);
        let b_power = f32::powf(2f32, 16f32);
        let c_power = f32::powf(2f32, 8f32);

        ((a * a_power) + (b * b_power) + (c * c_power) + d) / 40.0
    }

    pub fn engine_oil_temp(&mut self, mode: Service) -> f32 {
        let command = match mode {
            Service::Mode01 => Command::new_pid(b"015C"),
            Service::Mode22 => Command::new_arb("221154"),
        };

        let response = self.query(command);
        response.a_value() - 40.0
    }

    pub fn engine_oil_temp_sensors(&mut self) -> (f32, f32) {
        let response = self.query(Command::new_pid(b"0167"));
        let sensors_supported = self.sensors_supported_for(response.a_value() as u8);
        let mut oil_temp = (0f32, 0f32);

        if sensors_supported.contains(&SensorNumber::Sensor1) {
            oil_temp.0 = response.b_value() - 40.0;
        }

        if sensors_supported.contains(&SensorNumber::Sensor2) {
            oil_temp.1 = response.c_value() - 40.0;
        }

        oil_temp
    }

    pub fn engine_oil_pressure(&mut self) -> f32 {
        let response = self.query(Command::new_arb("221470"));
        response.a_value() * 3.985 // kPa
    }

    pub fn drivers_demand_engine_torque(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0161"));
        response.a_value() - 125.0
    }

    pub fn actual_engine_torque(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0162"));
        response.a_value() - 125.0
    }

    pub fn reference_engine_torque(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0163"));
        (256.0 * response.a_value()) + response.b_value()
    }

    pub fn engine_percent_torque_data(&mut self) -> (f32, f32, f32, f32, f32) {
        let response = self.query(Command::new_pid(b"0164"));
        let idle = response.a_value() - 125.0;
        let engine_point_1 = response.b_value() - 125.0;
        let engine_point_2 = response.c_value() - 125.0;
        let engine_point_3 = response.d_value() - 125.0;
        let engine_point_4 = response.e_value() - 125.0;

        (
            idle,
            engine_point_1,
            engine_point_2,
            engine_point_3,
            engine_point_4,
        )
    }

    pub fn get_engine_type(&mut self) -> EngineType {
        let response = self.query(Command::new_pid(b"0101"));
        match response.b_value() as u32 & 0b00001000 {
            0 => EngineType::SparkIgnition,
            _ => EngineType::CompressionIgnition,
        }
    }
}

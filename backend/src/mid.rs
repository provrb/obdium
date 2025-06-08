use std::collections::HashMap;

use crate::{
    engine::CylinderNumber, scalar::{Scalar, Unit}, BankNumber, Command, SensorNumber, OBD
};

pub enum EvapLeakSize {
    Large,  // 0.150"
    Medium, // 0.090"
    Small,  // 0.040"
    Tiny,   // 0.020"
}

pub struct MonitorTest {
    /// Min and maximum allowed values for the specific test
    /// If a value is less than `min` or greater than `max`, the test is considered a FAIL.
    min: u64,
    max: u64,

    value: u64,
    mid: [u8; 4],
}

impl MonitorTest {
    pub fn has_passed(&self) -> bool {
        self.value >= self.min && self.value <= self.max
    }
}

impl OBD {
    pub fn get_supported_mids(&mut self) -> HashMap<String, Vec<String>> {
        self.get_service_supported_pids("06")
    }

    pub fn test_oxygen_sensor_monitor(&mut self, bank: BankNumber, sensor: SensorNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_catalyst_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    // Exhaust gas recirculation
    pub fn test_egr_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    // Variable valve timing
    pub fn test_vvt_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_evap_monitor(&mut self, leak_size: EvapLeakSize) -> MonitorTest {
        todo!()
    }

    pub fn test_purge_flow_monitor(&mut self) -> MonitorTest {
        todo!()
    }

    pub fn test_oxygen_sensor_heater(&mut self, bank: BankNumber, sensor: SensorNumber) -> MonitorTest {
        todo!()
    }
    
    pub fn test_heated_catalyst_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_secondary_air_monitor(&mut self, id: u8) -> MonitorTest {
        todo!()
    }

    pub fn test_fuel_system_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_boost_pressure_control_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_nox_absorber_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_nox_catalyst_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_misfire_monitor_general(&mut self) -> MonitorTest {
        todo!()
    }

    pub fn test_misfire_cylinder_monitor(&mut self, cylinder: CylinderNumber) -> MonitorTest {
        todo!()
    }

    pub fn test_pm_filter_monitor(&mut self, bank: BankNumber) -> MonitorTest {
        todo!()
    }
}
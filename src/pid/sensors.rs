use crate::cmd::Command;
use crate::obd::{SensorNumber, OBD};

impl OBD {
    pub fn vehicle_speed(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010D"));
        response.a_value()
    }

    pub fn timing_advance(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010E"));
        (response.a_value() / 2.0) - 64.0
    }

    pub fn oxygen_sensors_present(&mut self) -> f32 {
        todo!()
    }

    pub fn max_values_for(&mut self) -> (f32, f32, f32, f32) {
        let response = self.query(Command::new_pid(b"014F"));
        (
            response.a_value(),
            response.b_value(),
            response.c_value(),
            response.d_value() * 10.0,
        )
    } // fuel-air equivalance ratio, o2 sensor voltage, current, and instake abs pressure

    pub fn throttle_position(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0111"));
        response.a_value() * (100.0 / 255.0)
    }

    pub fn relative_throttle_pos(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0145"));
        (100.0 / 255.0) * response.a_value()
    }

    pub fn abs_throttle_position_b(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0147"));
        (100.0 / 255.0) * response.a_value()
    }

    pub fn abs_throttle_position_c(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0148"));
        (100.0 / 255.0) * response.a_value()
    }

    // Accelerator pedal position d
    pub fn acc_pedal_position_d(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0149"));
        (100.0 / 255.0) * response.a_value()
    }

    // Accelerator pedal position e
    pub fn acc_pedal_position_e(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"014A"));
        (100.0 / 255.0) * response.a_value()
    }

    // Accelerator pedal position f
    pub fn acc_pedal_position_f(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"014B"));
        (100.0 / 255.0) * response.a_value()
    }

    pub(crate) fn sensors_supported_for(&self, byte: u8) -> Vec<SensorNumber> {
        let s1_supported = (byte & 0b0000_0001) != 0;
        let s2_supported = (byte & 0b0000_0010) != 0;
        let s3_supported = (byte & 0b0000_0100) != 0;
        let s4_supported = (byte & 0b0000_1000) != 0;
        let s5_supported = (byte & 0b0001_0000) != 0;
        let s6_supported = (byte & 0b0010_0000) != 0;
        let s7_supported = (byte & 0b0100_0000) != 0;
        let s8_supported = (byte & 0b1000_0000) != 0;

        // FIXME: Refactor this terrible code
        let mut sensors_supported = Vec::new();
        if s1_supported {
            sensors_supported.push(SensorNumber::Sensor1);
        }
        if s2_supported {
            sensors_supported.push(SensorNumber::Sensor2);
        }
        if s3_supported {
            sensors_supported.push(SensorNumber::Sensor3);
        }
        if s4_supported {
            sensors_supported.push(SensorNumber::Sensor4);
        }
        if s5_supported {
            sensors_supported.push(SensorNumber::Sensor5);
        }
        if s6_supported {
            sensors_supported.push(SensorNumber::Sensor6);
        }
        if s7_supported {
            sensors_supported.push(SensorNumber::Sensor7);
        }
        if s8_supported {
            sensors_supported.push(SensorNumber::Sensor8);
        }

        sensors_supported
    }
}

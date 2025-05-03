use crate::cmd::Command;
use crate::obd::OBD;

impl OBD {
    pub fn vehicle_speed(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010D")).unwrap_or_default();
        response.a_value()
    }

    pub fn timing_advance(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"010E")).unwrap_or_default();
        (response.a_value() / 2.0) - 64.0
    }

    pub fn oxygen_sensors_present(&mut self) -> f32 {
        todo!()
    }

    pub fn max_values_for(&mut self) -> (f32, f32, f32, f32) {
        let response = self.query(Command::new_pid(b"014F")).unwrap_or_default();
        (
            response.a_value(),
            response.b_value(),
            response.c_value(),
            response.d_value() * 10.0,
        )
    } // fuel-air equivalance ratio, o2 sensor voltage, current, and instake abs pressure

    pub fn throttle_position(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0111")).unwrap_or_default();
        response.a_value() * (100.0 / 255.0)
    }

    pub fn relative_throttle_pos(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0145")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

    pub fn abs_throttle_position_b(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0147")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

    pub fn abs_throttle_position_c(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0148")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

    // Accelerator pedal position d
    pub fn acc_pedal_position_d(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"0149")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

    // Accelerator pedal position e
    pub fn acc_pedal_position_e(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"014A")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }

        // Accelerator pedal position f
    pub fn acc_pedal_position_f(&mut self) -> f32 {
        let response = self.query(Command::new_pid(b"014B")).unwrap_or_default();
        (100.0 / 255.0) * response.a_value()
    }
}

use crate::{
    Command,
    SensorNumber, OBD,
    scalar::{Scalar, Unit}
};

impl OBD {
    pub fn vehicle_speed(&mut self) -> Scalar {
        self.query(Command::new_pid(b"010D"))
            .map_no_data(|r| Scalar::new(r.a_value(), Unit::KilometersPerHour))
    }

    pub fn timing_advance(&mut self) -> Scalar {
        self.query(Command::new_pid(b"010E"))
            .map_no_data(|r| Scalar::new((r.a_value() / 2.0) - 64.0, Unit::Degrees))
    }

    /// Fuel-air equivalance ratio, o2 sensor voltage, current, and instake abs pressure
    pub fn max_values_for(&mut self) -> (Scalar, Scalar, Scalar, Scalar) {
        let response = self.query(Command::new_pid(b"014F"));
        if *response.get_payload_size() == 0 {
            return (
                Scalar::no_data(),
                Scalar::no_data(),
                Scalar::no_data(),
                Scalar::no_data(),
            );
        }

        (
            Scalar::new(response.a_value(), Unit::Ratio),
            Scalar::new(response.b_value(), Unit::Volts),
            Scalar::new(response.c_value(), Unit::Milliampere),
            Scalar::new(response.d_value() * 10.0, Unit::KiloPascal),
        )
    }

    pub fn throttle_position(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0111"))
            .map_no_data(|r| Scalar::new(r.a_value() * (100.0 / 255.0), Unit::Percent))
    }

    pub fn relative_throttle_pos(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0145"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    pub fn abs_throttle_position_b(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0147"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    pub fn abs_throttle_position_c(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0148"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    // Accelerator pedal position d
    pub fn acc_pedal_position_d(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0149"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    // Accelerator pedal position e
    pub fn acc_pedal_position_e(&mut self) -> Scalar {
        self.query(Command::new_pid(b"014A"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    // Accelerator pedal position f
    pub fn acc_pedal_position_f(&mut self) -> Scalar {
        self.query(Command::new_pid(b"014B"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
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

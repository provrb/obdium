use crate::{
    scalar::{Scalar, Unit},
    BankNumber, Command, SensorNumber, OBD,
};

impl OBD {
    // Commanded exhaust gas recirculation
    pub fn commanded_egr(&mut self) -> Scalar {
        self.query(Command::new_pid(b"012C"))
            .map_no_data(|r| Scalar::new((100.0 / 255.0) * r.a_value(), Unit::Percent))
    }

    // Exhaust gas recirculation error
    pub fn egr_error(&mut self) -> Scalar {
        self.query(Command::new_pid(b"012D"))
            .map_no_data(|r| Scalar::new(((100.0 / 128.0) * r.a_value()) - 100.0, Unit::Percent))
    }

    pub fn catalyst_temp(&mut self, bank: BankNumber, sensor: SensorNumber) -> Scalar {
        let command = match (bank, sensor) {
            (BankNumber::Bank1, SensorNumber::Sensor1) => Command::new_pid(b"013C"),
            (BankNumber::Bank2, SensorNumber::Sensor1) => Command::new_pid(b"013D"),
            (BankNumber::Bank1, SensorNumber::Sensor2) => Command::new_pid(b"013E"),
            (BankNumber::Bank2, SensorNumber::Sensor2) => Command::new_pid(b"013F"),
            _ => {
                println!("catalyst temperature only supports bank 1, bank 2, sensor 1, and sensor 2 queries.");
                return Scalar::no_data();
            }
        };

        self.query(command).map_no_data(|r| {
            Scalar::new(
                (((256.0 * r.a_value()) + r.b_value()) / 10.0) - 40.0,
                Unit::Celsius,
            )
        })
    }

    pub fn boost_guage_pressure(&mut self) -> Scalar {
        let map = self.intake_manifold_abs_pressure();
        let baro = self.abs_barometric_pressure();

        // result in kPa, gets converted to PSI. if conversion fails, do no data
        // conversion never fail. map and baro will always be Unit::Kilopascal
        (map - baro).to(Unit::PSI).unwrap_or(Scalar::no_data())
    }

    pub fn turbocharger_rpm(&mut self) -> Scalar {
        self.query(Command::new_pid(b"0174"))
            .map_no_data(|r| Scalar::new((256.0 * r.a_value()) + r.b_value(), Unit::RPM))
    }

    // TODO!
    pub fn exhaust_gas_temp(&self) -> Scalar {
        Scalar::no_data()
    }
}

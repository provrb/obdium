use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PidInfo {
    pub supported: bool,
    pub pid: &'static str,
    pub mode: &'static str,
    pub unit: &'static str,
    pub pid_name: &'static str,
    pub formula: &'static str,
}

pub const PID_INFOS: &[PidInfo] = &[
    PidInfo {
        supported: false,
        pid: "01",
        mode: "01",
        unit: "",
        pid_name: "Monitor status since DTCs cleared",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "02",
        mode: "01",
        unit: "",
        pid_name: "DTC that caused freeze frame to be stored",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "03",
        mode: "01",
        unit: "",
        pid_name: "Fuel system status",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "04",
        mode: "01",
        unit: "%",
        pid_name: "Engine load",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "05",
        mode: "01",
        unit: "°C",
        pid_name: "Coolant temp.",
        formula: "A - 40",
    },
    PidInfo {
        supported: false,
        pid: "06",
        mode: "01",
        unit: "%",
        pid_name: "Short term fuel trim (Bank 1)",
        formula: "(100/128 * A) - 100",
    },
    PidInfo {
        supported: false,
        pid: "07",
        mode: "01",
        unit: "%",
        pid_name: "Long term fuel trim (Bank 1)",
        formula: "(100/128 * A) - 100",
    },
    PidInfo {
        supported: false,
        pid: "08",
        mode: "01",
        unit: "%",
        pid_name: "Short term fuel trim (Bank 2)",
        formula: "(100/128 * A) - 100",
    },
    PidInfo {
        supported: false,
        pid: "09",
        mode: "01",
        unit: "%",
        pid_name: "Long term fuel trim (Bank 2)",
        formula: "(100/128 * A) - 100",
    },
    PidInfo {
        supported: false,
        pid: "0A",
        mode: "01",
        unit: "kPa",
        pid_name: "Fuel pressure",
        formula: "3 * A",
    },
    PidInfo {
        supported: false,
        pid: "0B",
        mode: "01",
        unit: "kPa",
        pid_name: "Intake manifold abs. pressure",
        formula: "A",
    },
    PidInfo {
        supported: false,
        pid: "0C",
        mode: "01",
        unit: "RPM",
        pid_name: "Engine speed",
        formula: "((256 * A)+B) / 4",
    },
    PidInfo {
        supported: false,
        pid: "0D",
        mode: "01",
        unit: "km/h",
        pid_name: "Vehicle speed",
        formula: "A",
    },
    PidInfo {
        supported: false,
        pid: "0E",
        mode: "01",
        unit: "°",
        pid_name: "Timing advance",
        formula: "A/2 - 64",
    },
    PidInfo {
        supported: false,
        pid: "0F",
        mode: "01",
        unit: "°C",
        pid_name: "Intake air temp.",
        formula: "A - 40",
    },
    PidInfo {
        supported: false,
        pid: "10",
        mode: "01",
        unit: "g/s",
        pid_name: "MAF airflow rate",
        formula: "((256 * A)+B) / 100",
    },
    PidInfo {
        supported: false,
        pid: "11",
        mode: "01",
        unit: "%",
        pid_name: "Throttle pos.",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "12",
        mode: "01",
        unit: "",
        pid_name: "Commanded secondary air status",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "13",
        mode: "01",
        unit: "",
        pid_name: "Oxygen sensors present (in 2 banks)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "14",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 1 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "15",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 2 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "16",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 3 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "17",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 4 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "18",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 5 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "19",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 6 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "1A",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 7 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "1B",
        mode: "01",
        unit: "(V, %)",
        pid_name: "Oxygen Sensor 8 (A: Voltage B: STFT)",
        formula: "V: A / 200 %: 100/128B - 100",
    },
    PidInfo {
        supported: false,
        pid: "1C",
        mode: "01",
        unit: "",
        pid_name: "OBD standards this vehicle conforms to",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "1D",
        mode: "01",
        unit: "",
        pid_name: "Oxygen sensors present (in 4 banks)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "1E",
        mode: "01",
        unit: "",
        pid_name: "Aux input status",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "1F",
        mode: "01",
        unit: "s",
        pid_name: "Engine runtime (Session)",
        formula: "(256 * A) + B",
    },
    PidInfo {
        supported: false,
        pid: "21",
        mode: "01",
        unit: "km",
        pid_name: "Dist. with check engine light",
        formula: "(256 * A) + B",
    },
    PidInfo {
        supported: false,
        pid: "22",
        mode: "01",
        unit: "kPa",
        pid_name: "Fuel Rail Pressure",
        formula: "0.079(256A + B)",
    },
    PidInfo {
        supported: false,
        pid: "23",
        mode: "01",
        unit: "kPa",
        pid_name: "Fuel Rail Gauge Pressure",
        formula: "10(256A + B)",
    },
    PidInfo {
        supported: false,
        pid: "24",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (1) AFR",
        formula: "2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "24",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (1) Voltage (2)",
        formula: "8/65536(256C+D)",
    },
    PidInfo {
        supported: false,
        pid: "25",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (2) AFR",
        formula: "ratio: 2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "25",
        mode: "01",
        unit: "V",
        pid_name: "O2 Sensor (2) Voltage (2)",
        formula: " 8/65536(256C+D",
    },
    PidInfo {
        supported: false,
        pid: "26",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (3) AFR",
        formula: "2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "26",
        mode: "01",
        unit: "V",
        pid_name: "O2 Sensor (3) Voltage (2)",
        formula: "8/65536(256C+D)",
    },
    PidInfo {
        supported: false,
        pid: "27",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (4) AFR",
        formula: "2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "27",
        mode: "01",
        unit: "V",
        pid_name: "O2 Sensor (4) Voltage (2)",
        formula: "8/65536(256C+D)",
    },
    PidInfo {
        supported: false,
        pid: "28",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (5) AFR (2)",
        formula: "2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "28",
        mode: "01",
        unit: "V",
        pid_name: "O2 Sensor (5) Voltage (2)",
        formula: "8/65536(256C+D)",
    },
    PidInfo {
        supported: false,
        pid: "29",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (6) AFR",
        formula: "2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "29",
        mode: "01",
        unit: "V",
        pid_name: "O2 Sensor (6) Voltage (2)",
        formula: "8/65536(256C+D)",
    },
    PidInfo {
        supported: false,
        pid: "2A",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (7) AFR",
        formula: "2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "2A",
        mode: "01",
        unit: "V",
        pid_name: "O2 Sensor (7) Voltage (2)",
        formula: "8/65536(256C+D)",
    },
    PidInfo {
        supported: false,
        pid: "2B",
        mode: "01",
        unit: "ratio",
        pid_name: "O2 Sensor (8) AFR",
        formula: "2/65536(256A+B)",
    },
    PidInfo {
        supported: false,
        pid: "2B",
        mode: "01",
        unit: "V",
        pid_name: "O2 Sensor (8) Voltage (2)",
        formula: "8/65536(256C+D)",
    },
    PidInfo {
        supported: false,
        pid: "2C",
        mode: "01",
        unit: "%",
        pid_name: "Commanded EGR",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "2D",
        mode: "01",
        unit: "%",
        pid_name: "EGR Error",
        formula: "(100/128 * A) - 100",
    },
    PidInfo {
        supported: false,
        pid: "2E",
        mode: "01",
        unit: "%",
        pid_name: "Commanded EVAP purge",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "2F",
        mode: "01",
        unit: "%",
        pid_name: "Fuel Tank Level Input",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "30",
        mode: "01",
        unit: "",
        pid_name: "Warm-ups since codes cleared",
        formula: "A",
    },
    PidInfo {
        supported: false,
        pid: "31",
        mode: "01",
        unit: "km",
        pid_name: "Dist. since codes cleared",
        formula: "(256 * A)+B",
    },
    PidInfo {
        supported: false,
        pid: "32",
        mode: "01",
        unit: "Pa",
        pid_name: "EVAP System Vapor Pressure",
        formula: "((256 * A)+B) / 4",
    },
    PidInfo {
        supported: false,
        pid: "33",
        mode: "01",
        unit: "kPa",
        pid_name: "Absolute Barometric Pressure",
        formula: "A",
    },
    PidInfo {
        supported: false,
        pid: "34",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 1 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "35",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 2 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "36",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 3 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "37",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 4 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "38",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 5 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "39",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 6 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "3A",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 7 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "3B",
        mode: "01",
        unit: "(ratio, mA)",
        pid_name: "Oxygen Sensor 8 (AB: AFR CD: Current)",
        formula: "(ratio: 2/65536(256A+B) mA: ((256C + D) / 256) - 128",
    },
    PidInfo {
        supported: false,
        pid: "3C",
        mode: "01",
        unit: "°C",
        pid_name: "Catalyst Temp. (Bank 1: Sensor 1)",
        formula: "(((256 * A)+B) / 10) - 40",
    },
    PidInfo {
        supported: false,
        pid: "3D",
        mode: "01",
        unit: "°C",
        pid_name: "Catalyst Temp. (Bank 2: Sensor 1)",
        formula: "(((256 * A)+B) / 10) - 40",
    },
    PidInfo {
        supported: false,
        pid: "3E",
        mode: "01",
        unit: "°C",
        pid_name: "Catalyst Temp. (Bank 1: Sensor 2)",
        formula: "(((256 * A)+B) / 10) - 40",
    },
    PidInfo {
        supported: false,
        pid: "3F",
        mode: "01",
        unit: "°C",
        pid_name: "Catalyst Temp. (Bank 2: Sensor 2)",
        formula: "(((256 * A)+B) / 10) - 40",
    },
    PidInfo {
        supported: false,
        pid: "41",
        mode: "01",
        unit: "",
        pid_name: "Monitor status this drive cycle",
        formula: "",
    },
    PidInfo {
        supported: true,
        pid: "42",
        mode: "01",
        unit: "V",
        pid_name: "Control module voltage",
        formula: "((256 * A)+B) / 1000",
    },
    PidInfo {
        supported: false,
        pid: "43",
        mode: "01",
        unit: "%",
        pid_name: "Absolute load value",
        formula: "(100/255) * (256A + B)",
    },
    PidInfo {
        supported: false,
        pid: "44",
        mode: "01",
        unit: "ratio",
        pid_name: "Commanded Air-Fuel Equivalence Ratio",
        formula: "(2/65536) * (256A + B)",
    },
    PidInfo {
        supported: false,
        pid: "45",
        mode: "01",
        unit: "%",
        pid_name: "Relative throttle pos.",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "46",
        mode: "01",
        unit: "°C",
        pid_name: "Ambient air temp.",
        formula: "A - 40",
    },
    PidInfo {
        supported: false,
        pid: "47",
        mode: "01",
        unit: "%",
        pid_name: "Abs. throttle pos. (B)",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "4D",
        mode: "01",
        unit: "mins",
        pid_name: "Time with check engine light",
        formula: "256A + B",
    },
    PidInfo {
        supported: false,
        pid: "4F",
        mode: "01",
        unit: "ratio, V, mA, kPa",
        pid_name:
            "Max. value for AFR, O2 sensor voltage and current, and intake manifold abs. pressure",
        formula: "A, B, C, D * 10",
    },
    PidInfo {
        supported: false,
        pid: "50",
        mode: "01",
        unit: "g/s",
        pid_name: "MAF maximum airflow rate",
        formula: "A * 10",
    },
    PidInfo {
        supported: false,
        pid: "51",
        mode: "01",
        unit: "",
        pid_name: "Fuel Type",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "52",
        mode: "01",
        unit: "%",
        pid_name: "Ethanol fuel percentage",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "53",
        mode: "01",
        unit: "kPa",
        pid_name: "Absolute Evap system Vapor Pressure",
        formula: "((256 * A)+B) / 200",
    },
    PidInfo {
        supported: false,
        pid: "54",
        mode: "01",
        unit: "Pa",
        pid_name: "Evap system vapor pressure",
        formula: "(256 * A) + B",
    },
    PidInfo {
        supported: false,
        pid: "55",
        mode: "01",
        unit: "%",
        pid_name: "Short term secondary oxygen sensor trim, A: bank 1, B: bank 3",
        formula: "100/128(A OR B) - 100",
    },
    PidInfo {
        supported: false,
        pid: "56",
        mode: "01",
        unit: "%",
        pid_name: "Long term secondary oxygen sensor trim, A: bank 1, B: bank 3",
        formula: "100/128(A OR B) - 100",
    },
    PidInfo {
        supported: false,
        pid: "57",
        mode: "01",
        unit: "%",
        pid_name: "Short term secondary oxygen sensor trim, A: bank 2, B: bank 4",
        formula: "100/128(A OR B) - 100",
    },
    PidInfo {
        supported: false,
        pid: "58",
        mode: "01",
        unit: "%",
        pid_name: "Long term secondary oxygen sensor trim, A: bank 2, B: bank 4",
        formula: "100/128(A OR B) - 100",
    },
    PidInfo {
        supported: false,
        pid: "59",
        mode: "01",
        unit: "kPa",
        pid_name: "Fuel rail absolute pressure",
        formula: "10(256A + B)",
    },
    PidInfo {
        supported: false,
        pid: "5A",
        mode: "01",
        unit: "%",
        pid_name: "Relative accelerator pedal position",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "5B",
        mode: "01",
        unit: "%",
        pid_name: "Hybrid battery pack remaining life",
        formula: "100/255 * A",
    },
    PidInfo {
        supported: false,
        pid: "5C",
        mode: "01",
        unit: "°C",
        pid_name: "Engine oil temp. (mode 01)",
        formula: "A - 40",
    },
    PidInfo {
        supported: false,
        pid: "5D",
        mode: "01",
        unit: "°",
        pid_name: "Fuel injection timing",
        formula: "(((256 * A)+B) / 128) - 210",
    },
    PidInfo {
        supported: false,
        pid: "5E",
        mode: "01",
        unit: "L/h",
        pid_name: "Engine fuel rate",
        formula: "((256 * A)+B) / 20",
    },
    PidInfo {
        supported: false,
        pid: "5F",
        mode: "01",
        unit: "",
        pid_name: "Emission requirements to which vehicle is designed",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "61",
        mode: "01",
        unit: "%",
        pid_name: "Drivers demand engine torque",
        formula: "A - 125",
    },
    PidInfo {
        supported: false,
        pid: "62",
        mode: "01",
        unit: "%",
        pid_name: "Actual engine torque",
        formula: "A - 125",
    },
    PidInfo {
        supported: false,
        pid: "63",
        mode: "01",
        unit: "Nm",
        pid_name: "Reference engine torque",
        formula: "256A + B",
    },
    PidInfo {
        supported: false,
        pid: "64",
        mode: "01",
        unit: "%",
        pid_name: "Engine percent torque data",
        formula: "Subtract 125 from A - E",
    },
    PidInfo {
        supported: false,
        pid: "65",
        mode: "01",
        unit: "",
        pid_name: "Auxiliary input / output supported",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "66",
        mode: "01",
        unit: "g/s",
        pid_name: "Mass air flow sensor",
        formula: "{A0}== Sensor A Supported",
    },
    PidInfo {
        supported: false,
        pid: "67",
        mode: "01",
        unit: "°C",
        pid_name: "Engine coolant temperature",
        formula: "{A0}== Sensor 1 Supported",
    },
    PidInfo {
        supported: false,
        pid: "68",
        mode: "01",
        unit: "°C",
        pid_name: "Intake air temperature sensor",
        formula: "{A0}== Sensor 1 Supported",
    },
    PidInfo {
        supported: false,
        pid: "6A",
        mode: "01",
        unit: "",
        pid_name: "Commanded Diesel intake air flow control and relative intake air flow position",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "6B",
        mode: "01",
        unit: "",
        pid_name: "Exhaust gas recirculation temperature",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "6C",
        mode: "01",
        unit: "",
        pid_name: "Commanded throttle actuator control and relative throttle position",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "6D",
        mode: "01",
        unit: "",
        pid_name: "Fuel pressure control system",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "6E",
        mode: "01",
        unit: "",
        pid_name: "Injection pressure control system",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "6F",
        mode: "01",
        unit: "",
        pid_name: "Turbocharger compressor inlet pressure",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "70",
        mode: "01",
        unit: "",
        pid_name: "Boost pressure control",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "71",
        mode: "01",
        unit: "",
        pid_name: "Variable Geometry turbo (VGT) control",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "72",
        mode: "01",
        unit: "",
        pid_name: "Wastegate control",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "73",
        mode: "01",
        unit: "",
        pid_name: "Exhaust pressure",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "74",
        mode: "01",
        unit: "RPM",
        pid_name: "Turbocharger RPM",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "75",
        mode: "01",
        unit: "°C",
        pid_name: "Turbocharger temperature",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "76",
        mode: "01",
        unit: "°C",
        pid_name: "Turbocharger temperature",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "77",
        mode: "01",
        unit: "°C",
        pid_name: "Charge air cooler temperature (CACT)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "78",
        mode: "01",
        unit: "°C",
        pid_name: "Exhaust Gas temperature (EGT) Bank 1",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "79",
        mode: "01",
        unit: "°C",
        pid_name: "Exhaust Gas temperature (EGT) Bank 2",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "7A",
        mode: "01",
        unit: "",
        pid_name: "Diesel particulate filter (DPF)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "7B",
        mode: "01",
        unit: "",
        pid_name: "Diesel particulate filter (DPF)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "7C",
        mode: "01",
        unit: "°C",
        pid_name: "Diesel Particulate filter (DPF) temperature",
        formula: "(((256 * A)+B) / 10) - 40",
    },
    PidInfo {
        supported: false,
        pid: "7D",
        mode: "01",
        unit: "",
        pid_name: "NOx NTE",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "7E",
        mode: "01",
        unit: "",
        pid_name: "PM NTE",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "7F",
        mode: "01",
        unit: "s",
        pid_name: "Engine runtime",
        formula: "B(2^24) + C(2^16) + D(2^8) + E",
    },
    PidInfo {
        supported: false,
        pid: "81",
        mode: "01",
        unit: "",
        pid_name: "Engine runtime for Auxiliary Emissions Control Device(AECD)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "82",
        mode: "01",
        unit: "",
        pid_name: "Engine runtime for Auxiliary Emissions Control Device(AECD)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "83",
        mode: "01",
        unit: "",
        pid_name: "NOx sensor",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "84",
        mode: "01",
        unit: "",
        pid_name: "Manifold surface temperature",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "85",
        mode: "01",
        unit: "%",
        pid_name: "NOx reagent system",
        formula: "100/255 * F",
    },
    PidInfo {
        supported: false,
        pid: "86",
        mode: "01",
        unit: "",
        pid_name: "Particulate matter (PM) sensor",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "88",
        mode: "01",
        unit: "",
        pid_name: "SCR Induce System",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "89",
        mode: "01",
        unit: "",
        pid_name: "Run Time for AECD #11-#15",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "8A",
        mode: "01",
        unit: "",
        pid_name: "Run Time for AECD #16-#20",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "8B",
        mode: "01",
        unit: "",
        pid_name: "Diesel Aftertreatment",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "8C",
        mode: "01",
        unit: "",
        pid_name: "O2 Sensor (Wide Range)",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "8D",
        mode: "01",
        unit: "%",
        pid_name: "Throttle Position G",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "8E",
        mode: "01",
        unit: "%",
        pid_name: "Engine Friction - Percent Torque",
        formula: "A - 125",
    },
    PidInfo {
        supported: false,
        pid: "8F",
        mode: "01",
        unit: "",
        pid_name: "PM Sensor Bank 1 & 2",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "90",
        mode: "01",
        unit: "h",
        pid_name: "WWH-OBD Vehicle OBD System Information",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "91",
        mode: "01",
        unit: "h",
        pid_name: "WWH-OBD Vehicle OBD System Information",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "92",
        mode: "01",
        unit: "",
        pid_name: "Fuel System Control",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "93",
        mode: "01",
        unit: "h",
        pid_name: "WWH-OBD Vehicle OBD Counters support",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "94",
        mode: "01",
        unit: "",
        pid_name: "NOx Warning And Inducement System",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "98",
        mode: "01",
        unit: "°C",
        pid_name: "Exhaust Gas Temperature Sensor",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "99",
        mode: "01",
        unit: "°C",
        pid_name: "Exhaust Gas Temperature Sensor",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "9A",
        mode: "01",
        unit: "",
        pid_name: "Hybrid/EV Vehicle System Data, Battery, Voltage",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "9B",
        mode: "01",
        unit: "%",
        pid_name: "Diesel Exhaust Fluid Sensor Data",
        formula: "100/255 * D",
    },
    PidInfo {
        supported: false,
        pid: "9C",
        mode: "01",
        unit: "",
        pid_name: "O2 Sensor Data",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "9D",
        mode: "01",
        unit: "g/s",
        pid_name: "Engine Fuel Rate",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "9E",
        mode: "01",
        unit: "kg/h",
        pid_name: "Engine Exhaust Flow Rate",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "9F",
        mode: "01",
        unit: "",
        pid_name: "Fuel System Percentage Use",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "A1",
        mode: "01",
        unit: "ppm",
        pid_name: "NOx Sensor Corrected Data",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "A2",
        mode: "01",
        unit: "mg/stroke",
        pid_name: "Cylinder Fuel Rate",
        formula: "((256 * A)+B) / 32",
    },
    PidInfo {
        supported: false,
        pid: "A3",
        mode: "01",
        unit: "Pa",
        pid_name: "Evap System Vapor Pressure",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "A4",
        mode: "01",
        unit: "ratio",
        pid_name: "Transmission Actual Gear",
        formula: "((256 * C) + D) / 1000",
    },
    PidInfo {
        supported: false,
        pid: "A5",
        mode: "01",
        unit: "%",
        pid_name: "Commanded Diesel Exhaust Fluid Dosing",
        formula: "B / 2",
    },
    PidInfo {
        supported: false,
        pid: "A6",
        mode: "01",
        unit: "",
        pid_name: "Odometer",
        formula: "(A(2^24) + B(2^16) + C(2^8) + D) / 10",
    },
    PidInfo {
        supported: false,
        pid: "A7",
        mode: "01",
        unit: "",
        pid_name: "NOx Sensor Concentration Sensors 3 and 4",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "A8",
        mode: "01",
        unit: "",
        pid_name: "NOx Sensor Corrected Concentration Sensors 3 and 4",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "A9",
        mode: "01",
        unit: "",
        pid_name: "ABS Disable Switch State",
        formula: "{A0}= 1:Supported; 0:Unsupported",
    },
    PidInfo {
        supported: false,
        pid: "C3",
        mode: "01",
        unit: "%",
        pid_name: "Fuel Level Input A/B",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "C4",
        mode: "01",
        unit: "seconds / Count",
        pid_name: "Exhaust Particulate Control System Diagnostic Time/Count",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "C5",
        mode: "01",
        unit: "kPa",
        pid_name: "Fuel Pressure A and B",
        formula: "",
    },
    PidInfo {
        supported: false,
        pid: "C7",
        mode: "01",
        unit: "km",
        pid_name: "Distance Since Reflash or Module Replacement",
        formula: "",
    },
];

// "0600", "Supported MIDs [01-20]"
// "0601", "O2 Sensor Monitor Bank 1 - Sensor 1"
// "0602", "O2 Sensor Monitor Bank 1 - Sensor 2"
// "0603", "O2 Sensor Monitor Bank 1 - Sensor 3"
// "0604", "O2 Sensor Monitor Bank 1 - Sensor 4"
// "0605", "O2 Sensor Monitor Bank 2 - Sensor 1"
// "0606", "O2 Sensor Monitor Bank 2 - Sensor 2"
// "0607", "O2 Sensor Monitor Bank 2 - Sensor 3"
// "0608", "O2 Sensor Monitor Bank 2 - Sensor 4"
// "0609", "O2 Sensor Monitor Bank 3 - Sensor 1"
// "060A", "O2 Sensor Monitor Bank 3 - Sensor 2"
// "060B", "O2 Sensor Monitor Bank 3 - Sensor 3"
// "060C", "O2 Sensor Monitor Bank 3 - Sensor 4"
// "060D", "O2 Sensor Monitor Bank 4 - Sensor 1"
// "060E", "O2 Sensor Monitor Bank 4 - Sensor 2"
// "060F", "O2 Sensor Monitor Bank 4 - Sensor 3"
// "0610", "O2 Sensor Monitor Bank 4 - Sensor 4"
// "0620", "Supported MIDs [21-40]"
// "0621", "Catalyst Monitor Bank 1"
// "0622", "Catalyst Monitor Bank 2"
// "0623", "Catalyst Monitor Bank 3"
// "0624", "Catalyst Monitor Bank 4"
// "0631", "EGR Monitor Bank 1"
// "0632", "EGR Monitor Bank 2"
// "0633", "EGR Monitor Bank 3"
// "0634", "EGR Monitor Bank 4"
// "0635", "VVT Monitor Bank 1"
// "0636", "VVT Monitor Bank 2"
// "0637", "VVT Monitor Bank 3"
// "0638", "VVT Monitor Bank 4"
// "0639", "EVAP Monitor (Cap Off / 0.150\")"
// "063A", "EVAP Monitor (0.090\")"
// "063B", "EVAP Monitor (0.040\")"
// "063C", "EVAP Monitor (0.020\")"
// "063D", "Purge Flow Monitor"
// "0640", "Supported MIDs [41-60]"
// "0641", "O2 Sensor Heater Monitor Bank 1 - Sensor 1"
// "0642", "O2 Sensor Heater Monitor Bank 1 - Sensor 2"
// "0643", "O2 Sensor Heater Monitor Bank 1 - Sensor 3"
// "0644", "O2 Sensor Heater Monitor Bank 1 - Sensor 4"
// "0645", "O2 Sensor Heater Monitor Bank 2 - Sensor 1"
// "0646", "O2 Sensor Heater Monitor Bank 2 - Sensor 2"
// "0647", "O2 Sensor Heater Monitor Bank 2 - Sensor 3"
// "0648", "O2 Sensor Heater Monitor Bank 2 - Sensor 4"
// "0649", "O2 Sensor Heater Monitor Bank 3 - Sensor 1"
// "064A", "O2 Sensor Heater Monitor Bank 3 - Sensor 2"
// "064B", "O2 Sensor Heater Monitor Bank 3 - Sensor 3"
// "064C", "O2 Sensor Heater Monitor Bank 3 - Sensor 4"
// "064D", "O2 Sensor Heater Monitor Bank 4 - Sensor 1"
// "064E", "O2 Sensor Heater Monitor Bank 4 - Sensor 2"
// "064F", "O2 Sensor Heater Monitor Bank 4 - Sensor 3"
// "0650", "O2 Sensor Heater Monitor Bank 4 - Sensor 4"
// "0660", "Supported MIDs [61-80]"
// "0661", "Heated Catalyst Monitor Bank 1"
// "0662", "Heated Catalyst Monitor Bank 2"
// "0663", "Heated Catalyst Monitor Bank 3"
// "0664", "Heated Catalyst Monitor Bank 4"
// "0671", "Secondary Air Monitor 1"
// "0672", "Secondary Air Monitor 2"
// "0673", "Secondary Air Monitor 3"
// "0674", "Secondary Air Monitor 4"
// "0680", "Supported MIDs [81-A0]"
// "0681", "Fuel System Monitor Bank 1"
// "0682", "Fuel System Monitor Bank 2"
// "0683", "Fuel System Monitor Bank 3"
// "0684", "Fuel System Monitor Bank 4"
// "0685", "Boost Pressure Control Monitor Bank 1"
// "0686", "Boost Pressure Control Monitor Bank 1"
// "0690", "NOx Absorber Monitor Bank 1"
// "0691", "NOx Absorber Monitor Bank 2"
// "0698", "NOx Catalyst Monitor Bank 1"
// "0699", "NOx Catalyst Monitor Bank 2"
// "06A0", "Supported MIDs [A1-C0]"
// "06A1", "Misfire Monitor General Data"
// "06A2", "Misfire Cylinder 1 Data"
// "06A3", "Misfire Cylinder 2 Data"
// "06A4", "Misfire Cylinder 3 Data"
// "06A5", "Misfire Cylinder 4 Data"
// "06A6", "Misfire Cylinder 5 Data"
// "06A7", "Misfire Cylinder 6 Data"
// "06A8", "Misfire Cylinder 7 Data"
// "06A9", "Misfire Cylinder 8 Data"
// "06AA", "Misfire Cylinder 9 Data"
// "06AB", "Misfire Cylinder 10 Data"
// "06AC", "Misfire Cylinder 11 Data"
// "06AD", "Misfire Cylinder 12 Data"
// "06B0", "PM Filter Monitor Bank 1"
// "06B1", "PM Filter Monitor Bank 2"

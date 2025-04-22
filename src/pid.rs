use crate::obd::OBD;

#[derive(Debug, Default)]
pub struct Response {
    /**
     * Full response including request and
     * multiple responses from different ecus, if any.
     * Excludes '>' and '\r' characters. Formatted as hexadecimal.
     *
     * (e.g: "01 0C 41 0C 11 D0 41 0C 11 D0")
     *
     * Where:
     *     01 0C - the request sent (engine rpms)
     *     41 0C - a response to the request sent
     *     11 D0 - the data. (11: a-value, D0: b-value).
     *
     * The information is repeated as this is another response
     * from a second ECU.
     */
    pub(crate) raw_response: Option<String>, // Hex Response from ECU

    pub(crate) payload: Option<String>,

    /* Original service num in request  */
    pub(crate) service: [u8; 2],

    /* Original PID in request */
    pub(crate) pid: [u8; 2],

    /* Number of ECUs responded to the request  */
    pub(crate) ecu_count: usize,

    /**
     * How many bytes in the response.
     * Excluding request characters and duplicate ECU responses.
     *
     * (e.g: "01 0C 41 0C 11 D0 41 0C 11 D0")
     *
     * This above example would have a payload size of
     * 4 bytes. (41 0C 11 D0)
     */
    pub(crate) payload_size: usize,
}

impl Response {
    pub fn new(raw: String) -> Self {
        Self {
            raw_response: Some(raw),
            payload: None,
            service: [0u8; 2],
            pid: [0u8; 2],
            ecu_count: 0,
            payload_size: 0,
        }
    }

    pub fn full_response(&self) -> Option<String> {
        self.raw_response.clone()
    }

    pub fn get_payload(&self) -> Option<String> {
        if self.payload.is_none() && self.raw_response.is_some() {
            // self.payload likely has not been updated. Update it now.
            self.payload_from_response();
        }

        self.payload.clone()
    }

    pub fn get_payload_bytes(&self) -> Vec<Vec<u8>> {
        let clean = match self.get_payload() {
            Some(resp) => {
                if resp.len() < 6 {
                    println!("invalid response payload: {resp}");
                    return Vec::new();
                }
                resp[6..].to_string()
            },
            None => return Vec::new(),
        };

        let hex: Vec<Vec<u8>> = clean
            .as_bytes()
            .split(|&c| c == b' ')
            .map(|chunk| chunk.to_vec())
            .collect();

        hex
    }

    // TODO:
    pub fn a_value(&self) -> f32 {
        // let a = self.get_payload_bytes().get(0);
        0f32
    }

    pub fn b_value() -> f32 {
        0f32
    }

    pub fn c_value() -> f32 {
        0f32
    }

    pub fn d_value() -> f32 {
        0f32
    }

    pub(crate) fn payload_from_response(&self) -> String {
        let response = match &self.raw_response {
            Some(resp) => resp.to_owned(),
            None => return String::default(),
        };

        let stripped = &response.replace(" ", "");

        let chunks = stripped.as_bytes().chunks(2);
        let mut responses = 0;
        let mut payload = String::new();

        for pair in chunks {
            if pair.len() != 2 {
                continue;
            }

            if pair[0] == b'4' && pair[1] == self.service[1] {
                responses += 1;
                if responses > 1 {
                    break;
                }
            }

            if responses == 1 {
                // reached the start of a response
                // start appending response
                payload.push(pair[0] as char);
                payload.push(pair[1] as char);
            }
        }

        OBD::format_response(&payload)
    }
}

use std::collections::HashSet;

use crate::obd::OBD;

#[derive(Debug, Copy, Clone)]
enum PayloadComponent {
    A,
    B,
    C,
    D,
}

impl PayloadComponent {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

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

    /**
     * Information pulled from the raw_response.
     * This string is of size, 'payload_size'
     *
     * Includes the response to the request sent,
     * and the data (a, b, c, d values...)
     *
     * A raw response may look like:
     * 01 0C 41 0C 11 D0 41 0C 11 D0
     *
     * Where:
     *     01 0C - the request sent (engine rpms)
     *     41 0C - a response to the request sent
     *     11 D0 - the data. (11: a-value, D0: b-value).
     *
     * Then the payload would be:
     * 41 0C 11 D0
     *
     * Excluding the second response from a second ECU (41 0C 11 D0)
     * and excluding the request sent from us.
     */
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
            return Some(self.payload_from_response());
        }

        self.payload.clone()
    }

    pub fn get_payload_components(&self) -> Vec<Vec<u8>> {
        let clean = match self.get_payload() {
            Some(resp) => {
                if resp.len() < 6 {
                    println!("invalid response payload: {resp}");
                    return Vec::new();
                }
                resp[6..].to_string()
            }
            None => return Vec::new(),
        };

        let hex: Vec<Vec<u8>> = clean
            .as_bytes()
            .split(|&c| c == b' ')
            .map(|chunk| chunk.to_vec())
            .collect();

        hex
    }

    pub fn a_value(&self) -> f32 {
        self.get_component(PayloadComponent::A)
    }

    pub fn b_value(&self) -> f32 {
        self.get_component(PayloadComponent::B)
    }

    pub fn c_value(&self) -> f32 {
        self.get_component(PayloadComponent::C)
    }

    pub fn d_value(&self) -> f32 {
        self.get_component(PayloadComponent::D)
    }

    fn get_component(&self, value: PayloadComponent) -> f32 {
        let components = self.get_payload_components();
        let bytes = components.get(value.as_usize()).unwrap_or_else(|| {
            panic!(
                "warning; payload does not have a '{value:?} value' ({})",
                value.as_usize()
            );
        });

        let utf8 = std::str::from_utf8(&bytes).unwrap_or_else(|err| {
            panic!("error; converting value {bytes:?} to utf-8. {err}");
        });

        let decimal = u8::from_str_radix(utf8, 16).unwrap_or_else(|err| {
            panic!(
                "error; converting utf8 to u8. bytes: {bytes:?}. value: {value:?}. error: {err}."
            );
        });

        decimal as f32
    }

    pub(crate) fn payload_from_response(&self) -> String {
        let response = match &self.raw_response {
            Some(resp) => resp.to_owned(),
            None => return String::default(),
        };

        let stripped = &response.replace(" ", "");

        let mut chunks = stripped.as_bytes().chunks(2).peekable();
        let mut first_response_found = false;
        let mut payload = String::new();
        let mut pairs = 0;

        while let Some(pair) = chunks.next() {
            if pair.len() != 2 {
                continue;
            }

            if pair[0] == b'4' && pair[1] == self.service[1] {
                if first_response_found && pairs > self.payload_size {
                    break;
                }
                first_response_found = true;
            }

            if first_response_found {
                // reached the start of a response
                // start appending response
                payload.push(pair[0] as char);
                payload.push(pair[1] as char);
            }
            pairs += 1;
        }

        OBD::format_response(&payload)
    }
}

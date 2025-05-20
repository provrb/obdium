use crate::obd::OBD;
use crate::scalar::Scalar;

#[derive(Debug, Copy, Clone)]
enum PayloadComponent {
    A,
    B,
    C,
    D,
    E,
}

impl PayloadComponent {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Default, Clone)]
pub struct Response {

    /// Full response including request and
    /// multiple responses from different ecus, if any.
    /// Excludes escape characters, and ECU names. Formatted as hexadecimal.
    /// 
    /// (e.g: "01 0C 41 0C 11 D0 41 0C 11 D0")
    /// 
    /// Where:
    ///     01 0C - the request sent (engine rpms)
    ///     41 0C - a response to the request sent
    ///     11 D0 - the data. (11: a-value, D0: b-value).
    /// 
    /// The information is repeated as this is another response
    /// from a second ECU.
    /// 
    pub(crate) formatted_response: Option<String>, // Hex Response from ECU

    /// Similiar to 'formatted_response', except that
    /// this contains escape characters like '\n', '\r' and ECU names.
    /// Formatted as hexidecimal.
    /// 
    /// (e.g: "E8 06 41 01 00 07 A1 00 \n\n"")
    /// 
    /// When printing, be sure to use String::escape_debug to avoid
    /// unknown behaviour in the terminal caused by escape characters like '\r'
    pub(crate) raw_response: Option<String>,

    /// Information pulled from the raw_response.
    /// This string is of size, 'payload_size'
    ///
    /// Includes the response to the request sent,
    /// and the data (a, b, c, d values...)
    ///
    /// A raw response may look like:
    /// 01 0C 41 0C 11 D0 41 0C 11 D0
    ///
    /// Where:
    ///     01 0C - the request sent (engine rpms)
    ///     41 0C - a response to the request sent
    ///     11 D0 - the data. (11: a-value, D0: b-value).
    ///
    /// Then the payload would be:
    /// 41 0C 11 D0
    ///
    /// Excluding the second response from a second ECU (41 0C 11 D0)
    /// and excluding the request sent from us.
    ///
    pub(crate) payload: Option<String>,

    /// Original service num in request
    pub(crate) service: [u8; 2],

    /// A list of ECU names that replied with this response.
    pub(crate) responding_ecus: Vec<String>,

    /// How many 'components' in the response.
    /// Excluding request characters and duplicate ECU responses.
    ///
    /// Take the formatted response for PID 010C
    /// (e.g: "41 0C 11 D0")
    ///
    /// Where:
    ///     41 0C - Positive response to PID 010C
    ///
    /// This above example would have a payload size of
    /// 2 bytes, 'a' and 'b'. (11 D0)
    pub(crate) payload_size: usize,
}

impl Response {
    pub fn new(raw: String, escaped: String) -> Self {
        Self {
            formatted_response: Some(raw),
            raw_response: Some(escaped),
            payload: None,
            service: [0u8; 2],
            payload_size: 0,
            responding_ecus: Vec::new(),
        }
    }

    pub fn no_data() -> Self {
        Self {
            formatted_response: Some("NO DATA".to_string()),
            raw_response: Some("NO DATA".to_string()),
            ..Default::default()
        }
    }

    pub fn map_no_data<F>(self, op: F) -> Scalar
    where
        F: FnOnce(Self) -> Scalar,
    {
        if self.payload_size == 0 {
            return Scalar::no_data();
        }

        op(self)
    }

    pub fn full_response(&self) -> Option<String> {
        self.formatted_response.clone()
    }

    pub fn get_payload_size(&self) -> &usize {
        &self.payload_size
    }

    pub fn get_payload(&self) -> Option<String> {
        if self.payload.is_none() && self.formatted_response.is_some() {
            // self.payload likely has not been updated. Update it now.
            return Some(self.payload_from_response());
        }

        self.payload.clone()
    }

    pub fn get_payload_components(&self) -> Vec<Vec<u8>> {
        let clean = match self.get_payload() {
            Some(resp) => {
                // Check what type of response this is based on the prefix
                //
                // In a mode 22 response
                // (e.g, 62 F4 0D 2B)
                //       0123456789A
                //                ^ start here
                // 62 F4 0D is the reply, 2B is the payload byte
                // which starts 9 characters into the string.
                //
                // In a service 01 response
                // (e.g 41 0C 1A F8)
                //      0123456789A
                //            ^ start here
                // 41 0C is the reply, 1A F8 are the payload bytes
                // which starts 6 characters into the string.
                //
                // Due to the difference in structures, to get the payload bytes
                // for mode 22 responses, we have to start 8 characters in the string
                // instead of 6 for a 01 service response
                if resp.starts_with("62") {
                    // Mode 22 response
                    if resp.len() < 9 {
                        println!("invalid response payload: '{resp}'");
                        return Vec::new();
                    }
                    resp[9..].to_string()
                } else if resp.starts_with("41") {
                    // Service 01 response
                    if resp.len() < 6 {
                        println!("invalid response payload: '{resp}'");
                        return Vec::new();
                    }

                    resp[6..].to_string()
                } else {
                    return Vec::new();
                }
            }
            None => return Vec::new(),
        };

        // dbg: println!("Payload: '{clean}'");

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

    pub fn e_value(&self) -> f32 {
        self.get_component(PayloadComponent::E)
    }

    fn get_component(&self, value: PayloadComponent) -> f32 {
        let components = self.get_payload_components();
        let bytes = match components.get(value.as_usize()) {
            Some(b) => b,
            None => {
                return 0.0;
            }
        };

        let utf8 = std::str::from_utf8(bytes).unwrap_or_else(|err| {
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
        let response = match &self.formatted_response {
            Some(resp) => resp.to_owned(),
            None => return String::default(),
        };

        let stripped = &response.replace(" ", "");

        let chunks = stripped.as_bytes().chunks(2).peekable();
        let mut first_response_found = false;
        let mut payload = String::new();
        let mut pairs = 0;

        for pair in chunks {
            if pair.len() != 2 {
                continue;
            }

            if (pair[0] == b'4' || pair[0] == b'6') && pair[1] == self.service[1] {
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

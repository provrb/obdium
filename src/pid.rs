#[derive(Debug)]
pub struct Response {
    response: Option<String>, // Hex Response from ECU
    bytes: u8,                // How many bytes in the response
}

impl Response {
    pub fn new(res: String) -> Self {
        Self {
            response: Some(res),
            bytes: 0,
        }
    }

    // TODO:
    fn a_value() -> f32 {
        0f32
    }

    fn b_value() -> f32 {
        0f32
    }

    fn c_value() -> f32 {
        0f32
    }

    fn d_value() -> f32 {
        0f32
    }
}

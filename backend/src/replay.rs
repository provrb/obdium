use std::fs;

use crate::{
    cmd::{Command, CommandType},
    obd::OBD,
    response::Response,
    RECORDED_REQEUSTS_DIR,
};
use serde_json::{json, Value};

impl OBD {
    pub fn record_requests(&mut self, state: bool) {
        // clear requests file
        if fs::File::create(RECORDED_REQEUSTS_DIR).is_ok() && state {
            self.record_requests = state;
        } else {
            println!("file creation to save requests failed. not recording requests.");
            self.record_requests = false;
        }
    }

    pub fn replay_requests(&mut self, state: bool) {
        // stop recording when replaying
        if state {
            self.reset_played_flags();
            self.record_requests = false;
        }

        self.replay_requests = state
    }

    pub(crate) fn save_request(&mut self, request: &Command, response: &Response) {
        // get the current file contents
        let mut contents_json = {
            let contents = fs::read_to_string(RECORDED_REQEUSTS_DIR).expect("file read error");
            if contents.trim().is_empty() {
                vec![]
            } else {
                serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
            }
        };

        let obj = json!({
            "request": request.as_string(),
            "request_type": request.command_type(),
            "response": response.raw_response.as_deref().unwrap_or(""),
            "played": false
        });

        contents_json.push(obj);

        let pretty =
            serde_json::to_string_pretty(&contents_json).expect("failed pretty'ing string");
        fs::write(RECORDED_REQEUSTS_DIR, pretty).expect("failed to write requests file");
    }

    pub(crate) fn get_recorded_response(&self, request: &Command) -> Response {
        let mut contents_json: Vec<Value> = {
            let contents = fs::read_to_string(RECORDED_REQEUSTS_DIR).expect("file read error");
            if contents.trim().is_empty() {
                vec![]
            } else {
                serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
            }
        };

        if contents_json.is_empty() {
            return Response::no_data();
        }

        let mut found_index = None;
        for (i, value) in contents_json.iter_mut().enumerate() {
            if value["request"] == request.as_string() && value["played"] == false {
                value["played"] = serde_json::Value::Bool(true);
                found_index = Some(i);
                break;
            }
        }

        if let Some(i) = found_index {
            let pretty =
                serde_json::to_string_pretty(&contents_json).expect("failed pretty'ing string");

            fs::write(RECORDED_REQEUSTS_DIR, pretty).expect("failed to write requests file");

            let value = &contents_json[i];
            let escaped_response = value["response"].as_str().unwrap_or_default();
            if value["request_type"] == json!(CommandType::PIDCommand) {
                return self
                    .parse_pid_response(escaped_response)
                    .unwrap_or_default();
            }

            return Response::new(escaped_response.to_string(), escaped_response.to_string());
        }

        Response::no_data()
    }

    fn reset_played_flags(&self) {
        let mut contents_json: Vec<Value> = {
            let contents = fs::read_to_string(RECORDED_REQEUSTS_DIR).expect("file read error");
            serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
        };

        if contents_json.is_empty() {
            return;
        }

        for value in contents_json.iter_mut() {
            if value["played"] == true {
                value["played"] = serde_json::Value::Bool(false);
            }
        }

        let pretty =
            serde_json::to_string_pretty(&contents_json).expect("failed pretty'ing string");

        fs::write(RECORDED_REQEUSTS_DIR, pretty).expect("failed to write requests file");
    }
}

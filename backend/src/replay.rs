use std::{fs, time::Duration};

use crate::{
    cmd::{Command, CommandType},
    obd::OBD,
    response::Response,
};
use rand::seq::IndexedRandom;
use serde_json::{json, Value};

impl OBD {
    pub fn record_requests(&mut self, state: bool, path: String) {
        if state {
            self.requests_path = path;

            match fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.requests_path)
            {
                Ok(_) => {
                    self.record_requests = true;
                }
                Err(e) => {
                    println!("File creation to save requests failed: {e}. Not recording requests.");
                    self.record_requests = false;
                }
            }
        } else {
            self.record_requests = false;
        }

        println!("Recording requests: {state:?}");
    }

    pub fn replay_requests(&mut self, state: bool) {
        // stop recording when replaying
        if state {
            self.reset_played_flags();
            self.record_requests = false;
        }

        self.replay_requests = state;
        println!("Replaying requests: {state:?}");
    }

    pub(crate) fn save_request(&mut self, request: &Command, response: &Response) {
        // get the current file contents
        let mut contents_json = {
            let contents = fs::read_to_string(&self.requests_path).expect("file read error");
            if contents.trim().is_empty() {
                vec![]
            } else {
                serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
            }
        };

        let obj = json!({
            "request": request.as_string(),
            "request_type": request.command_type(),
            "response": response.raw_response.as_deref().unwrap_or("")
        });

        contents_json.push(obj);

        let pretty =
            serde_json::to_string_pretty(&contents_json).expect("failed pretty'ing string");
        fs::write(&self.requests_path, pretty).expect("failed to write requests file");
    }

    pub(crate) fn get_recorded_response(&self, request: &Command) -> Response {
        // add a delay to simulate vehicle
        std::thread::sleep(Duration::from_millis(300));

        let contents_json: Vec<Value> = {
            let contents = fs::read_to_string(&self.requests_path).expect("file read error");
            if contents.trim().is_empty() {
                vec![]
            } else {
                serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
            }
        };

        if contents_json.is_empty() {
            return Response::no_data();
        }

        let mut related_requests = Vec::new();
        for value in contents_json.iter() {
            if value["request"] == request.as_string() {
                related_requests.push(value);
                break;
            }
        }

        if related_requests.is_empty() {
            return Response::no_data();
        }

        // Randomly select response to use from related_requests
        if let Some(value) = related_requests.choose(&mut rand::rng()) {
            let escaped_response = value["response"].as_str().unwrap_or_default();
            if value["request_type"] == json!(CommandType::PIDCommand) {
                return self
                    .parse_pid_response(escaped_response)
                    .unwrap_or(Response::no_data());
            }
            return Response::new(escaped_response.to_string(), escaped_response.to_string());
        }
        Response::no_data()
    }

    fn reset_played_flags(&self) {
        let mut contents_json: Vec<Value> = {
            let contents = fs::read_to_string(&self.requests_path).expect("file read error");
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

        fs::write(&self.requests_path, pretty).expect("failed to write requests file");
    }
}

use tide::Response;

use super::structs::*;

pub struct Failed {}

impl Failed {
    pub fn get() -> Response {
        let status = 400;
        let message = "Missing ID".to_string();

        let response = Response::builder(status).body(message).build();

        response
    }
}

pub struct Success {}

impl Success {
    pub fn get(user: User) -> Response {
        let status = 200;
        let body_parsed = serde_json::to_string(&user).expect("Failed to serialize to JSON");

        let response = Response::builder(status).body(body_parsed).build();

        response
    }
}

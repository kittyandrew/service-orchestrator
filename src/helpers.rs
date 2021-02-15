use crate::storage::{ServiceStorage, StoredService};
use rocket_contrib::json::JsonValue;
use crate::auth::{SName, SToken};
use serde_json::Value;
use reqwest::Client;
use tokio::spawn;


pub fn verify(storage: &ServiceStorage, name: &SName, token: &SToken) -> Option<JsonValue> {
    match storage.lock().unwrap().get(&name.0) {
        Some(creds) => match creds {
            creds if creds.token.0 == token.0 => None,
            _ => Some(json!({
                "msg_code": "err_token_invalid",
                "message": "Service token is invalid!",
            })),
        },
        None => Some(json!({
            "msg_code": "err_token_invalid",
            "message": "Service token is invalid!",
        })),
    }
}


pub fn propagate(/*rt: &Runtime,*/ c: Client, body: Value, auth: &StoredService) {
    let lauth: StoredService = auth.clone();
    spawn(async move {
        c.post(&lauth.url.0)
            .json(&body)
            .header("X-TOKEN", &lauth.token.0)
            .send()
            .await
            .expect("[propagate] Fatal error sending request to target ...");
    });
}


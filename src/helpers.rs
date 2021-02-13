use crate::storage::{ServiceStorage, StoredService};
use rocket_contrib::json::JsonValue;
use jsonschema::{Draft, JSONSchema};
use serde_json::{Value, from_str};
use crate::auth::{SName, SToken};
use tokio::runtime::Runtime;
use reqwest::Client;
use std::fs;


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


pub fn propagate(rt: &Runtime, c: Client, body: Value, auth: &StoredService) {
    let lauth: StoredService = auth.clone();
    rt.spawn(async move {
        c.post(&lauth.url.0)
            .json(&body)
            .header("X-TOKEN", &lauth.token.0)
            .send()
            .await
            .expect("[propagate] Fatal error sending request to target ...");
    });
}


pub fn read_schemas<'a>(path: &str) -> JSONSchema<'a> {
    // TODO: read all schemas from the folder
    let file = fs::read_to_string(path).expect("Unable to read file");
    let schema: Value = from_str(&file).expect("Unable to parse schema from string.");
    // @GITHUB_ISSUE: https://github.com/Stranger6667/jsonschema-rs/issues/145
    let schema_boxed: &'static Value = Box::leak(Box::new(schema));
    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_boxed)
        .expect("Failed to compile jsonschema.")
}


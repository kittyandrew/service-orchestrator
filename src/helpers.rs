use crate::storage::{AuthStorage, StoredAuth};
use rocket_contrib::json::JsonValue;
use tokio::runtime::Runtime;
use crate::auth::Auth;
use serde_json::Value;
use reqwest::Client;


pub fn verify(storage: &AuthStorage, auth: &Auth) -> Option<JsonValue> {
    match storage.lock().unwrap().get(&auth.service) {
        Some(creds) => match creds {
            creds if creds.token == auth.token => None,
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


pub fn propagate(rt: &Runtime, c: Client, body: Value, auth: &StoredAuth) {
    let lauth: StoredAuth = auth.clone();
    rt.spawn(async move {
        c.post(&lauth.url.0)
            .json(&body)
            .header("X-TOKEN", &lauth.token)
            .send()
            .await
            .expect("[propagate] Fatal error sending request to target ...");
        println!("Now running on a worker thread");
    });
}


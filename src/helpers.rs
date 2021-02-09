use rocket_contrib::json::JsonValue;
use crate::storage::AuthStorage;
use crate::auth::Auth;
use serde_json::Value;


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


pub fn propagate(body: &Value, token: &str) {
    // TODO: finish
    println!("BODY: {}\nTOKEN: {}", body, token);
}


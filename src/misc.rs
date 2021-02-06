use crate::auth::{Auth, ServiceUrl, TargetService};
use rocket_contrib::json::JsonValue;

// Root

#[get("/")]
pub fn get_index(auth: Auth, url: ServiceUrl, target: TargetService) -> JsonValue {
    json!({
        "msg_code": "info_root_msg",
        "message": "Hello from Orchestrator v0.0.1!",
        "test_token": auth.token,
        "test_service": auth.service,
        "test_url": url,
        "test_target": target,
    })
}

// Catch Errors

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "msg_code": "err_resource_not_found",
        "message": "Please make sure you entered the correct url!",
    })
}

// #TODO: explain invalid token/invalid data
#[catch(401)]
pub fn unauth_handler() -> JsonValue {
    json!({
        "msg_code": "err_unauthorized",
        "message": "Your are missing required headers or they are not correct!",
    })
}


// Third Party
use rocket_contrib::json::JsonValue;
use crate::auth::Auth;

// Root

#[get("/")]
pub fn get_index(auth: Auth) -> JsonValue {
    json!({
        "msg_code": "info_root_msg",
        "message": "Hello from HumanBios v0.0.1!",
        "test_token": auth.token,
        "test_service": auth.service,
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
#[catch(400)]
pub fn bad_request() -> JsonValue {
    json!({
        "msg_code": "err_bad_request",
        "message": "Server couldn't parse the data!",
    })
}


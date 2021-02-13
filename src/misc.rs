use rocket_contrib::json::JsonValue;
use crate::auth::{SToken, OToken};
use rocket::State;


// ROOT
#[get("/")]
pub fn get_index(stoken: SToken, otoken: State<OToken>) -> JsonValue {
    if otoken.0 != stoken.0 {
        return json!({
            "msg_code": "err_token_invalid",
            "message": "Orchestrator token is invalid!",
            "token": &stoken.0,
        })
    }

    json!({
        "msg_code": "info_root_msg",
        "message": "Hello from Orchestrator v0.0.9!",
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


// #TODO: return an actual error messages ROCKET. HOW. EXPLAIN.
#[catch(500)]
pub fn serverside_handler() -> JsonValue {
    json!({
        "msg_code": "err_serverside",
        "message": "bad.",
    })
}


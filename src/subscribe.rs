use crate::auth::{Auth, ServiceUrl, TargetService, OToken};
use crate::storage::{AuthStorage, StoredAuth};
use rand::distributions::Alphanumeric;
use rocket_contrib::json::JsonValue;
use rocket::State;
use std::iter;
use rand::Rng;


#[get("/new")]
pub fn subscriptions_new(map: State<AuthStorage>, otoken: State<OToken>, auth: Auth, url: ServiceUrl) -> JsonValue {
    if otoken.0 != auth.token {
        return json!({
            "msg_code": "err_token_invalid",
            "message": "Orchestrator token is invalid!",
            "token": &auth.token,
        })
    }

    let mut storage = map.lock().unwrap();
    let mut rng = rand::thread_rng();
    let new_token: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(32)
        .collect::<String>();

    // Rewrites if already contained any information
    storage.insert(
        auth.service.clone(),
        StoredAuth {
            service: auth.service,
            url: url,
            token: new_token.clone(),
        }
    );
    
    json!({
        "msg_code": "info_subscription_ok",
        "message": "Successfully subscribed to the orchestrator!",
        "new_token": &new_token,
    })
}


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


#[post("/forward", format = "application/json")]
pub fn subscriptions_forward(
    map: State<AuthStorage>,
    /*otoken: State<OToken>,*/
    auth: Auth,
    target: TargetService
) -> JsonValue {
    /* TODO: do we really want to do 2 factor auth (two tokens)?
    if otoken.0 != auth.token {
        return Err(json!({
            "msg_code": "err_token_invalid",
            "message": "Orchestrator token is invalid!",
            "token": &auth.token,
        }))
    }
    */

    // Check if service matches registered token
    if let Some(err) = verify(&map, &auth) {
        return err
    }

    // Get url for the requested service and forward request
    match map.lock().unwrap().get(&target.0) {
        Some(_) =>json!({
            "test": "Success!",
            // TODO: think about resending headers
        }),
        None => json!({
            "msg_code": "err_service_invalid",
            "message": "Requested service name is invalid!",
        }),
    }
}


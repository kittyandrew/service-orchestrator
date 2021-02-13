use crate::auth::{Auth, ServiceUrl, TargetService, OToken};
use crate::storage::{AuthStorage, StoredAuth};
use crate::helpers::{verify, propagate};
use rand::distributions::Alphanumeric;
use rocket_contrib::json::JsonValue;
use tokio::runtime::Runtime;
use jsonschema::JSONSchema;
use crate::body::ReqData;
use reqwest::Client;
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


#[post("/forward", format = "application/json", data = "<data>")]
pub fn subscriptions_forward(
    data: ReqData,
    schema: State<JSONSchema>,
    map: State<AuthStorage>,
    auth: Auth,
    target: TargetService,
    runtime: State<Runtime>,
    client: State<Client>,
) -> JsonValue {
    // Check if service matches registered token
    if let Some(err) = verify(&map, &auth) {
        return err
    }

    // Get url for the requested service and forward request
    match map.lock().unwrap().get(&target.0) {
        Some(creds) => {
            if schema.is_valid(&data.0) {
                // Propagating request here
                propagate(&runtime, client.inner().clone(), data.0.clone(), &creds);
                // Writing 
                return json!({
                    "msg_code": "info_propagation_ok",
                    "message": "Your request was successfully propagated to the desired service!",
                })
            }
            // If we got here, we don't need speed anymore -> just return useful error.
            let mut errors = schema.validate(&data.0).unwrap_err();
            return json!({
                "msg_code": "err_schema_invalid",
                "message": format!("Validation error: {}", errors.next().unwrap()),
            })
        },
        None => json!({
            "msg_code": "err_service_invalid",
            "message": "Requested service name is invalid!",
        }),
    }
}


use crate::auth::{SName, SToken, SUrl, STarget, OToken, SchemaName};
use crate::storage::{ServiceStorage, StoredService, SchemasStorage};
use crate::helpers::{verify, propagate};
use rand::distributions::Alphanumeric;
use rocket_contrib::json::JsonValue;
use crate::body::ReqData;
use reqwest::Client;
use rocket::State;
use std::iter;
use rand::Rng;


#[get("/new")]
pub fn subscriptions_new(
    services: State<ServiceStorage>,
    schemas: State<SchemasStorage>,
    otoken: State<OToken>,
    sname: SName,
    stoken: SToken,
    surl: SUrl,
    schema: SchemaName,
) -> JsonValue {
    if otoken.0 != stoken.0 {
        return json!({
            "msg_code": "err_token_invalid",
            "message": "Orchestrator token is invalid!",
            "token": &stoken,
        })
    }
    // Check if registrant provided valid schema name
    if !schemas.lock().unwrap().contains_key(&schema.0) {
        return json!({
            "msg_code": "err_schema_name_invalid",
            "message": "Could not map provided schema name to existing schema!",
            "name": &schema,
        })
    }

    let mut storage = services.lock().unwrap();
    let mut rng = rand::thread_rng();
    let new_token: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(32)
        .collect::<String>();

    // Rewrites if already contained any information
    storage.insert(
        sname.0.clone(),
        StoredService {
            name: sname,
            url: surl,
            token: SToken(new_token.clone()),
            schema: schema,
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
    client: State<Client>,
    services: State<ServiceStorage>,
    schemas: State<SchemasStorage>,
    data: ReqData,
    sname: SName,
    stoken: SToken,
    target: STarget,
) -> JsonValue {
    // Check if service matches registered token
    if let Some(err) = verify(&services, &sname, &stoken) {
        return err
    }
    // Get info about the requested service and forward request
    match services.lock().unwrap().get(&target.0) {
        Some(creds) => {
            // Check if sender provided valid schema
            match schemas.lock().unwrap().get(&creds.schema.0) {
                Some(schema) => {
                    if schema.is_valid(&data.0) {
                        // Propagating request here
                        propagate(client.inner().clone(), data.0.clone(), &creds);
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
                None => return json!({
                    "msg_code": "err_schema_name_invalid",
                    "message": "Could not map provided schema name to existing schema!",
                    "name": &creds.schema,
                }),
            }
        },
        None => json!({
            "msg_code": "err_service_invalid",
            "message": "Requested service name is invalid!",
        }),
    }
}


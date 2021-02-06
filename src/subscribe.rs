use crate::auth::{Auth, ServiceUrl, TargetService};
use crate::storage::{AuthStorage, StoredAuth};
use rocket_contrib::json::JsonValue;
use rand::distributions::Alphanumeric;
use rocket::State;
use std::iter;
use rand::Rng;


#[get("/new")]
pub fn subscriptions_new(map: State<AuthStorage>, auth: Auth, url: ServiceUrl) -> JsonValue {
    // TODO verify own token here (from `auth.token`)
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
            service: auth.service.clone(),
            url: url,
            token: new_token.clone(),
        }
    );
    
    json!({
        "msg_code": "info_subscription_ok",
        "message": "Successfully subscribed to the orchestrator!",
        "new_token": new_token,
    })
}

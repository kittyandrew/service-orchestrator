use std::collections::HashMap;
use crate::auth::ServiceUrl;
use std::sync::Mutex;


pub struct StoredAuth {
    pub url: ServiceUrl,
    pub service: String,
    pub token: String,
}


pub type AuthStorage = Mutex<HashMap<String, StoredAuth>>;


pub fn init() -> AuthStorage {
    Mutex::new(HashMap::<String, StoredAuth>::new())
}

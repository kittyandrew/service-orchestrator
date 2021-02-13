use crate::auth::{SName, SToken, SUrl};
use std::collections::HashMap;
use std::sync::Mutex;


#[derive(Clone, Debug)]
pub struct StoredService {
    pub url: SUrl,
    pub name: SName,
    pub token: SToken,
}


pub type ServiceStorage = Mutex<HashMap<String, StoredService>>;


pub fn init() -> ServiceStorage {
    Mutex::new(HashMap::<String, StoredService>::new())
}


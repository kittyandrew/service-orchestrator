use crate::auth::{SName, SToken, SUrl, SchemaName};
use jsonschema::{Draft, JSONSchema};
use serde_json::{Value, from_str};
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs;


#[derive(Clone, Debug)]
pub struct StoredService {
    pub url: SUrl,
    pub name: SName,
    pub token: SToken,
    pub schema: SchemaName,
}


pub type ServiceStorage = Mutex<HashMap<String, StoredService>>;

pub type SchemasStorage<'a> = Mutex<HashMap<String, JSONSchema<'a>>>;


pub fn init_services() -> ServiceStorage {
    Mutex::new(HashMap::<String, StoredService>::new())
}

pub fn init_schemas(dir_path: &str) -> SchemasStorage {
    let mut schemas = HashMap::<String, JSONSchema>::new();
    for entry in fs::read_dir(dir_path).expect("Problem with a schema dir path!") {
        let entry = entry.expect("Problem with reading schema dir!");
        let path = &entry.path();
        let file = fs::read_to_string(path).expect("Unable to read file");
        let schema: Value = from_str(&file).expect("Unable to parse schema from string.");
        // @GITHUB_ISSUE: https://github.com/Stranger6667/jsonschema-rs/issues/145
        let schema_boxed: &'static Value = Box::leak(Box::new(schema));
        let name = path.with_extension("").file_name().unwrap().to_os_string().into_string().expect("Bad OS String");
        schemas.insert(
            name,
            JSONSchema::options()
                .with_draft(Draft::Draft7)
                .compile(&schema_boxed)
                .expect("Failed to compile jsonschema.")
        );
    }
    Mutex::new(schemas)
}

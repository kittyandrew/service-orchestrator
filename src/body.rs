use rocket::data::{Outcome, FromData, ByteUnit};
use rocket::http::{Status, ContentType};
use serde::{Deserialize, Serialize};
use jsonschema::{Draft, JSONSchema};
use serde_json::{Value, from_str};
use rocket::{Request, Data};
use std::fs;


// Always use a limit to prevent DoS attacks.
const LIMIT: ByteUnit = ByteUnit::Byte(4096);


#[derive(Serialize, Deserialize)]
pub struct ReqData(pub Value);


#[rocket::async_trait]
impl FromData for ReqData {
    type Error = String;

    async fn from_data(req: &Request<'_>, data: Data) -> Outcome<Self, String> {
        // Ensure the content type is correct before opening the data.
        let json_ct = ContentType::new("application", "json");
        if req.content_type() != Some(&json_ct) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        // TODO: add optional header to set custom limit
        // let limit = req.limits().get("").unwrap_or(LIMIT);
        let json = match data.open(LIMIT).stream_to_string().await {
            Ok(string) => match from_str(&string) {
                Ok(valid_json) => valid_json,
                Err(e) => return Outcome::Failure((
                    Status::InternalServerError, format!("Failed to parse json: {}", e)
                )),
            },
            Err(e) => return Outcome::Failure((
                Status::InternalServerError, format!("Failed to read body: {}", e)
            ))
        };
    
        // Return successfully.
        Outcome::Success(ReqData(json))
    }
}


pub fn read_schemas<'a>(path: &str) -> JSONSchema<'a> {
    // TODO: read all schemas from the folder
    let file = fs::read_to_string(path).expect("Unable to read file");
    let schema: Value = from_str(&file).expect("Unable to parse schema from string.");
    // @RUSTWTF: https://github.com/Stranger6667/jsonschema-rs/issues/145
    let schema_boxed: &'static Value = Box::leak(Box::new(schema));
    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_boxed)
        .expect("Failed to compile jsonschema.")
}


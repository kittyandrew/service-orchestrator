use rocket::data::{Outcome, FromData, ByteUnit};
use rocket::http::{Status, ContentType};
// use rocket::tokio::io::AsyncReadExt;
use serde::{Deserialize, Serialize};
use rocket::{Request, Data};
// use std::io::Read;


// Always use a limit to prevent DoS attacks.
// TODO: Document and (maybe) allow to be tweakable
const LIMIT: ByteUnit = ByteUnit::Byte(256);


#[derive(Serialize, Deserialize)]
pub struct ReqData(pub String);


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
            Ok(string) => string,
            Err(e) => return Outcome::Failure((Status::InternalServerError, format!("{}", e)))
        };
    
        // Return successfully.
        Outcome::Success(ReqData(json))
    }
}


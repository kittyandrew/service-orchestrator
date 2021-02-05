use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};


pub struct Auth {
    pub token: String,
    pub service: String,
}


#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    MissingService,
}


#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = AuthError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token: &str;
        let service: &str;

        match req.headers().get_one("X-TOKEN") {
            Some(v) => token = v,
            // Early return on error
            None => return Outcome::Failure((Status::BadRequest, AuthError::MissingToken)),
        }

        match req.headers().get_one("X-SERVICE") {
            Some(v) => service = v,
            // Early return on error
            None => return Outcome::Failure((Status::BadRequest, AuthError::MissingService)),
        }

        // Returning parsed auth
        Outcome::Success(Auth {
            token: token.to_string(),
            service: service.to_string(),
        })
    }
}


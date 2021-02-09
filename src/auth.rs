use rocket::request::{Outcome, Request, FromRequest};
use url::{Url, ParseError};
use rocket::http::Status;
use serde::{Serialize};


pub struct Auth {
    pub token: String,
    pub service: String,
}

#[derive(Debug, Serialize)]
pub struct ServiceUrl(pub String);

pub struct OToken(pub String);

#[derive(Debug, Serialize)]
pub struct TargetService(pub String);


#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    MissingService,
}

#[derive(Debug)]
pub enum UrlError {
    MissingUrl,
    Parse(ParseError),
}

#[derive(Debug)]
pub enum TargetServiceError {
    MissingTargetService,
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
            None => return Outcome::Failure((Status::Unauthorized, AuthError::MissingToken)),
        }

        match req.headers().get_one("X-SERVICE") {
            Some(v) => service = v,
            // Early return on error
            None => return Outcome::Failure((Status::Unauthorized, AuthError::MissingService)),
        }

        // Returning parsed auth
        Outcome::Success(Auth {
            token: token.to_string(),
            service: service.to_string(),
        })
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ServiceUrl {
    type Error = UrlError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-URL") {
            Some(v) => match Url::parse(v) {
                Ok(url) => Outcome::Success(ServiceUrl(url.to_string())),
                Err(e)  => Outcome::Failure((Status::Unauthorized, UrlError::Parse(e)))
            },
            None => Outcome::Failure((Status::Unauthorized, UrlError::MissingUrl)),
        }
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for TargetService {
    type Error = TargetServiceError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-TARGET-SERVICE") {
            Some(v) => Outcome::Success(TargetService(v.to_string())),
            None => Outcome::Failure((Status::Unauthorized, TargetServiceError::MissingTargetService)),
        }
    }
}


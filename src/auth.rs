use rocket::request::{Outcome, Request, FromRequest};
use url::{Url, ParseError};
use rocket::http::Status;
use serde::{Serialize};


#[derive(Debug, Serialize, Clone)]
pub struct SToken(pub String);

#[derive(Debug, Serialize, Clone)]
pub struct SName(pub String);

#[derive(Debug, Serialize, Clone)]
pub struct SUrl(pub String);

#[derive(Debug, Serialize)]
pub struct OToken(pub String);

#[derive(Debug, Serialize)]
pub struct STarget(pub String);


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
impl<'a, 'r> FromRequest<'a, 'r> for SToken {
    type Error = AuthError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-TOKEN") {
            Some(v) => Outcome::Success(SToken(v.to_string())),
            None => Outcome::Failure((Status::Unauthorized, AuthError::MissingToken)),
        }
    }
}


#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for SName {
    type Error = AuthError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-SERVICE") {
            Some(v) => Outcome::Success(SName(v.to_string())),
            None => Outcome::Failure((Status::Unauthorized, AuthError::MissingService)),
        }
    }
}


#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for SUrl {
    type Error = UrlError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-URL") {
            Some(v) => match Url::parse(v) {
                Ok(url) => Outcome::Success(SUrl(url.to_string())),
                Err(e)  => Outcome::Failure((Status::Unauthorized, UrlError::Parse(e)))
            },
            None => Outcome::Failure((Status::Unauthorized, UrlError::MissingUrl)),
        }
    }
}


#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for STarget {
    type Error = TargetServiceError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-TARGET-SERVICE") {
            Some(v) => Outcome::Success(STarget(v.to_string())),
            None => Outcome::Failure((Status::Unauthorized, TargetServiceError::MissingTargetService)),
        }
    }
}


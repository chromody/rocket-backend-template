use rocket::serde::{Deserialize, Serialize};
use jsonwebtoken::errors::{Error, ErrorKind};
use mongodb::bson::{oid::ObjectId};

use chrono::Utc;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Algorithm, Header, Validation}; // 👈 New!
use std::env;
use dotenvy::dotenv;

use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

use crate::responses::network_responses::{NetworkResponse, ResponseBody, Response};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub_id: ObjectId,
    exp: usize,
    pub preferred_username: String
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}


pub fn create_jwt(id: ObjectId, username: String) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set."); // 👈 New!

    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60)).expect("Invalid timestamp").timestamp();
    
    let claims = Claims {
        sub_id: id,
        exp: expiration as usize,
        preferred_username: username
    }; 

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    // 👇 New!
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response { body: ResponseBody::Message(String::from("Error validating JWT token - No token provided"))};
                Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Expired Token"))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Invalid Token"))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    },
                    _ => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - {}", err))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    }
                }
            },
        }
    }
}